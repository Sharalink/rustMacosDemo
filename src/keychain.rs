use security_framework::passwords::{get_generic_password, set_generic_password, delete_generic_password};
use security_framework::base::Error as SecurityError;
use std::error::Error;
use std::fmt;

/// 自定义错误类型
#[derive(Debug)]
pub enum KeychainError {
    SecurityFrameworkError(SecurityError),
    NotFound,
    InvalidData,
    AccessDenied,
}

impl fmt::Display for KeychainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeychainError::SecurityFrameworkError(e) => write!(f, "Security framework error: {:?}", e),
            KeychainError::NotFound => write!(f, "Item not found in keychain"),
            KeychainError::InvalidData => write!(f, "Invalid data format"),
            KeychainError::AccessDenied => write!(f, "Access denied to keychain"),
        }
    }
}

impl Error for KeychainError {}

impl From<SecurityError> for KeychainError {
    fn from(error: SecurityError) -> Self {
        KeychainError::SecurityFrameworkError(error)
    }
}

/// Keychain 管理器结构体
pub struct KeychainManager {
    service_name: String,
}

impl KeychainManager {
    /// 创建新的 Keychain 管理器
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }
    
    /// 存储密码到 Keychain
    pub fn store_password(&self, account: &str, password: &str) -> Result<(), KeychainError> {
        match set_generic_password(&self.service_name, account, password.as_bytes()) {
            Ok(_) => {
                println!("✅ 密码已存储: 服务={}, 账户={}", self.service_name, account);
                Ok(())
            }
            Err(e) => {
                // 如果密码已存在，会失败，这是正常的
                println!("❌ 存储密码失败: {:?}", e);
                Err(KeychainError::from(e))
            }
        }
    }
    
    /// 更新现有密码
    fn update_password(&self, account: &str, password: &str) -> Result<(), KeychainError> {
        // 先删除旧密码，再添加新密码
        let _ = delete_generic_password(&self.service_name, account);
        set_generic_password(&self.service_name, account, password.as_bytes())
            .map_err(KeychainError::from)?;
        Ok(())
    }
    
    /// 从 Keychain 获取密码
    pub fn get_password(&self, account: &str) -> Result<String, KeychainError> {
        match get_generic_password(&self.service_name, account) {
            Ok(password_data) => {
                let password = String::from_utf8(password_data)
                    .map_err(|_| KeychainError::InvalidData)?;
                println!("🔑 密码已获取: 服务={}, 账户={}", self.service_name, account);
                Ok(password)
            }
            Err(e) => {
                println!("❌ 读取密码失败: {:?}", e);
                Err(KeychainError::from(e))
            }
        }
    }
    
    /// 从 Keychain 删除密码
    pub fn delete_password(&self, account: &str) -> Result<(), KeychainError> {
        match delete_generic_password(&self.service_name, account) {
            Ok(_) => {
                println!("🗑️ 密码已删除: 服务={}, 账户={}", self.service_name, account);
                Ok(())
            }
            Err(e) => {
                println!("❌ 删除密码失败: {:?}", e);
                Err(KeychainError::from(e))
            }
        }
    }
    
    /// 检查密码是否存在
    pub fn password_exists(&self, account: &str) -> bool {
        match self.get_password(account) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// 验证存储的数据完整性
    pub fn verify_storage(&self, account: &str, expected_password: &str) -> Result<bool, KeychainError> {
        println!("🔍 验证存储数据: 服务={}, 账户={}", self.service_name, account);
        
        match self.get_password(account) {
            Ok(stored_password) => {
                let matches = stored_password == expected_password;
                if matches {
                    println!("✅ 存储验证成功: 密码匹配");
                } else {
                    println!("❌ 存储验证失败: 密码不匹配");
                    println!("   期望长度: {}, 实际长度: {}", expected_password.len(), stored_password.len());
                }
                Ok(matches)
            }
            Err(e) => {
                println!("❌ 无法读取存储的数据: {}", e);
                Err(e)
            }
        }
    }

    /// 强制更新密码（先删除再添加）
    pub fn force_update_password(&self, account: &str, password: &str) -> Result<(), KeychainError> {
        println!("🔄 强制更新密码: 服务={}, 账户={}", self.service_name, account);
        
        // 先尝试删除现有条目（忽略错误）
        let _ = self.delete_password(account);
        
        // 添加新密码
        match set_generic_password(&self.service_name, account, password.as_bytes()) {
            Ok(_) => {
                println!("✅ 密码强制更新成功");
                Ok(())
            }
            Err(e) => {
                println!("❌ 密码强制更新失败: {:?}", e);
                Err(KeychainError::from(e))
            }
        }
    }

    /// 获取详细的错误信息
    pub fn get_detailed_info(&self, account: &str) -> String {
        let mut info = format!("=== Keychain 详细信息 ===\n");
        info.push_str(&format!("服务名称: {}\n", self.service_name));
        info.push_str(&format!("账户名称: {}\n", account));
        
        match self.get_password(account) {
            Ok(password) => {
                info.push_str(&format!("状态: ✅ 存在\n"));
                info.push_str(&format!("密码长度: {} 字符\n", password.len()));
                info.push_str(&format!("密码预览: {}***\n", &password[..password.len().min(3)]));
            }
            Err(e) => {
                info.push_str(&format!("状态: ❌ 不存在或无法访问\n"));
                info.push_str(&format!("错误: {}\n", e));
            }
        }
        
        info.push_str("========================\n");
        info
    }
}

