# Command Line Interface

Mortar provides a simple and easy-to-use command-line tool for compiling `.mortar` files.

## Installation

### Install from crates.io (Recommended)

```bash
cargo install mortar_cli
```

### Build from Source

```bash
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release

# Compiled executable is at:
# target/release/mortar (Linux/macOS)
# target/release/mortar.exe (Windows)
```

### Verify Installation

```bash
mortar --version
```

Should display the version number, for example: `mortar 0.3.0`

## Basic Usage

### Simplest Compilation

```bash
mortar your_file.mortar
```

This generates a `.mortared` file with the same name (default is compressed JSON).

**For example**:
```bash
mortar hello.mortar
# Generates hello.mortared
```

### Formatted Output

If you want human-readable formatted JSON (with indentation and line breaks):

```bash
mortar hello.mortar --pretty
```

**Comparison**:

```bash
# Compressed format (default)
{"nodes":{"Start":{"texts":[{"content":"Hello"}]}}}

# Formatted output (--pretty)
{
  "nodes": {
    "Start": {
      "texts": [
        {
          "content": "Hello"
        }
      ]
    }
  }
}
```

### Specify Output File

Use `-o` or `--output` parameter:

```bash
mortar input.mortar -o output.json

# Or full form:
mortar input.mortar --output output.json
```

### Combining Options

```bash
# Formatted output to specified file
mortar hello.mortar -o dialogue.json --pretty

# Or this way:
mortar hello.mortar --output dialogue.json --pretty
```

## Complete Parameter List

```bash
mortar [OPTIONS] <INPUT_FILE>
```

### Required Parameters

- `<INPUT_FILE>` - Path to the `.mortar` file to compile

### Optional Parameters

| Parameter | Short | Description |
|-----------|-------|-------------|
| `--output <FILE>` | `-o` | Specify output file path |
| `--pretty` | - | Generate formatted JSON (with indentation) |
| `--version` | `-v` | Display version information |
| `--help` | `-h` | Display help information |

## Usage Scenarios

### Development Stage

During development, use `--pretty` for easy viewing and debugging:

```bash
mortar story.mortar --pretty
```

You can directly open the generated JSON to view structure.

### Production Environment

When releasing games, use compressed format to reduce file size:

```bash
mortar story.mortar -o assets/dialogues/story.json
```

## Error Handling

### Syntax Errors

If the Mortar file has syntax errors, the compiler will clearly indicate them:

```
Error: Unexpected token
  ┌─ hello.mortar:5:10
  │
5 │     text: Hello"
  │          ^ missing quote
  │
```

Error messages include:
- Error type
- Filename and location (line number, column number)
- Related code snippet
- Error hint

### Undefined Nodes

```
Error: Undefined node 'Unknown'
  ┌─ hello.mortar:10:20
  │
10 │     choice: ["Go" -> Unknown]
   │                      ^^^^^^^ this node doesn't exist
   │
```

### Type Errors

```
Error: Type mismatch
  ┌─ hello.mortar:8:15
  │
8 │     0, play_sound(123)
  │                   ^^^ expected String, got Number
  │
```

### File Not Found

```bash
$ mortar notfound.mortar
Error: File not found: notfound.mortar
```

## Exit Codes

Mortar CLI follows standard exit code conventions:

- `0` - Compilation successful
- `1` - Compilation failed (syntax error, type error, etc.)
- `2` - File read failed
- `3` - File write failed

This is especially useful in CI/CD scripts:

```bash
#!/bin/bash
if mortar dialogue.mortar; then
    echo "✅ Compilation successful"
else
    echo "❌ Compilation failed"
    exit 1
fi
```

## Common Questions

### Q: Why is compressed format the default?

A: Compressed format files are smaller and load faster, suitable for production. Use `--pretty` during development to view.

### Q: Can I compile an entire directory?

A: Not currently supported, but you can batch compile with shell scripts.

### Q: Can output be something other than JSON?

A: Currently only JSON output is supported. JSON is a universal format that almost all languages and engines can parse.

### Q: How to check syntax without generating a file?

A: There's no dedicated check mode currently, but you can output to a temporary file:
```bash
mortar test.mortar -o /tmp/test.json
```

## Summary

Key points of Mortar CLI:
- ✅ Simple and easy to use, one command does it all
- ✅ Clear error messages
- ✅ Supports formatted output for easy debugging
- ✅ Easy to integrate into development workflow
- ✅ Fast and reliable

## Next Steps

- Learn about editor support: [Editor Support](./6_2_ide-support.md)
- Check JSON output format: [JSON Output Format](./7_1_json-output.md)
- Back to quick start: [Quick Start Guide](./2_quick-start.md)
