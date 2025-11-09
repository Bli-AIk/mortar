# Mortar LSP

> **Current Status**: 🚧 Early Development (Initial version in progress)

**Mortar LSP** is the Language Server Protocol (LSP) implementation for the Mortar language. It provides IDE integration features such as syntax highlighting, error reporting, auto-completion, and more.

## Features
- **LSP Compliance**: Implements the Language Server Protocol standard
- **Syntax Highlighting**: Rich syntax highlighting for Mortar files
- **Error Diagnostics**: Real-time error checking and reporting
- **Auto-completion**: Intelligent code completion suggestions
- **Cross-Platform**: Works with any LSP-compatible editor

## Supported Editors
Any editor with LSP support, including:
- Visual Studio Code
- Vim/Neovim (with LSP plugins)
- Emacs (with lsp-mode)
- Sublime Text
- And many more...

## Installation
```bash
cargo install mortar_lsp
```

## Usage
The LSP server runs as a background process and communicates with your editor through the LSP protocol. Configuration depends on your specific editor.

## Development
The server is built using:
- `tower-lsp-server` for LSP protocol implementation
- `tokio` for async runtime
- `mortar_compiler` for language analysis

## License

Mortar LSP uses a dual-license model:

- **MIT License**: Allows free use, modification, and distribution
- **Apache License 2.0**: Distributed under Apache 2.0

You can choose either license according to your needs.