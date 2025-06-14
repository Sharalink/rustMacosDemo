# 🔐 Rust macOS Keychain 测试工程

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![macOS](https://img.shields.io/badge/macOS-10.15+-blue.svg)](https://www.apple.com/macos)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

> **✨ 这是给慎亮的测试工程，用于演示和学习 macOS Keychain 开发！**  
> 这是一个完整的教学 Demo，展示了如何在 Rust 中安全地使用 macOS 系统功能。

这是一个完整的 Rust macOS 应用示例，展示了如何：
- 🔑 **安全地访问 macOS Keychain**
- 📝 **进行应用代码签名和公证**
- 📦 **创建分发就绪的 DMG 安装包**
- 🛡️ **正确配置 entitlements 权限**

## 🎯 项目说明

**📚 这是专门为慎亮准备的 macOS 开发学习项目！**

本项目是一个完整的教学示例，帮助理解：
- 🦀 **Rust 在 macOS 平台的开发实践**
- 🔐 **macOS Keychain 服务的正确使用**
- 📱 **macOS 应用的签名和分发流程**
- 🛡️ **应用安全和权限管理**

**🚀 适合学习场景：**
- macOS 原生开发入门
- Rust 系统编程实践
- 应用安全和权限配置
- 代码签名和应用分发

---

## ✨ 主要功能

- **🔐 完整的 Keychain 管理**: 存储、读取、更新、删除密码
- **🔒 权限验证系统**: 自动检查和验证 Keychain 访问权限
- **🛠️ 交互式演示**: 提供多种验证和测试模式
- **🔧 自动化构建**: 一键构建、签名、打包流程
- **📋 错误处理**: 详细的错误信息和故障排除指南
- **🎯 生产就绪**: 包含公证和分发配置

## 📋 前置要求

1. **macOS 开发环境**
   - Xcode Command Line Tools: `xcode-select --install`
   - Rust 工具链: 运行 `./install_rust.sh` 自动安装

2. **代码签名证书**
   - **生产环境**: Apple Developer Program 会员资格和 Developer ID 证书
   - **测试环境**: 可以创建自签名证书

## 🚀 快速开始

### 🛠️ 环境准备
```bash
# 1. 安装 Rust 和开发工具
./install_rust.sh

# 2. 查看所有可用命令
make help
```

### ⚡ 快速开始
```bash
# 构建应用
make build

# 测试 Keychain 功能
make test
```

### 🎯 分步操作
```bash
# 1. 构建应用
make build

# 2. 签名应用（可选，仅用于测试）
make sign

# 3. 运行应用
make run

# 4. 清理构建文件
make clean
```

## 🔐 Keychain 功能详解

### 核心功能
本应用提供了完整的 macOS Keychain 管理功能：

```rust
use crate::keychain::KeychainManager;

// 创建 Keychain 管理器
let keychain = KeychainManager::new("com.yourcompany.shenliang");

// 基本操作
keychain.store_password("user@example.com", "secret123")?;    // 存储密码
let password = keychain.get_password("user@example.com")?;    // 读取密码
keychain.password_exists("user@example.com");                // 检查存在性
keychain.delete_password("user@example.com")?;               // 删除密码
```

### 🎮 交互式演示模式
应用提供了多种验证模式：

1. **📝 基本演示** - 完整的 CRUD 操作演示
2. **🔍 验证现有条目** - 检查已存在的 Keychain 项目
3. **🧪 测试条目验证** - 创建临时测试数据进行验证
4. **🔧 系统诊断** - 全面的系统状态检查
5. **🚀 完整验证套件** - 运行所有测试

### 🛡️ 安全特性
- **权限自动检查**: 启动时自动验证 Keychain 访问权限
- **错误处理**: 详细的错误信息和恢复建议
- **安全密码生成**: 内置安全随机密码生成器
- **数据清理**: 自动清理测试数据，避免泄露

### 📋 可用命令
```bash
make build      # 构建 release 版本
make run        # 运行应用  
make test       # 测试 Keychain 访问
make sign       # 签名应用 (仅用于测试)
make clean      # 清理构建文件
make help       # 显示帮助信息
```

## � 项目结构

```
├── 📁 src/
│   ├── main.rs          # 主程序入口，交互式演示
│   └── keychain.rs      # Keychain 管理核心模块
├── 📄 Cargo.toml        # 项目配置和依赖
├── 📄 entitlements.plist # 应用权限配置
├── 🛠️ Makefile          # 自动化构建脚本
├── 📜 QUICKSTART.md     # 快速入门指南
└── 🔧 脚本文件/
    ├── install_rust.sh  # Rust 环境安装
    ├── sign_app.sh      # 应用签名脚本
    ├── notarize_app.sh  # 应用公证脚本
    └── create_dmg.sh    # DMG 创建脚本
```

## 🛠️ 技术栈

- **🦀 Rust 2021 Edition** - 主要编程语言
- **🔐 security-framework** - macOS 安全框架绑定
- **📊 serde/serde_json** - 数据序列化
- **🏗️ Make** - 构建自动化
- **📱 macOS APIs** - 原生系统集成
## 🔧 应用签名和分发

### 📋 前置要求

1. **💻 开发环境**
   - Xcode Command Line Tools: `xcode-select --install`
   - Rust 工具链 (自动安装): `./install_rust.sh`

2. **🔐 代码签名证书**
   - **🧪 开发测试**: 自签名证书 (`make test-cert`)
   - **🚀 生产分发**: Apple Developer Program 会员和 Developer ID 证书

### 🏗️ 构建流程

#### 开发模式
```bash
# 构建应用
make build

# 构建和签名（测试用）
make sign
```

#### 生产模式
```bash
# 使用生产证书签名（需要先配置证书）
make sign CERT_NAME="Developer ID Application: Your Name (TEAM_ID)"
```

### 🔑 权限配置 (entitlements.plist)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- Keychain 访问权限 -->
    <key>keychain-access-groups</key>
    <array>
        <string>$(AppIdentifierPrefix)com.yourcompany.shenliang</string>
    </array>
    
    <!-- 网络访问权限 -->
    <key>com.apple.security.network.client</key>
    <true/>
    
    <!-- 文件访问权限 -->
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
    
    <!-- 运行时强化 -->
    <key>com.apple.security.cs.allow-jit</key>
    <false/>
    <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
    <false/>
    <key>com.apple.security.cs.disable-executable-page-protection</key>
    <false/>
</dict>
</plist>
```

### 🔍 签名验证
```bash
# 如果已签名，可手动验证签名状态
codesign --verify --verbose target/release/shenliang
codesign --display --verbose=4 target/release/shenliang
```

## � 应用公证和分发

### 📤 公证配置
编辑 `notarize_app.sh` 配置你的 Apple ID 信息：

```bash
APPLE_ID="your-apple-id@example.com"
TEAM_ID="XXXXXXXXXX"
APP_PASSWORD="xxxx-xxxx-xxxx-xxxx"  # App-specific password
```

### 🔄 公证流程
```bash
# 注意：当前 Makefile 不包含公证功能
# 如需公证，需要手动配置 notarize_app.sh 脚本
./notarize_app.sh

# 检查公证状态
xcrun notarytool history --apple-id "your-apple-id@example.com" \
                        --password "your-app-password" \
                        --team-id "TEAM_ID"
```

### 📦 创建分发包
```bash
# 注意：当前 Makefile 不包含 DMG 创建功能
# 如需创建 DMG，需要手动运行脚本
./create_dmg.sh
```

## 🔧 Make 命令参考

| 命令 | 功能描述 |
|------|----------|
| `make help` | 📖 显示所有可用命令 |
| `make build` | 🔨 构建 release 版本 |
| `make run` | ▶️ 运行应用 |
| `make test` | 🔐 测试 Keychain 访问 |
| `make sign` | ✍️ 签名应用 (仅用于测试) |
| `make clean` | 🗑️ 清理构建文件 |

## � 故障排除

### 常见问题及解决方案

| 问题 | 症状 | 解决方案 |
|------|------|----------|
| **� 构建失败** | `cargo build` 报错 | 检查 Rust 工具链是否正确安装<br/>运行 `./install_rust.sh` 重新安装 |
| **🔐 签名失败** | `codesign` 报错 | 检查证书是否存在<br/>可以跳过签名步骤，直接运行 `make run` |
| **❌ Keychain 访问被拒** | 权限错误 | 运行时会提示授权<br/>在系统偏好设置中允许访问 |
| **🚫 应用无法运行** | 运行时崩溃 | 检查 macOS 版本兼容性<br/>查看终端错误输出 |

### 🔍 调试命令
```bash
# 检查应用是否可执行
ls -la target/release/shenliang

# 直接运行并查看输出
./target/release/shenliang

# 检查依赖是否正确链接
otool -L target/release/shenliang

# 如果已签名，检查签名状态
codesign --verify --verbose target/release/shenliang
```

### 🆘 重置环境
```bash
# 完全清理并重新构建
make clean
make build
```

## � 相关资源

### 🍎 Apple 官方文档
- [📖 Code Signing Guide](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/) - 代码签名完整指南
- [🔐 Notarization Guide](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution) - 应用公证流程
- [🎫 Entitlements Reference](https://developer.apple.com/documentation/bundleresources/entitlements) - 权限配置参考
- [🔑 Keychain Services](https://developer.apple.com/documentation/security/keychain_services) - Keychain 服务文档

### 🦀 Rust 生态
- [security-framework-rs](https://github.com/kornelski/rust-security-framework) - macOS 安全框架绑定
- [Rust macOS 开发指南](https://forge.rust-lang.org/infra/platform-support.html) - Rust macOS 平台支持

### �️ 开发工具
- [create-dmg](https://github.com/sindresorhus/create-dmg) - DMG 创建工具
- [Keychain Access](https://support.apple.com/guide/keychain-access/) - 钥匙串访问使用指南

## 📄 项目文件说明

| 文件 | 用途 |
|------|------|
| `📄 entitlements.plist` | 应用权限配置文件 |
| `🔧 sign_app.sh` | 应用签名自动化脚本 |
| `📤 notarize_app.sh` | 应用公证自动化脚本 |
| `📦 create_dmg.sh` | DMG 安装包创建脚本 |
| `⚙️ install_rust.sh` | Rust 开发环境安装脚本 |
| `📋 Makefile` | 构建和任务自动化配置 |
| `🚀 QUICKSTART.md` | 快速入门指南 |
| `📝 NON_XCODE_SIGNING_GUIDE.md` | 非 Xcode 签名指南 |

## 🤝 贡献指南

1. Fork 本项目
2. 创建特性分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m 'Add amazing feature'`
4. 推送分支: `git push origin feature/amazing-feature`
5. 创建 Pull Request

## 📋 版本历史

- **v0.1.0** - 初始版本
  - ✅ 基本 Keychain 功能
  - ✅ 应用签名和公证
  - ✅ DMG 创建
  - ✅ 交互式演示

## 📞 支持和反馈

如果您遇到问题或有建议，请：
1. 📝 [创建 Issue](../../issues) 报告 bug 或功能请求
2. 💬 查看 [Discussions](../../discussions) 参与讨论
3. 📖 阅读 [QUICKSTART.md](QUICKSTART.md) 获取快速入门指南

## 📜 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

⭐ **如果这个项目对你有帮助，请给它一个 Star！** ⭐
