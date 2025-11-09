# Mortar Compiler

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_compiler.svg)](https://crates.io/crates/mortar_compiler)
[![Documentation](https://docs.rs/mortar_compiler/badge.svg)](https://docs.rs/mortar_compiler)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **当前状态**：🚧 早期开发中（初始版本正在开发）

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar Compiler** 是 Mortar 语言的核心编译库。它为 Mortar 文件提供词法分析、语法分析和代码生成功能。

## 功能特性
- **高性能词法分析器**：使用 logos crate 进行标记化
- **健壮的解析器**：使用 chumsky 进行完整的标记解析
- **AST 生成**：完整的抽象语法树定义
- **错误报告**：使用 ariadne 提供友好的错误消息
- **JSON 输出**：标准化的编译输出格式

## 作为库使用
```rust
use mortar_compiler::{compile, CompileOptions};

let source = r#"
node Start {
    text: "Hello, world!"
} -> End
"#;

let result = compile(source, CompileOptions::default())?;
println!("{}", result.json);
```

## 架构
- 使用 logos 进行词法分析
- 使用 chumsky 解析器组合子进行语法分析
- 使用 ariadne 进行错误处理
- 使用 serde 进行 JSON 序列化

## 许可证

Mortar Compiler 采用双许可证模式：

- **MIT 许可证**：允许免费使用、修改和分发
- **Apache 许可证 2.0**：在 Apache 2.0 下分发

您可以根据需要选择任一许可证。