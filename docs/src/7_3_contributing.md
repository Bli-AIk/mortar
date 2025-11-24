# Contributing Guide

Thank you for your interest in Mortar! We welcome all forms of contributions.

## Ways to Contribute

You can contribute to Mortar in the following ways:

- 🐛 **Report Bugs** - If you find a problem, let us know.
- ✨ **Propose New Features** - Share your creative ideas.
- 📝 **Improve Documentation** - Make the documentation clearer.
- 💻 **Submit Code** - Fix bugs or implement new features.
- 🌍 **Translate** - Help translate the documentation.
- 💬 **Answer Questions** - Help others in the Discussions.
- ⭐ **Share the Project** - Let more people know about Mortar.

## Code of Conduct

When participating in the Mortar community, please:

- ✅ Be friendly and respectful.
- ✅ Welcome newcomers.
- ✅ Accept constructive criticism.
- ✅ Focus on what is best for the community.

We are committed to providing a friendly, safe, and welcoming environment for all.

## Reporting Bugs

### Before Reporting

1. **Search Existing Issues** - Confirm that the issue has not already been reported.
2. **Update to the Latest Version** - Confirm that the issue still exists in the latest version.
3. **Prepare a Minimal Reproduction Example** - Simplify the problem as much as possible.

### How to Report

