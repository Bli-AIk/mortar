# Installation

To use Mortar, you need to install the compilation tools. Don't worry, the process is simple!

## Method 1: Install with Cargo (Recommended)

If you already have [Rust](https://rust-lang.org/) installed, it's very convenient:

```bash
cargo install mortar_cli
```

Wait for the installation to complete, then check if it was successful:

```bash
mortar --version
```

Seeing the version number means the installation was successful!

## Method 2: Build from Source (Not for Average Users)

Want to experience the latest development version? You can build from source (this also requires a Rust development environment):

```bash
# Download source code
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# Build
cargo build --release

# The compiled program is here
./target/release/mortar --version
```

**Tip**: The compiled executable is located at `target/release/mortar`, you can add it to your environment variables.

## Method 3: Download from GitHub Release (Not for Average Users)

If you don't want to use Rust or Cargo, you can also download pre-compiled binaries directly from [Mortar's GitHub Release page](https://github.com/Bli-AIk/mortar/releases).

### Linux / macOS

1. Open the Release page and download the corresponding version, for example `mortar-x.x.x-linux-x64.tar.gz` or `mortar-x.x.x-macos-x64.tar.gz`.
2. Extract to any directory:

```bash
tar -xzf mortar-x.x.x-linux-x64.tar.gz -C ~/mortar
```

3. Add the executable path to environment variables, for example:

```bash
export PATH="$HOME/mortar:$PATH"
```

4. Check if installation was successful:

```bash
mortar --version
```

### Windows

1. Download the corresponding version of `mortar-x.x.x-windows-x64.zip`.
2. Extract to any directory, for example `D:\mortar`.
3. Add the directory to system environment variable PATH:
    * Right-click "This PC" → "Properties" → "Advanced system settings" → "Environment Variables"
    * Find `Path` in "System variables" or "User variables" → Edit → Add `D:\mortar`
4. Open a new command prompt and check installation:

```cmd
mortar --version
```

⚠️ **Note**:

* Need to manually set environment variables
* May encounter issues when opening new terminals or modifying system configuration
* Not very user-friendly for average users

Therefore, we recommend **Method 1 (Cargo)** for a smoother installation experience.

## Verify Installation

Run this command to test:

```bash
mortar --help
```

You should see help information explaining various usage options.

## Editor Support (Optional but Recommended)

For a better writing experience, you can install the language server:

```bash
cargo install mortar_lsp
```

Then configure it in your favorite editor.

Check [Editor Support](./6_2_ide-support.md) to learn how to configure your editor.

## Encountering Problems?

### "cargo command not found"

You need to install Rust first. Visit [https://rust-lang.org/](https://rust-lang.org/) and follow the installation guide.

### "Build failed"

Make sure your Rust version is new enough:

```bash
rustup update
```

### Other Issues

- Check [GitHub Issues](https://github.com/Bli-AIk/mortar/issues)
- Or ask in [Discussions](https://github.com/Bli-AIk/mortar/discussions)

## Next Steps

Installed? Then go try out [Quick Start Guide](./2_quick-start.md)!
