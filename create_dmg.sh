#!/bin/bash

# 创建 DMG 安装文件的脚本

set -e

APP_NAME="shenliang"
APP_BUNDLE="$APP_NAME.app"
DMG_NAME="$APP_NAME-installer"
DMG_TEMP="$DMG_NAME-temp"
DMG_FINAL="$DMG_NAME.dmg"

# 检查应用包是否存在
if [ ! -d "$APP_BUNDLE" ]; then
  echo "❌ 错误: 找不到应用包 $APP_BUNDLE"
  echo "请先运行 ./sign_app.sh 创建应用包"
  exit 1
fi

echo "📦 创建 DMG 安装文件..."

# 清理旧文件
rm -rf "$DMG_TEMP.dmg" "$DMG_FINAL"

# 创建临时 DMG
hdiutil create -size 100m -type SPARSE -fs HFS+ -volname "$APP_NAME" "$DMG_TEMP"

# 挂载 DMG
hdiutil attach "$DMG_TEMP.dmg" -mountpoint "/Volumes/$APP_NAME"

# 复制应用到 DMG
cp -R "$APP_BUNDLE" "/Volumes/$APP_NAME/"

# 创建 Applications 文件夹的符号链接
ln -s /Applications "/Volumes/$APP_NAME/Applications"

# 设置 DMG 外观（可选）
echo "🎨 设置 DMG 外观..."

# 创建隐藏的 .DS_Store 文件来设置窗口外观
cat >"/tmp/dmg_settings.applescript" <<'EOF'
tell application "Finder"
    tell disk "APP_NAME_PLACEHOLDER"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 900, 400}
        set viewOptions to the icon view options of container window
        set arrangement of viewOptions to not arranged
        set icon size of viewOptions to 72
        set position of item "APP_NAME_PLACEHOLDER.app" of container window to {150, 150}
        set position of item "Applications" of container window to {350, 150}
        close
        open
        update without registering applications
        delay 2
    end tell
end tell
EOF

# 替换占位符
sed -i '' "s/APP_NAME_PLACEHOLDER/$APP_NAME/g" "/tmp/dmg_settings.applescript"

# 运行 AppleScript（如果可能）
if command -v osascript &>/dev/null; then
  osascript "/tmp/dmg_settings.applescript" || echo "⚠️  无法设置 DMG 外观，继续..."
fi

# 卸载 DMG
hdiutil detach "/Volumes/$APP_NAME"

# 转换为最终的压缩 DMG
echo "🗜️  压缩 DMG..."
hdiutil convert "$DMG_TEMP.dmg" -format UDZO -o "$DMG_FINAL"

# 清理临时文件
rm -f "$DMG_TEMP.dmg"
rm -f "/tmp/dmg_settings.applescript"

echo "✅ DMG 创建完成: $DMG_FINAL"
echo "📱 您可以分发这个 DMG 文件给用户安装。"
