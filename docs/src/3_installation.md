# Installation

Get Mortar up and running on your system with these installation options.

## Prerequisites

- **Rust**: Version 1.70 or later (if building from source)
- **Git**: For cloning the repository (if building from source)

## Option 1: Install from Crates.io (Recommended)

The easiest way to install Mortar is from the official Rust package registry:

```bash
# Install the complete Mortar CLI
cargo install mortar_cli

# Verify installation
mortar --version
```

You can also install individual components:

```bash
# Language Server for IDE support
cargo install mortar_lsp

# Core library only (for Rust projects)
# Add to your Cargo.toml:
[dependencies]
mortar_language = "0.3"
```

## Option 2: Build from Source

For the latest development features or to contribute:

```bash
# Clone the repository
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# Build all components
cargo build --release

# Install CLI globally
cargo install --path crates/mortar_cli

# Install LSP server
cargo install --path crates/mortar_lsp
```

## Option 3: Download Pre-built Binaries

Pre-built binaries are available on the [GitHub Releases page](https://github.com/Bli-AIk/mortar/releases):

1. Download the appropriate binary for your platform
2. Extract the archive
3. Add the binary to your system PATH

## Verify Installation

Test that Mortar is correctly installed:

```bash
# Check CLI version
mortar --version

# Try compiling a simple script
echo 'node Test { text: "Hello World!" }' > test.mortar
mortar test.mortar --pretty
```

## IDE Support Setup

### Visual Studio Code

1. Install the Mortar extension from the VS Code marketplace
2. The extension automatically uses your installed `mortar_lsp` server

### Other IDEs

For IDEs with Language Server Protocol support:

1. Install `mortar_lsp`:
   ```bash
   cargo install mortar_lsp
   ```

2. Configure your IDE to use `mortar_lsp` for `.mortar` files
3. Set the language ID to `mortar`

## Development Dependencies (Optional)

For contributing to Mortar development:

```bash
# Install development tools
cargo install mdbook          # For documentation
cargo install cargo-tarpaulin # For test coverage
cargo install cargo-audit     # For security audits

# Run development commands
cargo test                    # Run tests
cargo clippy                  # Lint code
cargo fmt                     # Format code
```

## Troubleshooting

### Common Issues

**`mortar` command not found**
- Ensure `~/.cargo/bin` is in your PATH
- Run `source ~/.bashrc` or restart your terminal

**Build fails on older Rust versions**
- Update Rust: `rustup update stable`
- Minimum required version is Rust 1.70

**LSP not working in IDE**
- Verify `mortar_lsp` is installed: `which mortar_lsp`
- Check IDE LSP configuration
- Restart your IDE after installation

**Permission denied on Unix systems**
- Ensure the binary is executable: `chmod +x mortar`

### Getting Help

- **GitHub Issues**: [Report bugs or ask questions](https://github.com/Bli-AIk/mortar/issues)
- **Discussions**: [Community support](https://github.com/Bli-AIk/mortar/discussions)
- **Documentation**: You're reading it!

## Next Steps

Now that Mortar is installed:

1. Try the [Quick Start](./quick-start.md) guide
2. Explore [Basic Concepts](./basic-concepts.md)
3. Set up your [IDE Support](./ide-support.md)
4. Read about [Best Practices](./best-practices.md)