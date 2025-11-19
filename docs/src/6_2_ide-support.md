# Editor Support

Mortar currently mainly provides editor support through LSP, helping you write dialogues more efficiently. Dedicated editor plugins are not yet available.

## Editor Adaptation Plan

Our plan is:

1. **First adapt JetBrains IDEs through LSP2IJ plugin** (such as IntelliJ IDEA, PyCharm, etc.)
2. **Then adapt VS Code**

Future editor features are expected to include:

* Automatic index updates
* Visual structure diagrams

## Language Server Protocol (LSP)

Mortar provides an LSP server, which is the core tool for implementing advanced editor features.

### Install LSP

```bash
cargo install mortar_lsp
```

### Features

#### 1. Real-time Error Checking

Discover errors while editing, without waiting for compilation:

```mortar
node TestNode {
    text: "Hello
    // ↑ Shows red underline: missing quote
}
```

#### 2. Go to Definition

Support Ctrl/Command + click on node or function names to jump to definition:

```mortar
node Start {
    choice: [
        "Next" -> NextNode  // ← Click to jump
    ]
}

node NextNode {  // ← Jump here
    text: "Arrived!"
}
```

#### 3. Find References

Right-click on node or function → "Find All References" to list all usage locations.

#### 4. Auto-completion

Provides auto-completion for keywords, defined nodes, function names, and type names.

#### 5. Hover Information

Mouse hover over elements shows type or function signature information.

#### 6. Code Diagnostics

LSP analyzes code and provides warnings or suggestions, such as unused nodes or functions.

### Using LSP in Different Editors

* **JetBrains IDE**: Adapt through LSP2IJ plugin
* **VS Code**: Will support through official plugin in the future
* **Neovim, Emacs, Sublime Text**: Can manually configure with respective LSP plugins

## Recommended Practices

Even without dedicated plugins, you can still get through LSP in IDE:

* Syntax highlighting (based on existing language features or manual configuration)
* Real-time error checking
* Go to definition and find references
* Auto-completion

Future expansion plans will further improve automatic indexing and structure visualization features.

## Summary

* Mortar currently mainly relies on **LSP** to support editor features
* JetBrains IDE will receive official adaptation first
* VS Code will receive support subsequently
* Feature coverage: error checking, completion, jumping, reference finding
* Planned additions: automatic index updates, structure visualization

Good tools make writing dialogues easier!


## Next Steps

- Learn about compilation tools: [Command Line Interface](./6_1_cli.md)
- Check output format: [JSON Output Format](./7_1_json-output.md)
- Back to quick start: [Quick Start Guide](./2_quick-start.md)
