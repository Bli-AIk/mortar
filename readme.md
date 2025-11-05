# Mortar DSL

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/mortar.svg)](https://crates.io/crates/mortar)
[![Documentation](https://docs.rs/mortar/badge.svg)](https://docs.rs/mortar)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **Current Status**: 🚧 Early Development (Initial version in progress)

**Mortar** is a Domain Specific Language (DSL) designed for game dialogue and text event systems. Its core philosophy is to achieve **strict separation between text content and event logic**.

| English         | Simplified Chinese          |
|-----------------|-----------------------------|
| English Version | [简体中文](./readme_zh-hant.md) |

## Introduction

Mortar is inspired by [ink](https://github.com/inkle/ink) and [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner),
but its key distinction is:

> **Mortar aims for strict separation of text content and event logic**

* **Text Part**: Pure narrative content, written entirely by humans, with no event logic mixed in;
* **Event Part**: System execution commands, used to control presentation effects, independent of text content;
* **Mortar Language Itself**: Provides an elegant bridge, allowing the two to be clearly associated and remain unpolluted.

## Design Goals

Mortar's design adheres to the following core principles: **Content Separation, Clear Semantics, Program Friendly**

1.  **Decoupling Content and Logic**: Events are triggered by character indices, avoiding rich text markup polluting the content; the text contains no control markers, maintaining purity.
2.  **Clear Semantics**: Adopting a Rust-style syntax design, the DSL syntax is intuitive, readable, and maintainable.
3.  **Program Friendly**: Compiles into a JSON structure, supporting tailored parsing by the user.

## Quick Start

### Installation

```bash
# Install from crates.io (Not yet complete)
cargo install mortar

# Or build from source
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release
```

### Basic Usage

Create a simple Mortar file `hello.mortar`:

```mortar
// Node definition (Rust-style struct)
node start {
    text: "Hello, welcome to this interactive story.",
    events: vec![
        Event::at(0).play_sound("greeting.wav"),
        Event::at(6).set_animation("wave"),
        Event::at(17).set_color("#FF6B6B"),
    ],
}

node choice_point {
    text: "What would you like to do?",
    
    choices: vec![
        Choice::new("Explore the forest") => forest_scene,
        Choice::new("Stay in town")
            .when(has_map == true) => town_scene,
        Choice::new("Check inventory") => inventory,
    ],
}
```

Compile the Mortar file:

```bash
# Basic compilation (outputs hello.mortared)
mortarc hello.mortar

# Specify output file
mortarc hello.mortar -o hello.json

# Enable verbose output
mortarc hello.mortar --verbose
```

## Applicable Scenarios

* 🎮 **Game Dialogue Systems**: RPG dialogue, visual novels
* 📖 **Interactive Fiction**: Text adventures, branching narratives
* 📚 **Educational Content**: Interactive instruction, guided learning scenarios
* 🤖 **Chat Scripts**: Structured dialogue logic
* 🖼️ **Multimedia Presentation**: Synchronization of text and media events

## Development Progress

Features to be implemented:

* 🚧 **Command Line Tool**: Complete CLI compiler
* 🚧 **Lexer**: High-performance tokenization using logos
* 🚧 **Parsing Framework**: Support for complete token parsing
* 🚧 **AST Structure**: Complete Abstract Syntax Tree definition
* 🚧 **Node Definition**: Support for `text`, `speaker`, `tags` fields
* 🚧 **Event System**: Event types and AST construction
* 🚧 **Choice System**: Builder pattern-based choice definition
* 🚧 **Error Handling**: `ariadne` friendly error reporting
* 🚧 **JSON Output**: Standardized output format

Planned features:

* 🚧 **Advanced Syntax Parsing**: Full event and choice syntax
* 🚧 **Conditional Expressions**: Support for complex logic
* 🚧 **Variable System**: Global and local variable management
* 🚧 **Function Calls**: Built-in and custom functions
* 🚧 **Language Server**: IDE integration and syntax highlighting

## Contributing

Community contributions are welcome\! Please see the [Contributing Guide](https://www.google.com/search?q=./CONTRIBUTING.md) for details.

### Contributors

The following people have contributed to this project.

\<a href = "[https://github.com/Bli-AIk/mortar/Python/graphs/contributors](https://github.com/Bli-AIk/mortar/Python/graphs/contributors)"\>
\<img src = "[https://contrib.rocks/image?repo=Bli-AIk/mortar](https://contrib.rocks/image?repo=Bli-AIk/mortar)" alt=""/\>
\</a\>

**A heartfelt thank you to each and every one of you\! 🎔**

### Development Environment Setup

```bash
# Clone the repository
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# Install dependencies and build
cargo build

# Run tests
cargo test

# Code check
cargo clippy

# Format code
cargo fmt
```

## License

Mortar uses a dual-license model:

### MIT License

Allows anyone to use, copy, modify, and distribute this software free of charge.

### Apache License 2.0

Distributed under the Apache 2.0 license.

You can choose either license according to your needs.
See [LICENSE-MIT](https://www.google.com/search?q=./LICENSE-MIT) and [LICENSE-APACHE](https://www.google.com/search?q=./LICENSE-APACHE) for details.

## Community

* **GitHub Issues**: [Report issues or suggest features](https://github.com/Bli-AIk/mortar/issues)
* **Discussions**: [Community Q\&A and discussion](https://github.com/Bli-AIk/mortar/discussions)

## Related Projects

* [ink](https://github.com/inkle/ink) — Inkle's narrative scripting language
* [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner) — A tool for building interactive dialogue

## Acknowledgments

Special thanks to the creators of ink and Yarn Spinner for paving the way for interactive narrative tools.

Also, thanks to the Rust community for providing excellent parsing and compilation-related libraries, enabling Mortar's creation.

