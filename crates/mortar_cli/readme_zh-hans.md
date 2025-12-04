# Mortar CLI

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_cli.svg)](https://crates.io/crates/mortar_cli)
[![Documentation](https://docs.rs/mortar_cli/badge.svg)](https://docs.rs/mortar_cli)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar CLI** 是 Mortar 语言编译器的命令行界面工具。它提供了 `mortar` 命令，允许您将 Mortar 文件编译成 JSON 输出。

## 安装
```bash
cargo install mortar_cli
```

## 使用方法
```bash
# 基本编译（输出 .mortared 文件，其本质上就是 JSON 文件）
mortar hello.mortar

# 生成带缩进的格式化 JSON
mortar hello.mortar --pretty

# 指定输出文件
mortar hello.mortar -o hello.json

# 启用详细输出
mortar hello.mortar --verbose
```

## 功能特性
- 将 Mortar 文件编译为 JSON 格式
- 直观的命令行界面选项
- 用于调试的详细输出
- 跨平台支持

## 许可证

Mortar CLI 采用双许可证模式：

- **MIT 许可证**：允许免费使用、修改和分发
- **Apache 许可证 2.0**：在 Apache 2.0 下分发

您可以根据需要选择任一许可证。