# Mortar Language

> **Current Status**: 🚧 Early Development (Initial version in progress)

**Mortar Language** is the main library crate for the Mortar language ecosystem. It re-exports core functionality from the compiler and LSP server, providing a unified interface for Mortar language tools.

## Features
- Unified API for Mortar language functionality
- Re-exports compiler and LSP server components
- Primary entry point for Mortar language integration
- Comprehensive language support

## Usage
```rust
use mortar_language::*;

// Access compiler functionality
let compiled = compile_mortar_file("script.mortar")?;

// Access LSP functionality for IDE integration
// (Implementation details depend on your use case)
```

## What's Included
- Complete Mortar compiler functionality
- Language Server Protocol (LSP) support
- AST definitions and parsing
- Error handling and reporting

## Integration
This crate is designed to be the primary dependency for applications that need to work with Mortar files, providing everything needed for compilation, analysis, and IDE support.

## License

Mortar Language uses a dual-license model:

- **MIT License**: Allows free use, modification, and distribution
- **Apache License 2.0**: Distributed under Apache 2.0

You can choose either license according to your needs.