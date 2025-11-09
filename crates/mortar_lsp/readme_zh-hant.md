# Mortar LSP

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_lsp.svg)](https://crates.io/crates/mortar_lsp)
[![Documentation](https://docs.rs/mortar_lsp/badge.svg)](https://docs.rs/mortar_lsp)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **当前状态**：🚧 早期开发中（初始版本正在开发）

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar LSP** 是 Mortar 语言的语言服务器协议 (LSP) 实现。它提供 IDE 集成功能，如语法高亮、错误报告、自动补全等。

## 功能特性
- **LSP 兼容**：实现语言服务器协议标准
- **语法高亮**：为 Mortar 文件提供丰富的语法高亮
- **错误诊断**：实时错误检查和报告
- **自动补全**：智能代码补全建议
- **跨平台**：与任何支持 LSP 的编辑器配合使用

## 支持的编辑器
任何支持 LSP 的编辑器，包括：
- Visual Studio Code
- Vim/Neovim（配合 LSP 插件）
- JetBrains IDEs（配合 LSP2IJ 插件）
- Emacs（配合 lsp-mode）
- Sublime Text
- 以及更多...

## 安装
```bash
cargo install mortar_lsp
```

## 使用方法
LSP 服务器作为后台进程运行，通过 LSP 协议与您的编辑器通信。具体配置取决于您使用的编辑器。

## 开发
服务器使用以下技术构建：
- `tower-lsp-server` 用于 LSP 协议实现
- `tokio` 用于异步运行时
- `mortar_compiler` 用于语言分析

## 许可证

Mortar LSP 采用双许可证模式：

- **MIT 许可证**：允许免费使用、修改和分发
- **Apache 许可证 2.0**：在 Apache 2.0 下分发

您可以根据需要选择任一许可证。