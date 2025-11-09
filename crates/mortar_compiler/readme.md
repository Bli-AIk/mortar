# Mortar Compiler

> **Current Status**: 🚧 Early Development (Initial version in progress)

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

## License

Mortar Compiler uses a dual-license model:

- **MIT License**: Allows free use, modification, and distribution
- **Apache License 2.0**: Distributed under Apache 2.0

You can choose either license according to your needs.