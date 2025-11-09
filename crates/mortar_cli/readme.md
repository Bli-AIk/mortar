# Mortar CLI

> **Current Status**: 🚧 Early Development (Initial version in progress)

**Mortar CLI** is the command line interface for the Mortar language compiler. It provides the `mortar` command that allows you to compile Mortar files into JSON output.

## Installation
```bash
cargo install mortar_cli
```

## Usage
```bash
# Basic compilation (outputs .mortared file)
mortar hello.mortar

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