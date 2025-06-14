#!/bin/bash

# Rust 和相关工具安装脚本

set -e

echo "🦀 安装 Rust 开发环境"
echo "===================="

# 检查是否已安装 Rust
if command -v cargo &>/dev/null; then
  echo "✅ Rust 已安装"
  echo "📋 当前版本信息:"
  rustc --version
  cargo --version
else
  echo "📥 安装 Rust..."

  # 下载并安装 rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

  # 添加到环境变量
  source ~/.cargo/env

  echo "✅ Rust 安装完成"
  echo "📋 版本信息:"
  rustc --version
  cargo --version
fi

# 检查 Xcode Command Line Tools
echo ""
echo "🔧 检查 Xcode Command Line Tools..."
if xcode-select -p &>/dev/null; then
  echo "✅ Xcode Command Line Tools 已安装"
else
  echo "📥 安装 Xcode Command Line Tools..."
  xcode-select --install
  echo "⚠️  请完成 Xcode Command Line Tools 安装后重新运行此脚本"
  exit 1
fi

# 安装必要的系统工具
echo ""
echo "🛠️  检查系统工具..."

if command -v brew &>/dev/null; then
  echo "✅ Homebrew 已安装"
else
  echo "📥 安装 Homebrew..."
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

echo ""
echo "🎉 开发环境准备完成！"
echo ""
echo "🚀 下一步:"
echo "1. 重新启动终端或运行: source ~/.cargo/env"
echo "2. 运行: make build"
echo "3. 运行: make sign"