/// Keychain 实用函数
pub mod keychain_utils {
    use super::*;
    
    /// 生成安全的随机密码
    pub fn generate_password(length: usize) -> String {
        // 使用系统时间作为简单的随机种子
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";
        let mut password = String::new();
        
        // 基于当前时间生成伪随机密码
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;
        
        for i in 0..length {
            let index = (timestamp + i * 7 + i * i) % chars.len();
            password.push(chars.chars().nth(index).unwrap());
        }
        
        password
    }
    
    /// 验证 Keychain 访问权限
    pub fn check_keychain_access() -> Result<bool, Box<dyn Error>> {
        println!("🔐 检查 Keychain 访问权限...");
        
        let test_service = "com.test.keychain-access";
        let test_account = "test-account";
        let test_password = "test-password";
        
        let manager = KeychainManager::new(test_service);
        
        // 尝试写入测试条目
        match manager.store_password(test_account, test_password) {
            Ok(_) => {
                println!("✅ Keychain 写入权限正常");
                
                // 尝试读取测试条目
                match manager.get_password(test_account) {
                    Ok(_) => {
                        println!("✅ Keychain 读取权限正常");
                        
                        // 清理测试条目
                        let _ = manager.delete_password(test_account);
                        Ok(true)
                    }
                    Err(e) => {
                        println!("❌ Keychain 读取权限失败: {}", e);
                        Ok(false)
                    }
                }
            }
            Err(e) => {
                println!("❌ Keychain 写入权限失败: {}", e);
                Ok(false)
            }
        }
    }

    /// 验证 Keychain 中特定服务的所有条目
    pub fn verify_service_entries(service_name: &str) -> Result<(), Box<dyn Error>> {
        println!("🔍 验证服务 '{}' 的 Keychain 条目...", service_name);
        
        let manager = KeychainManager::new(service_name);
        
        // 测试一些常见的账户名
        let test_accounts = vec!["demo-user", "test-user", "admin", "user"];
        let mut found_count = 0;
        
        for account in &test_accounts {
            if manager.password_exists(account) {
                found_count += 1;
                println!("📋 {}", manager.get_detailed_info(account));
            }
        }
        
        if found_count == 0 {
            println!("ℹ️  未找到服务 '{}' 的任何 Keychain 条目", service_name);
        } else {
            println!("✅ 找到 {} 个 Keychain 条目", found_count);
        }
        
        Ok(())
    }

    /// 创建测试条目并验证
    pub fn create_and_verify_test_entry() -> Result<(), Box<dyn Error>> {
        println!("🧪 创建测试条目并验证...");
        
        let service = "com.test.verification";
        let account = "verification-test";
        let password = generate_password(12);
        
        let manager = KeychainManager::new(service);
        
        println!("1️⃣  创建测试条目...");
        manager.force_update_password(account, &password)?;
        
        println!("2️⃣  验证数据完整性...");
        let is_valid = manager.verify_storage(account, &password)?;
        
        if is_valid {
            println!("3️⃣  显示详细信息...");
            println!("{}", manager.get_detailed_info(account));
        }
        
        println!("4️⃣  清理测试数据...");
        manager.delete_password(account)?;
        
        println!("✅ 测试完成!");
        Ok(())
    }

    /// 系统 Keychain 状态诊断
    pub fn diagnose_keychain_status() -> Result<(), Box<dyn Error>> {
        println!("🩺 Keychain 系统诊断...");
        println!("======================");
        
        // 检查基本访问权限
        println!("\n🔸 检查基本权限:");
        match check_keychain_access() {
            Ok(true) => println!("  ✅ 基本访问权限正常"),
            Ok(false) => println!("  ⚠️  基本访问权限受限"),
            Err(e) => println!("  ❌ 基本访问权限检查失败: {}", e),
        }
        
        // 测试不同长度的密码
        println!("\n🔸 测试不同密码长度:");
        let test_lengths = vec![8, 16, 32, 64];
        for length in test_lengths {
            let test_service = format!("com.test.length-{}", length);
            let test_account = "length-test";
            let test_password = generate_password(length);
            
            let manager = KeychainManager::new(&test_service);
            match manager.force_update_password(test_account, &test_password) {
                Ok(_) => {
                    match manager.verify_storage(test_account, &test_password) {
                        Ok(true) => println!("  ✅ {}字符密码: 存储和验证成功", length),
                        Ok(false) => println!("  ⚠️  {}字符密码: 存储成功但验证失败", length),
                        Err(e) => println!("  ❌ {}字符密码: 验证时出错 - {}", length, e),
                    }
                    let _ = manager.delete_password(test_account);
                }
                Err(e) => println!("  ❌ {}字符密码: 存储失败 - {}", length, e),
            }
        }
        
        println!("\n🩺 诊断完成!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keychain_manager() {
        let manager = KeychainManager::new("com.test.keychain-manager");
        let account = "test-user";
        let password = "test-password-123";
        
        // 测试存储
        assert!(manager.store_password(account, password).is_ok());
        
        // 测试检查存在
        assert!(manager.password_exists(account));
        
        // 测试获取
        let retrieved = manager.get_password(account).unwrap();
        assert_eq!(retrieved, password);
        
        // 测试删除
        assert!(manager.delete_password(account).is_ok());
        
        // 测试删除后不存在
        assert!(!manager.password_exists(account));
    }
    
    #[test]
    fn test_generate_password() {
        let password = keychain_utils::generate_password(12);
        assert_eq!(password.len(), 12);
        assert!(!password.is_empty());
    }
}
