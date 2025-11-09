# Mortar CLI

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_cli.svg)](https://crates.io/crates/mortar_cli)
[![Documentation](https://docs.rs/mortar_cli/badge.svg)](https://docs.rs/mortar_cli)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

> **Current Status**: 🚧 Early Development (Initial version in progress)

**Mortar CLI** is the command line interface for the Mortar language compiler. It provides the `mortar` command that allows you to compile Mortar files into JSON output.

## Installation
```bash
cargo install mortar_cli
```

## Usage
```bash
# Basic compilation (output .mortared file, which is essentially a JSON file)
mortar hello.mortar

# Generate formatted JSON with indentation
mortar hello.mortar --pretty

# Specify output file
mortar hello.mortar -o hello.json

# Enable verbose output
mortar hello.mortar --verbose
```

## Features
- Compile Mortar files to JSON format
- Command-line interface with intuitive options
- Verbose output for debugging
- Cross-platform support

## License

Mortar CLI uses a dual-license model:

- **MIT License**: Allows free use, modification, and distribution
- **Apache License 2.0**: Distributed under Apache 2.0

You can choose either license according to your needs.