#!/bin/bash

# macOS 应用公证脚本
# 需要有效的 Apple Developer 账户和 App Store Connect API 密钥

set -e

APP_BUNDLE="shenliang.app"
ZIP_FILE="app-for-notarization.zip"

# Apple ID 和 App Store Connect 配置
APPLE_ID="your-apple-id@example.com"
TEAM_ID="XXXXXXXXXX"               # 您的 Team ID
APP_PASSWORD="xxxx-xxxx-xxxx-xxxx" # App-specific password

echo "📦 创建公证用的 ZIP 文件..."
ditto -c -k --keepParent "$APP_BUNDLE" "$ZIP_FILE"

echo "📤 上传到 Apple 进行公证..."
xcrun notarytool submit "$ZIP_FILE" \
  --apple-id "$APPLE_ID" \
  --password "$APP_PASSWORD" \
  --team-id "$TEAM_ID" \
  --wait

echo "🔗 将公证票据附加到应用..."
xcrun stapler staple "$APP_BUNDLE"

echo "🔍 验证公证状态..."
xcrun stapler validate "$APP_BUNDLE"

echo "🗑️ 清理临时文件..."
rm "$ZIP_FILE"

echo "✅ 公证完成!"
echo "📱 您的应用现在已完全签名和公证，可以分发了。"
