# Makefile for Rust macOS Keychain App

APP_NAME = shenliang
CERT_NAME ?= "Test Certificate"

.PHONY: help build run test sign clean

help: ## 显示帮助信息
	@echo "Rust macOS Keychain 应用"
	@echo ""
	@echo "使用方法:"
	@echo "  make build      - 构建 release 版本"
	@echo "  make run        - 运行应用"
	@echo "  make test       - 测试 Keychain 访问"
	@echo "  make sign       - 签名应用 (测试用)"
	@echo "  make clean      - 清理构建文件"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## 构建 release 版本
	@echo "🔨 构建 Rust 应用..."
	cargo build --release

run: build ## 运行应用
	@echo "� 运行应用..."
	./target/release/$(APP_NAME)

test: build ## 测试 Keychain 访问
	@echo "🔐 测试 Keychain 访问..."
	./target/release/$(APP_NAME)

sign: build ## 签名应用 (仅用于测试)
	@echo "🔏 签名应用..."
	@if [ -x "./sign_app.sh" ]; then \
		./sign_app.sh "$(CERT_NAME)"; \
	else \
		echo "⚠️  签名脚本不存在，跳过签名步骤"; \
	fi

clean: ## 清理构建文件
	@echo "🗑️  清理文件..."
	cargo clean
	rm -rf $(APP_NAME).app
	rm -f $(APP_NAME)-installer.dmg
