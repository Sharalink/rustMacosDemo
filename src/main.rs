mod keychain;

use keychain::KeychainManager;
use keychain::keychain_utils;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔐 macOS Keychain 访问示例应用");
    println!("================================");
    
    // 检查 Keychain 访问权限
    match keychain_utils::check_keychain_access() {
        Ok(true) => println!("✅ Keychain 访问权限验证成功"),
        Ok(false) => {
            println!("❌ Keychain 访问权限不足");
            println!("💡 请确保应用已正确签名并具有 Keychain 权限");
            return Ok(());
        }
        Err(e) => {
            println!("❌ Keychain 权限检查失败: {}", e);
            return Err(e);
        }
    }
    
    // 显示验证菜单
    show_verification_menu()?;
    
    println!("\n✅ 应用运行完成！");
    Ok(())
}

fn show_verification_menu() -> Result<(), Box<dyn Error>> {
    println!("\n📋 Keychain 验证菜单");
    println!("====================");
    println!("1️⃣  运行基本演示");
    println!("2️⃣  验证现有条目");
    println!("3️⃣  创建并验证测试条目");
    println!("4️⃣  系统诊断");
    println!("5️⃣  运行所有验证");
    println!("\n请选择一个选项 (1-5)，或按 Enter 运行所有验证:");
    
    use std::io::{self, Write};
    print!("> ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();
    
    match choice {
        "1" | "" => {
            let keychain = KeychainManager::new("com.yourcompany.shenliang");
            demo_keychain_operations(&keychain)?;
        }
        "2" => {
            keychain_utils::verify_service_entries("com.yourcompany.shenliang")?;
        }
        "3" => {
            keychain_utils::create_and_verify_test_entry()?;
        }
        "4" => {
            keychain_utils::diagnose_keychain_status()?;
        }
        "5" => {
            println!("🔄 运行完整验证套件...\n");
            
            // 1. 基本演示
            println!("=== 1. 基本演示 ===");
            let keychain = KeychainManager::new("com.yourcompany.shenliang");
            demo_keychain_operations(&keychain)?;
            
            // 2. 验证现有条目
            println!("\n=== 2. 验证现有条目 ===");
            keychain_utils::verify_service_entries("com.yourcompany.shenliang")?;
            
            // 3. 创建并验证测试条目
            println!("\n=== 3. 测试条目验证 ===");
            keychain_utils::create_and_verify_test_entry()?;
            
            // 4. 系统诊断
            println!("\n=== 4. 系统诊断 ===");
            keychain_utils::diagnose_keychain_status()?;
        }
        _ => {
            println!("❌ 无效选择，运行默认演示");
            let keychain = KeychainManager::new("com.yourcompany.shenliang");
            demo_keychain_operations(&keychain)?;
        }
    }
    
    Ok(())
}

fn demo_keychain_operations(keychain: &KeychainManager) -> Result<(), Box<dyn Error>> {
    println!("\n📝 演示 Keychain 操作:");
    println!("---------------------");
    
    let account = "demo-user";
    let original_password = "my-secret-password-123";
    
    // 1. 存储密码
    println!("\n1️⃣  存储密码到 Keychain...");
    keychain.store_password(account, original_password)?;
    
    // 2. 检查密码是否存在
    println!("\n2️⃣  检查密码是否存在...");
    if keychain.password_exists(account) {
        println!("✅ 密码存在于 Keychain 中");
    } else {
        println!("❌ 密码不存在于 Keychain 中");
    }
    
    // 3. 读取密码
    println!("\n3️⃣  从 Keychain 读取密码...");
    match keychain.get_password(account) {
        Ok(retrieved_password) => {
            println!("✅ 成功读取密码");
            if retrieved_password == original_password {
                println!("✅ 密码匹配正确");
            } else {
                println!("❌ 密码不匹配！");
            }
        }
        Err(e) => {
            println!("❌ 读取密码失败: {}", e);
        }
    }
    
    // 4. 更新密码
    println!("\n4️⃣  更新 Keychain 中的密码...");
    let new_password = keychain_utils::generate_password(16);
    println!("🔄 生成新密码: {}", new_password);
    keychain.store_password(account, &new_password)?; // 覆盖现有密码
    
    // 5. 验证更新
    println!("\n5️⃣  验证密码更新...");
    let updated_password = keychain.get_password(account)?;
    if updated_password == new_password {
        println!("✅ 密码更新成功");
    } else {
        println!("❌ 密码更新失败");
    }
    
    // 6. 清理 - 删除测试密码
    println!("\n6️⃣  清理测试数据...");
    keychain.delete_password(account)?;
    
    // 7. 验证删除
    println!("\n7️⃣  验证密码已删除...");
    if !keychain.password_exists(account) {
        println!("✅ 密码已成功从 Keychain 删除");
    } else {
        println!("❌ 密码删除失败");
    }
    
    println!("\n🎉 Keychain 操作演示完成！");
    Ok(())
}