Go to [GitHub Issues](https://github.com/Bli-AIk/mortar/issues/new) to create a new issue.

**A good bug report should include**:

```markdown
## Description
A brief description of the problem.

## Steps to Reproduce
1. Create a file with this content...
2. Run this command...
3. See the error...

## Expected Behavior
What should happen.

## Actual Behavior
What actually happened.

## Minimal Reproduction Example
```mortar
// The minimal code that can reproduce the problem
node TestNode {
    text: "..."
}
```

## Environment Information
- Mortar Version: 0.3.0
- Operating System: Windows 11 / macOS 14 / Ubuntu 22.04
- Rust Version (if building from source): 1.75.0
```

**Example**:

```markdown
## Description
Compiler crashes when compiling a node with an empty choice list.

## Steps to Reproduce
1. Create a file `test.mortar`.
2. Write the following content:
   ```mortar
   node TestNode {
       text: "Hello"
       choice: []
   }
   ```
3. Run `mortar test.mortar`.
4. The program crashes.

## Expected Behavior
Should give a friendly error message: "Choice list cannot be empty".

## Actual Behavior
The program crashes directly, showing:
```
thread 'main' panicked at 'index out of bounds'
```

## Environment Information
- Mortar Version: 0.3.0
- Operating System: Windows 11
```

## Proposing New Features

### Before Proposing

1. **Search Existing Issues** - Confirm that the feature has not already been proposed.
2. **Consider the Necessity** - Is this feature useful for most users?
3. **Consider Alternatives** - Are there other ways to implement it?

### How to Propose

Start a discussion on [GitHub Discussions](https://github.com/Bli-AIk/mortar/discussions).

**A good feature proposal should include**:

```markdown
## Problem/Need
Describe the problem you encountered or the need you want to solve.

## Proposed Solution
Describe in detail the feature you want to add.

## Example
Show how the feature would be used.

## Alternatives
Have you considered other implementation methods?

## Impact
Will this feature affect existing users?
```

**Example**:

```markdown
## Problem/Need
When writing large dialogues, I often need to share function declarations between multiple files.
Currently, I need to repeat the declarations in each file, which is very troublesome.

## Proposed Solution
Add an `import` syntax to import function declarations from other files:

```mortar
import functions from "common_functions.mortar"

node MyNode {
    text: "Triggering shared logic."
    with events: [
        0, play_sound("test.wav")  // This function comes from common_functions.mortar
    ]
}
```

## Alternatives
1. Use a preprocessor to merge files.
2. Solve it at the game engine level.

## Impact
It will not affect existing code because the `import` keyword is not currently supported.
```

## Improving Documentation

The documentation is in the `docs/` directory and is written in Markdown.

### Types of Documentation

- **Tutorials** - Step-by-step guides for beginners.
- **How-to Guides** - Steps to solve a specific problem.
- **Reference** - Detailed technical descriptions.
- **Explanations** - Concepts and design ideas.

### Steps to Improve Documentation

1. Fork the repository.
2. Create a branch: `git checkout -b improve-docs`
3. Edit the documentation files.
4. Preview locally: `mdbook serve docs/zh-Hans` or `mdbook serve docs/en`
5. Commit your changes: `git commit -m "docs: improve installation instructions"`
6. Push the branch: `git push origin improve-docs`
7. Create a Pull Request.

### Documentation Style Guide

- **Clear and concise** - Explain complex concepts in simple language.
- **Friendly tone** - Like chatting with a friend.
- **Practical examples** - Provide runnable code.
- **Step-by-step** - From simple to complex.
- **Visual aids** - Use diagrams and emojis appropriately.
- **Code formatting** - Use syntax highlighting.

**Good documentation**:
```markdown
## Creating Your First Dialogue

Let's write a simple NPC dialogue:

```mortar
node Villager {
    text: "Hello, traveler!"
}
```

It's that simple! After saving the file, compile it:

```bash
mortar hello.mortar
```
```

**Bad documentation**:
```markdown
## Node Creation

To create a node, use the node keyword, followed by an identifier and a block.
Inside the block, use the text field to define the text content.
To compile, use the mortar command with the filename as an argument.
```

## Submitting Code

### Development Environment Setup

1. **Install Rust** (1.70+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/Bli-AIk/mortar.git
   cd mortar
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Code checks**:
   ```bash
   cargo clippy
   cargo fmt --check
   ```

### Project Structure

```
mortar/
├── crates/
│   ├── mortar_compiler/  # Compiler core
│   ├── mortar_cli/       # Command-line tool
│   ├── mortar_lsp/       # Language server
│   └── mortar_language/  # Main library
├── docs/                 # Documentation
└── tests/                # Integration tests
```

### Development Workflow

1. **Create an Issue** - Describe the changes you want to make.
2. **Fork the repository**.
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/bug-description
   ```

4. **Develop**:
   - Write code.
   - Add tests.
   - Run tests to ensure they pass.
   - Use clippy to check the code.

5. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

6. **Push the branch**:
   ```bash
   git push origin feature/my-feature
   ```

7. **Create a Pull Request**.

### Commit Message Guidelines

Use Conventional Commits:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Type**:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code formatting (does not affect code execution)
- `refactor` - Refactoring
- `test` - Adding tests
- `chore` - Changes to the build process or auxiliary tools

**Example**:

```
feat(compiler): add support for nested choices

Adds the ability to parse nested choices, now you can write:
choice: [
    "Option" -> [
        "Sub-option" -> Node
    ]
]

Closes #42
```

```
fix(cli): fix path issues on Windows

The path separator was incorrect when compiling on Windows, causing compilation to fail.
Now uses std::path::PathBuf to handle paths correctly.

Fixes #38
```

### Code Style

Follow the standard Rust style:

```bash
# Format code
cargo fmt

# Check code
cargo clippy -- -D warnings
```

**Code comments**:

```rust
// Good comment: explains why, not what
// Use a hash map instead of a vector because we need O(1) lookup speed
let mut nodes = HashMap::new();

// Bad comment: repeats the code content
// Create a new HashMap
let mut nodes = HashMap::new();
```

**Naming conventions**:

```rust
// Use snake_case
fn parse_node() { }
let node_name = "test";

// Use PascalCase for types
struct NodeData { }
enum TokenType { }

// Use SCREAMING_SNAKE_CASE for constants
const MAX_DEPTH: usize = 10;
```

### Testing

Add tests for new features:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_node() {
        let input = r#"
            node TestNode {
                text: "Hello"
            }
        "#;
        
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0].name, "Test");
    }
}
```

### Pull Request

**A good PR should**:

- ✅ Solve a single problem.
- ✅ Include tests.
- ✅ Update relevant documentation.
- ✅ Pass all CI checks.
- ✅ Have a clear description.

**PR description template**:

```markdown
## Changes
A brief description of what this PR does.

## Motivation
Why is this change needed? What problem does it solve?

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Code refactoring
- [ ] Other: ___

## Testing
How to test this change?

## Related Issue
Closes #issue_number

## Screenshots (if applicable)
```

### Code Review

After submitting a PR:

1. **CI Checks** - Ensure all automated tests pass.
2. **Wait for Review** - Maintainers will review your code.
3. **Respond to Feedback** - Make changes based on suggestions.
4. **Merge** - After approval, it will be merged into the main branch.

## Translating Documentation

Want to help translate the documentation into other languages? Great!

### Currently Supported Languages

- 🇨🇳 Simplified Chinese (zh-Hans)
- 🇬🇧 English (en)

### Adding a New Language

1. Create a new directory under `docs/`: `docs/your-language/`
2. Copy `book.toml` and modify the language settings.
3. Translate all `.md` files in the `src/` directory.
4. Test the build: `mdbook build docs/your-language`
5. Submit a PR.

### Translation Guidelines

- **Keep the structure consistent** - Do not change the documentation structure.
- **Localize examples** - Adjust examples based on cultural context.
- **Keep terminology consistent** - Maintain consistency in technical terms.
- **Do not translate code** - Keep code examples in English.
- **Update links** - Ensure internal links point to the corresponding language pages.

## Community

### Getting Help

- 💬 [GitHub Discussions](https://github.com/Bli-AIk/mortar/discussions) - Ask questions and have discussions.
- 🐛 [GitHub Issues](https://github.com/Bli-AIk/mortar/issues) - Report bugs.
- 📧 Email - See the project README.

### Stay Connected

- ⭐ Star the project to follow updates.
- 👀 Watch the repository to receive notifications.
- 🔔 Subscribe to Release notifications.

## License

Contributed code will be licensed under the same licenses as the project:

- MIT License
- Apache License 2.0

By submitting a PR, you agree to distribute your contributions under these licenses.

## Acknowledgements

Thank you to all contributors! Your help makes Mortar better ❤️

The list of contributors can be found in the project README.

---

Thank you again for your contribution! If you have any questions, feel free to ask in the Discussions 🎉
