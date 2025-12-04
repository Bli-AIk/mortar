# Mortar Language

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_language.svg)](https://crates.io/crates/mortar_language)
[![Documentation](https://docs.rs/mortar_language/badge.svg)](https://docs.rs/mortar_language)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar** is a Domain Specific Language (DSL) designed for game dialogue and text event systems. Its core philosophy is
to achieve **strict separation between text content and event logic**.

Read the [official guide](https://bli-aik.github.io/mortar/en) to learn how to use mortar!

| English         | Simplified Chinese          |
|-----------------|-----------------------------|
| English Version | [简体中文](./readme_zh-hans.md) |

## Introduction

Mortar is inspired by [ink](https://github.com/inkle/ink)
and [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner),
but its key distinction is:

> **Mortar aims for strict separation of text content and event logic.**

* **Text Part**: Pure narrative content, written entirely by humans, with no event logic mixed in;
* **Event Part**: System execution commands, used to control presentation effects, independent of text content;
* **Mortar Language Itself**: Provides an elegant bridge, allowing the two to be clearly associated and remain
  unpolluted.

> `Mortar Language` is a derivative of the SoupRune project and is the language of choice for dialogue systems.
>
> SoupRune is a game framework specifically for Deltarune / Undertale Fangame.  [Learn more](https://github.com/Bli-AIk/souprune).

## Design Goals

Mortar's design adheres to the following core principles: **Content Separation, Clear Semantics, Program Friendly,
Static typing.**

1. **Decoupling Content and Logic**: Events are triggered by character indices, avoiding rich text markup polluting the
   content; the text contains no control markers, maintaining purity.
2. **Clear Semantics**: Adopting a Rust-style syntax design, the DSL syntax is intuitive, readable, and maintainable.
3. **Program Friendly**: Compiles into a JSON structure, supporting tailored parsing by the user.
4. **Static typing**: As a statically typed language, type checking is performed at compile time to catch type errors in
   advance and improve runtime reliability.

## Quick Start

### Installation

```bash
# Install the CLI tool from crates.io
cargo install mortar_cli

# Or build from source
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release
```

### Minimal Example

Create a file named `hello.mortar`:

```mortar
node Start {
    text: "Hello, welcome to Mortar!"
    text: "This is a minimal example."
}
```

### Advanced Features

Mortar supports complex events, choices, and logic:

```mortar
node Start {
    text: "Hello, welcome to this interactive story."
    
    // Event list associated with the text
    events: [
        0, play_sound("greeting.wav")
        6, set_animation("wave")
    ]
    
    text: $"I think your name is {get_name()}, right?"
    events: [
        4.2, set_color("#33CCFF")
    ]
    
} -> ChoicePoint

node ChoicePoint {
    text: "What would you like to do?"
    
    choice: [
        "Explore the forest" -> ForestScene,
        ("Stay in town").when(has_map) -> TownScene,
        "Have something to eat" -> [
            "Apple" -> EatApple,
            "Bread" -> EatBread
        ]
        "Quit" -> return,
    ]
}

// Function declarations
fn play_sound(file_name: String)
fn set_animation(anim_name: String)
fn set_color(value: String)
fn get_name() -> String
function has_map() -> Bool
```

Compile the Mortar file:

```bash
# Basic compilation (outputs compressed JSON by default)
mortar hello.mortar

# Generate formatted JSON with indentation  
mortar hello.mortar --pretty

# Specify output file
mortar hello.mortar -o output_file

# Combine options
mortar hello.mortar -o custom.json --pretty
```

The compiler now generates compressed JSON by default for optimal file size and performance. Use the `--pretty` flag when you need human-readable formatted output for debugging or review.

## Applicable Scenarios

* 🎮 **Game Dialogue Systems**: RPG dialogue, visual novels
* 📖 **Interactive Fiction**: Text adventures, branching narratives
* 📚 **Educational Content**: Interactive instruction, guided learning scenarios
* 🤖 **Chat Scripts**: Structured dialogue logic
* 🖼️ **Multimedia Presentation**: Synchronization of text and media events

## Development Progress

Features:

* ✅ **Command Line Tool**: Complete CLI compiler
* ✅ **Lexer**: High-performance tokenization using logos
* ✅ **Parsing Framework**: Support for complete token parsing
* ✅ **AST Structure**: Complete Abstract Syntax Tree definition
* ✅ **Error Handling**: `ariadne` friendly error reporting
* ✅ **JSON Output**: Standardized output format
* ✅ **Language Server**: IDE integration and syntax highlighting
* ✅ **Variable System**: Variable declarations, constants, and enums
* ✅ **Branch Interpolation**: Non-symmetric text support (inspired by [Fluent](https://github.com/projectfluent/fluent))
* ✅ **Conditional Expressions**: AND, OR, NOT, comparisons
* ✅ **Control Flow Statements**：if，else
* ✅ **Event System**: Extracts events into independent nodes

## Contributing

Community contributions are welcome! Please see
the [Contributing Guide](./CONTRIBUTING.md) for details.

### Contributors

The following people have contributed to this project.


<a href = "https://github.com/Bli-AIk/mortar/Python/graphs/contributors">
<img src = "https://contrib.rocks/image?repo=Bli-AIk/mortar" alt=""/>
</a>

**A heartfelt thank you to each and every one of you! 🎔**

## Project Structure

```mermaid
graph TD
    subgraph "Mortar Ecosystem"
        Compiler[mortar_compiler<br>(Core Logic)]
        CLI[mortar_cli<br>(Command Line)]
        LSP[mortar_lsp<br>(Language Server)]
        Lib[mortar_language<br>(Main Library)]
    end

    CLI --> Compiler
    LSP --> Compiler
    Lib --> Compiler
    Lib --> LSP
```

This project is organized as a Rust workspace with four main crates:

* **`mortar_language`** - The main library crate that re-exports functionality from all other crates
* **`mortar_compiler`** - Core compilation library with lexing, parsing, and code generation
* **`mortar_cli`** - Command-line interface providing the `mortar` command
* **`mortar_lsp`** - Language Server Protocol implementation for IDE integration

### Building the Project

```bash
# Clone the repository
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# Build all crates in the workspace
cargo build

# Build optimized release version
cargo build --release

# Build specific crate
cargo build -p mortar_cli
cargo build -p mortar_compiler
cargo build -p mortar_language
cargo build -p mortar_lsp

# Run tests for all crates
cargo test

# Run tests for specific crate
cargo test -p mortar_compiler

# Code check
cargo clippy

# Format code
cargo fmt
```

### Installing Individual Components

```bash
# Install CLI tool only
cargo install mortar_cli

# Install LSP server only  
cargo install mortar_lsp

# Use as library dependency in Cargo.toml
[dependencies]
mortar_language = "0.4"
# Or use individual components
mortar_compiler = "0.4"
```

## License

Mortar uses a dual-license model:

### MIT License

Allows anyone to use, copy, modify, and distribute this software free of charge.

### Apache License 2.0

Distributed under the Apache 2.0 license.

You can choose either license according to your needs.
See [LICENSE-MIT](./LICENSE-MIT)
and [LICENSE-APACHE](./LICENSE-APACHE) for details.

## Community

* **GitHub Issues**: [Report issues or suggest features](https://github.com/Bli-AIk/mortar/issues)
* **Discussions**: [Community Q\&A and discussion](https://github.com/Bli-AIk/mortar/discussions)

## Related Projects

* [ink](https://github.com/inkle/ink) — Inkle's narrative scripting language
* [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner) — A tool for building interactive dialogue

## Acknowledgments

Special thanks to the creators of ink and Yarn Spinner for paving the way for interactive narrative tools.

Also, thanks to the Rust community for providing excellent parsing and compilation-related libraries, enabling Mortar's
creation.
