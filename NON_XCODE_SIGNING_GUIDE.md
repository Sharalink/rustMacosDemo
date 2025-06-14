# 非 Xcode 项目代码签名完整配置指南

## ⚠️ 重要提醒
对于 Rust 等非 Xcode 项目，您必须严格按照以下顺序在 Apple Developer 网站手动配置所有内容。

## 📋 必须的预配置步骤

### 1. 在 developer.apple.com 配置 App ID
```
位置: Certificates, Identifiers & Profiles > Identifiers > App IDs

配置项:
- Bundle ID: com.yourcompany.shenliang (必须与代码中完全一致)
- App Services (Capabilities):
  ✅ App Groups (如需要)
  ✅ Keychain Sharing
  ✅ Network Extensions (如需要)
  ✅ Personal VPN (如需要)
  ✅ 其他您需要的服务
```

### 2. 配置 App Groups (如果使用)
```
位置: Certificates, Identifiers & Profiles > Identifiers > App Groups

配置项:
- Group ID: group.com.yourcompany.shared
- Description: Shared data between apps
```

### 3. 创建开发/分发证书
```
位置: Certificates, Identifiers & Profiles > Certificates

类型:
- 开发: Apple Development
- 分发: Developer ID Application (for Mac App Store外分发)
- 或者: Mac App Store Distribution (for Mac App Store)
```

### 4. 创建 Provisioning Profile
```
位置: Certificates, Identifiers & Profiles > Profiles

配置:
- Type: macOS App Development / Developer ID
- App ID: 选择步骤1创建的App ID
- Certificates: 选择对应证书
- Devices: 选择开发设备 (Developer ID不需要)
```

## 🚨 常见配置错误和陷阱

### 错误1: Bundle ID 不匹配
```bash
# entitlements.plist 中的
<string>TEAM_ID.com.yourcompany.shenliang</string>

# 必须与 sign_app.sh 中的 CFBundleIdentifier 完全一致
<string>com.yourcompany.shenliang</string>

# 也必须与 developer.apple.com 上的 App ID 完全一致
```

### 错误2: Entitlements 不匹配
```xml
<!-- entitlements.plist 中声明的权限 -->
<key>com.apple.security.application-groups</key>
<array>
  <string>group.com.yourcompany.shared</string>
</array>

<!-- 必须在 App ID 的 Capabilities 中启用对应服务 -->
<!-- 必须在 App Groups 中创建对应的 group -->
```

### 错误3: Team ID 错误
```bash
# 查看正确的 Team ID
security find-identity -v -p codesigning

# 输出示例：
# 1) ABC123DEF4 "Developer ID Application: Your Name (ABC123DEF4)"
#              ^^^^^^^^^^ 这是您的 Team ID
```

## 🔧 配置验证清单

### 在 developer.apple.com 检查：
- [ ] App ID 已创建，Bundle ID 正确
- [ ] 所需的 App Services 已启用
- [ ] App Groups 已创建（如需要）
- [ ] 证书已创建并下载到本地
- [ ] Provisioning Profile 已创建并下载

### 在本地项目检查：
- [ ] entitlements.plist 中的 Team ID 正确
- [ ] entitlements.plist 中的 Bundle ID 与 App ID 匹配
- [ ] entitlements.plist 中的权限与 App ID 的 Capabilities 匹配
- [ ] Info.plist 中的 CFBundleIdentifier 正确
- [ ] 证书已正确安装到 Keychain

## 🚀 推荐的开发流程

### 方案A: 纯手动管理 (适合简单项目)
1. 在 developer.apple.com 预先配置好所有内容
2. 下载证书和配置文件到本地
3. 在项目中硬编码所有 ID 和权限
4. 使用 codesign 命令签名

### 方案B: 脚本化管理 (适合复杂项目)
1. 使用 Apple 的 App Store Connect API 自动化配置
2. 编写脚本动态生成 entitlements.plist
3. 集成到 CI/CD 流程中

### 方案C: 混合方案 (推荐)
1. 在 developer.apple.com 手动配置基础设置
2. 使用环境变量管理不同的配置 (开发/生产)
3. 使用验证脚本确保配置正确性

## 💡 最佳实践建议

1. **创建配置文档**：记录所有 ID、证书、权限的对应关系
2. **使用版本控制**：entitlements.plist 等配置文件加入 Git
3. **环境隔离**：开发和生产使用不同的 Bundle ID
4. **自动验证**：使用脚本验证配置的一致性
5. **备份证书**：导出 .p12 文件作为备份

## 🔍 调试技巧

```bash
# 检查 App ID 配置
xcrun altool --list-apps -u "your-apple-id" -p "app-specific-password"

# 验证 entitlements
codesign -d --entitlements - YourApp.app

# 检查签名信息
codesign --display --verbose=4 YourApp.app

# 验证配置文件
security cms -D -i YourProfile.provisionprofile
```

确实，非 Xcode 项目的配置管理要复杂得多，需要更谨慎的规划和验证！
