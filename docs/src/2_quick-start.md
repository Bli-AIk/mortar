# Quick Start

Let's write your first Mortar dialogue! This guide will walk you through creating a simple interactive conversation.

## Your First Script

Create a file called `hello.mortar` and add the following content:

```mortar
// A basic dialogue node
node Start {
    // Text content - clean and readable
    text: "Hello! Welcome to this interactive story."
    
    // Events - triggered at specific character positions  
    events: [
        0, play_sound("greeting.wav")
        6, set_animation("wave")
        17, set_color("#FF6B6B")
    ]
    
    // Another text block with string interpolation
    text: $"Your name is {get_player_name()}, right?"
    events: [
        10, set_color("#33CCFF")
    ]
    
    // Jump to the next node
} -> ChoiceDemo

// A node with player choices
node ChoiceDemo {
    text: "What would you like to do?"
    
    choice: [
        "Explore the world" -> Exploration,
        "Check inventory" when has_backpack -> Inventory,
        "Say goodbye" -> return
    ]
}

node Exploration {
    text: "You venture forth into the unknown..."
}

// Function declarations - these connect to your game code
fn play_sound(file: String)
fn set_animation(name: String)  
fn set_color(color: String)
fn get_player_name() -> String
fn has_backpack() -> Bool
```

## Compiling Your Script

Use the Mortar CLI to compile your script:

```bash
# Basic compilation
mortar hello.mortar

# Pretty-printed output for debugging
mortar hello.mortar --pretty

# Custom output file
mortar hello.mortar -o dialogue.json
```

This generates a JSON file that your game can parse and execute.

## Understanding the Structure

### Nodes
Nodes are the building blocks of your dialogue. Each node can contain:
- **Text blocks**: The actual dialogue content
- **Events**: Actions triggered at specific character positions
- **Choices**: Player decision points
- **Navigation**: Jumps to other nodes

### Events and Indexing
Events are triggered based on character position in the text:
```mortar
text: "Hello World!"
events: [
    0, play_sound("hello.wav")    // Triggered at 'H'
    6, set_color("red")           // Triggered at 'W' 
]
```

### Choices
Create branching dialogue with the `choice` field:
```mortar
choice: [
    "Option 1" -> NextNode,
    "Conditional option" when condition -> AnotherNode,
    "Exit" -> return
]
```

## What's Next?

- Learn more about [Basic Concepts](./basic-concepts.md)
- Explore the complete [Syntax Reference](./syntax-reference.md)  
- See more [Examples](./examples/basic-dialogue.md)
- Set up [IDE Support](./ide-support.md) for better development experience

## JSON Output Example

Your Mortar script compiles to structured JSON like this:

```json
{
  "nodes": {
    "Start": {
      "text_blocks": [
        {
          "content": "Hello! Welcome to this interactive story.",
          "events": [
            {"index": 0, "action": "play_sound", "args": ["greeting.wav"]},
            {"index": 6, "action": "set_animation", "args": ["wave"]},
            {"index": 17, "action": "set_color", "args": ["#FF6B6B"]}
          ]
        }
      ],
      "next_node": "ChoiceDemo"
    }
  },
  "functions": {
    "play_sound": {"params": ["String"], "returns": null},
    "set_animation": {"params": ["String"], "returns": null}
  }
}
```

Perfect for game engine integration!