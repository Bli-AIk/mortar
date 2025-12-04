# Mortar Language

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_language.svg)](https://crates.io/crates/mortar_language)
[![Documentation](https://docs.rs/mortar_language/badge.svg)](https://docs.rs/mortar_language)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar Language** 是 Mortar 语言生态系统的主要库 crate。它重新导出编译器和 LSP 服务器的核心功能，为 Mortar 语言工具提供统一接口。

## 功能特性
- Mortar 语言功能的统一 API
- 重新导出编译器和 LSP 服务器组件
- Mortar 语言集成的主要入口点
- 全面的语言支持

## 使用方法
```rust
use mortar_language::*;

// 访问编译器功能
let compiled = compile_mortar_file("script.mortar")?;

// 访问 LSP 功能进行 IDE 集成
// （实现细节取决于您的使用场景）
```

## 包含内容
- 完整的 Mortar 编译器功能
- 语言服务器协议 (LSP) 支持
- AST 定义和解析
- 错误处理和报告

## 集成
此 crate 设计为需要处理 Mortar 文件的应用程序的主要依赖项，提供编译、分析和 IDE 支持所需的一切功能。

## 许可证

Mortar Language 采用双许可证模式：

- **MIT 许可证**：允许免费使用、修改和分发
- **Apache 许可证 2.0**：在 Apache 2.0 下分发

您可以根据需要选择任一许可证。