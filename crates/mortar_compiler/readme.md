# Mortar Compiler

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_compiler.svg)](https://crates.io/crates/mortar_compiler)
[![Documentation](https://docs.rs/mortar_compiler/badge.svg)](https://docs.rs/mortar_compiler)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar Compiler** is the core compilation library for the Mortar language. It provides lexing, parsing, and code generation functionality for Mortar files.

## Features
- **High-performance Lexer**: Tokenization using the logos crate
- **Robust Parser**: Complete token parsing with chumsky
- **AST Generation**: Complete Abstract Syntax Tree definition
- **Error Reporting**: Friendly error messages using ariadne
- **JSON Output**: Standardized compilation output format

## Usage as Library
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

## Architecture
- Lexical analysis with logos
- Parsing with chumsky parser combinators
- Error handling with ariadne
- JSON serialization with serde

## Dependencies

| Crate                                                         | Version | Description                                |
|---------------------------------------------------------------|---------|--------------------------------------------|
| [ariadne](https://crates.io/crates/ariadne)                   | *       | Diagnostic error reporting                 |
| [chumsky](https://crates.io/crates/chumsky)                   | *       | Parser combinators                         |
| [logos](https://crates.io/crates/logos)                        | *       | High-performance lexer                     |
| [owo-colors](https://crates.io/crates/owo-colors)             | *       | Terminal color output                      |
| [serde](https://crates.io/crates/serde)                       | *       | Serialization framework                    |
| [serde_json](https://crates.io/crates/serde_json)             | *       | JSON serialization/deserialization         |
| [chrono](https://crates.io/crates/chrono)                     | *       | Date and time utilities                    |

### Dev Dependencies

| Crate                                                         | Version | Description                                |
|---------------------------------------------------------------|---------|--------------------------------------------|
| [tempfile](https://crates.io/crates/tempfile)                 | 3.25    | Temporary file management for tests        |
| [proptest](https://crates.io/crates/proptest)                 | 1.6     | Property-based / fuzz testing              |
| [arbitrary](https://crates.io/crates/arbitrary)               | 1.4     | Structured fuzzing data generation         |

## License

Mortar Compiler uses a dual-license model:

- **MIT License**: Allows free use, modification, and distribution
- **Apache License 2.0**: Distributed under Apache 2.0

You can choose either license according to your needs.