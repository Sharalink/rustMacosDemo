# 🚀 快速开始指南

## 第一次使用

### 1. 准备环境
```bash
# 1. 安装 Rust 和相关工具
./install_rust.sh

# 2. 重新加载环境变量
source ~/.cargo/env

# 3. 验证安装
cargo --version
```

### 2. 创建测试证书
```bash
# 创建用于开发测试的自签名证书
make test-cert
```

如果上述命令失败，请手动创建：
1. 打开 "钥匙串访问" (Keychain Access)
2. 钥匙串访问 → 证书助理 → 创建证书
3. 名称: `Test Certificate`
4. 证书类型: `代码签名`

### 3. 构建和签名
```bash
# 构建、签名并创建 DMG（一条命令完成所有操作）
make all

# 或者分步执行：
make build    # 构建应用
make sign     # 签名应用
make dmg      # 创建 DMG
```

### 4. 测试 Keychain 功能
```bash
# 测试 Keychain 访问权限
make keychain-test

# 查看 Keychain 相关信息
make keychain-info
```

## 🔐 Keychain 功能演示

运行应用后，您将看到：

1. ✅ **权限检查**: 验证 Keychain 访问权限
2. 📝 **存储密码**: 将密码安全存储到 Keychain
3. 🔍 **读取密码**: 从 Keychain 读取已存储的密码
4. 🔄 **更新密码**: 生成新密码并更新 Keychain
5. 🗑️ **删除密码**: 清理测试数据

## 🛠️ 常用命令

```bash
make help           # 显示所有命令帮助
make build          # 构建 Release 版本
make sign           # 签名应用
make verify         # 验证签名
make run            # 运行应用
make clean          # 清理构建文件
make certificates   # 列出可用证书
```

## ⚠️ 故障排除

### 问题：找不到 cargo 命令
```bash
# 解决方案：安装 Rust
./install_rust.sh
source ~/.cargo/env
```

### 问题：签名失败
```bash
# 解决方案：检查证书
make certificates

# 如果没有证书，创建测试证书
make test-cert
```

### 问题：Keychain 访问被拒绝
1. 确保应用已正确签名
2. 检查 entitlements.plist 配置
3. 验证 Bundle ID 匹配

## 📚 更多文档

- `README.md` - 完整使用指南
- `KEYCHAIN_GUIDE.md` - Keychain 权限详细说明
- `entitlements.plist` - 权限配置文件
