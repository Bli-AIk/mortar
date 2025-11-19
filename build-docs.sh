#!/bin/bash

# Mortar 文档构建脚本
# 构建所有语言版本的文档

set -e

echo "🔨 构建 Mortar 文档..."

# 检查是否安装了 mdbook
if ! command -v mdbook &> /dev/null; then
    echo "❌ 错误: 未找到 mdbook"
    echo "请运行以下命令安装:"
    echo "  cargo install mdbook"
    exit 1
fi

# 进入项目根目录
cd "$(dirname "$0")"

echo "📖 构建英文文档..."
cd docs
mdbook build
cd ..

echo "📖 构建中文文档..."
cp -r docs/theme docs/zh-Hans/ 2>/dev/null || true  # Copy theme files for Chinese docs
cd docs/zh-Hans
mdbook build
cd ../..

echo "✅ 文档构建完成!"
echo "📁 英文文档: docs/book/en/"
echo "📁 中文文档: docs/book/zh-Hans/"
echo ""
echo "🌐 要预览文档，运行:"
echo "  ./serve-en.sh   # 预览英文文档"
echo "  ./serve-zh.sh   # 预览中文文档"