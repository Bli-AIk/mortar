# Mortar Language

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/mortar.svg)](https://crates.io/crates/mortar)
[![Documentation](https://docs.rs/mortar/badge.svg)](https://docs.rs/mortar)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **Current Status**: 🚧 Early Development (Initial version in progress)

![Mortar](./crates/mortar_logo.svg)

**Mortar** is a Domain Specific Language (DSL) designed for game dialogue and text event systems. Its core philosophy is
to achieve **strict separation between text content and event logic**.

| English         | Simplified Chinese          |
|-----------------|-----------------------------|
| English Version | [简体中文](./readme_zh-hant.md) |

## Introduction

Mortar is inspired by [ink](https://github.com/inkle/ink)
and [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner),
but its key distinction is:

> **Mortar aims for strict separation of text content and event logic.**

* **Text Part**: Pure narrative content, written entirely by humans, with no event logic mixed in;
* **Event Part**: System execution commands, used to control presentation effects, independent of text content;
* **Mortar Language Itself**: Provides an elegant bridge, allowing the two to be clearly associated and remain
  unpolluted.

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
# Install from crates.io
cargo install mortar

# Or build from source
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release
```

### Basic Usage

Create a simple Mortar file `hello.mortar`:

```mortar
// 'node' is a basic dialogue node.
// It can also be abbreviated as 'nd'!
node Start {
    // Write your text content.
    // Double quotes (or single quotes) are required, but semicolons and commas can be omitted!
    text: "Hello, welcome to this interactive story."
    
    // This event list is written immediately next to the previous text, so they are related.
    events: [
        // Use index + event function to write events. Supports chain writing.
        // The index here represents the character position where the event is triggered (counting starts from 0).
        // Will it be tied to your game implementation... Where does the typewriter play? Audio timeline? 
        // Or anything else, it all works, it depends on how you implement it.
        0, play_sound("greeting.wav")
        6, set_animation("wave").play_sound("wave_sound.wav")
        17, set_color("#FF6B6B")
    ]
    // When we use the text field again, it means that this is another text block for the same node.
    // You can write several text blocks and they will be played sequentially.
    text: "I think your name is {get_name()}, right?"
    events: [
        // The index can be a floating point number! 
        // Generally speaking, decimal points are used for voice synchronization. And typewriters are integers.
        // In fact, the numbers in mortar are Number, which is the same as the number type in json.
        4.2, set_color("#33CCFF")
        10.8, set_color("#FF6B6B")
    ]
    
    // This text block has no events... which is perfectly legal!
    text: "Ok, Let's GO!"
    
// The arrow after a node indicates jumping to the next node.
} -> ChoicePoint

/*
There is also a node here that shows how to write options - by choice field.
*/

node ChoicePoint {
    text: "What would you like to do?"
    
    // By choice field, we can also jump to different nodes.
    choice: [
        // This option does not have any conditional judgment. Logically speaking, you can always choose it.
        "Explore the forest" -> forest_scene,
        
        // These two options have the when keyword, which means they have conditional judgment!
        // The when keyword supports chain writing and functional writing.
        ("Stay in town").when(has_map) -> town_scene,
        "Check inventory" when has_backpack  -> inventory,
        
        // A selection field can also be nested within a selection field. You can nest as many levels as you want!
        "Have something to eat" -> [
            "Apple" -> eat_apple,
            "Bread" -> eat_bread
        ]
        
        // Use the return keyword to exit the current node.
        // By the way, if this node has subsequent nodes, 
        // then return will not terminate the entire conversation process, but will only exit the current node.
        "I don't want to talk to you anymore!!" -> return,
        
        // Use the break keyword to terminate the option list.
        "I don't know..." -> break,
    ],
    
    // In this selection field, you will only come to this line if you select "I don't know...".
    text: "What a shame. So let’s end the conversation first.",
    
    // Then, since there are no subsequent nodes, the conversation ends anyway.
}

// The functions called in start don't just appear for a reason - you need to define them in the Mortar file!
// This is a bit like function declaration in C/C++. 
// They will eventually be recognized by the compiler and correlated into your game code.
fn play_sound(file_name: String)

fn set_animation(anim_name: String)

fn set_color(value: String)

fn get_name() -> String

fn has_map() -> Boolean

fn has_backpack() -> Boolean
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
* ✅ **Lexer**: High-performance tokenization using logos
* ✅ **Parsing Framework**: Support for complete token parsing
* ✅ **AST Structure**: Complete Abstract Syntax Tree definition
* 🚧 **Error Handling**: `ariadne` friendly error reporting
* ✅ **JSON Output**: Standardized output format
* 🚧 **Language Server**: IDE integration and syntax highlighting

Planned features:

* 🚧 **Advanced Syntax Parsing**: Full event and choice syntax
* 🚧 **Conditional Expressions**: Support for complex logic
* 🚧 **Variable System**: Global and local variable management
* 🚧 **Function Calls**: Built-in and custom functions
## Contributing

Community contributions are welcome\! Please see
the [Contributing Guide](https://www.google.com/search?q=./CONTRIBUTING.md) for details.

### Contributors

The following people have contributed to this project.


<a href = "https://github.com/Bli-AIk/mortar/Python/graphs/contributors">
<img src = "https://contrib.rocks/image?repo=Bli-AIk/mortar" alt=""/>
</a>

**A heartfelt thank you to each and every one of you! 🎔**

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
See [LICENSE-MIT](https://www.google.com/search?q=./LICENSE-MIT)
and [LICENSE-APACHE](https://www.google.com/search?q=./LICENSE-APACHE) for details.

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

