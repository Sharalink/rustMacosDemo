#!/bin/bash

# macOS 应用签名脚本
# 使用方法: ./sign_app.sh [开发者证书名称]

set -e

# 配置变量
APP_NAME="shenliang"
BINARY_PATH="target/release/$APP_NAME"
ENTITLEMENTS_FILE="entitlements.plist"
BUNDLE_DIR="$APP_NAME.app"
BUNDLE_CONTENTS="$BUNDLE_DIR/Contents"
BUNDLE_MACOS="$BUNDLE_CONTENTS/MacOS"
BUNDLE_RESOURCES="$BUNDLE_CONTENTS/Resources"

# 开发者证书名称（从命令行参数获取，或使用默认值）
DEVELOPER_ID="${1:-Developer ID Application: Your Name (XXXXXXXXXX)}"

echo "🔨 构建 Release 版本..."
cargo build --release

echo "📦 创建应用包结构..."
# 清理并创建应用包目录
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_MACOS"
mkdir -p "$BUNDLE_RESOURCES"

echo "📋 创建 Info.plist..."
cat >"$BUNDLE_CONTENTS/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.yourcompany.$APP_NAME</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © $(date +%Y) Your Name. All rights reserved.</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
</dict>
</plist>
EOF

echo "📂 复制可执行文件..."
cp "$BINARY_PATH" "$BUNDLE_MACOS/$APP_NAME"

echo "🔐 开始代码签名..."

# 检查证书是否存在
if ! security find-identity -v -p codesigning | grep -q "$DEVELOPER_ID"; then
  echo "❌ 错误: 找不到证书 '$DEVELOPER_ID'"
  echo "📝 可用的代码签名证书:"
  security find-identity -v -p codesigning
  echo ""
  echo "💡 提示:"
  echo "1. 如果您有 Apple Developer 账户，请从 developer.apple.com 下载证书"
  echo "2. 对于测试，您可以创建自签名证书:"
  echo "   - 打开 Keychain Access"
  echo "   - Certificate Assistant > Create a Certificate"
  echo "   - Name: Test Certificate"
  echo "   - Certificate Type: Code Signing"
  echo "   然后使用: ./sign_app.sh 'Test Certificate'"
  exit 1
fi

echo "✅ 找到证书: $DEVELOPER_ID"

# 签名可执行文件
echo "🔏 签名可执行文件..."
codesign --force --options runtime --entitlements "$ENTITLEMENTS_FILE" --sign "$DEVELOPER_ID" "$BUNDLE_MACOS/$APP_NAME"

# 签名整个应用包
echo "🔏 签名应用包..."
codesign --force --options runtime --entitlements "$ENTITLEMENTS_FILE" --sign "$DEVELOPER_ID" "$BUNDLE_DIR"

echo "🔍 验证签名..."
codesign --verify --verbose "$BUNDLE_DIR"

echo "📋 显示签名信息..."
codesign --display --verbose=4 "$BUNDLE_DIR"

echo "✅ 签名完成!"
echo "📱 应用包位置: $BUNDLE_DIR"
echo ""
echo "🚀 下一步:"
echo "1. 测试应用: open $BUNDLE_DIR"
echo "2. 如需公证 (notarization): ./notarize_app.sh"
echo "3. 创建 DMG: ./create_dmg.sh"
