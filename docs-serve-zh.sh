#!/bin/bash

# Mortar 文档本地预览脚本
# 用于启动中文文档服务器

set -e

echo "🚀 启动 Mortar 中文文档预览..."
echo "📚 文档地址: http://localhost:3001"
echo "⏹️  按 Ctrl+C 停止服务器"
echo ""

# 检查是否安装了 mdbook
if ! command -v mdbook &> /dev/null; then
    echo "❌ 错误: 未找到 mdbook"
    echo "请运行以下命令安装:"
    echo "  cargo install mdbook"
    exit 1
fi

# 进入中文文档目录
cd "$(dirname "$0")/docs/zh-Hans"

# 启动服务器
mdbook serve --port 3001 --open