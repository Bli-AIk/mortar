# Basic Concepts

Understanding these core concepts will help you write effective Mortar scripts.

## Nodes

Nodes are the fundamental building blocks of Mortar dialogue. Think of them as scenes or conversation segments.

```mortar
node WelcomeScene {
    text: "Welcome to our tavern!"
    
    // Nodes can connect to other nodes
} -> MainMenu
```

**Key characteristics:**
- Each node has a unique identifier
- Nodes contain text, events, and/or choices
- Nodes can jump to other nodes or return/break

## Text and Events

Mortar's core philosophy is separating clean text from interactive events.

### Text Blocks
Text blocks contain your dialogue content:

```mortar
node Example {
    text: "Hello there, traveler!"
    text: "What brings you to these lands?"
}
```

### Event System
Events are triggered at specific character positions:

```mortar
node Example {
    text: "The dragon roars loudly!"
    events: [
        4, play_sound("dragon_roar.wav")  // At 'd' in "dragon"
        11, screen_shake(intensity: 3)    // At 'r' in "roars"
    ]
}
```

**Event indexing starts at 0** and counts Unicode characters, not bytes.

## String Interpolation

Use the `$` prefix for dynamic text with function calls:

```mortar
node Greeting {
    text: $"Welcome back, {get_player_name()}!"
    text: $"You have {get_gold_count()} gold pieces."
}
```

## Choices

Create branching dialogue with the `choice` field:

```mortar
node DecisionPoint {
    text: "Which path do you choose?"
    
    choice: [
        "Take the forest path" -> ForestPath,
        "Follow the river" -> RiverPath,
        "Go back" -> return
    ]
}
```

### Conditional Choices
Choices can have conditions using the `when` keyword:

```mortar
choice: [
    "Use magic spell" when has_magic -> CastSpell,
    "Attack with sword" when has_sword -> SwordAttack,
    "Try to negotiate" -> Negotiate
]
```

## Function Declarations

Declare functions that your game will implement:

```mortar
// Audio functions
fn play_sound(filename: String)
fn stop_music()

// Visual effects
fn set_color(hex_color: String)
fn screen_shake(intensity: Number)

// Game state queries
fn get_player_name() -> String
fn has_magic() -> Bool
fn get_gold_count() -> Number
```

**Supported types:**
- `String` - Text data
- `Number` - Numeric values (integers and floats)
- `Bool` / `Boolean` - True/false values

## Navigation

Control dialogue flow with navigation keywords:

### Jump to Node
```mortar
node A {
    text: "Moving to scene B"
} -> SceneB
```

### Return
Exit the current node or choice block:
```mortar
choice: [
    "Leave conversation" -> return
]
```

### Break
Stop processing the current choice list:
```mortar
choice: [
    "I don't know" -> break
]
// Execution continues here after break
text: "Let me think about it..."
```

## Comments

Use `//` for single-line comments and `/* */` for multi-line:

```mortar
// This is a single-line comment
node Example {
    /* 
     * Multi-line comment
     * for detailed explanations
     */
    text: "Hello!"  // Inline comment
}
```

## Data Flow

Here's how Mortar processes your script:

1. **Parse**: Your `.mortar` file is tokenized and parsed
2. **Validate**: Type checking and reference validation  
3. **Compile**: Generate structured JSON output
4. **Execute**: Your game loads and interprets the JSON

## Best Practices

- **Keep nodes focused**: Each node should represent a single conversation moment
- **Use descriptive names**: Node and function names should be clear
- **Organize events logically**: Group related events near the text they affect
- **Comment complex logic**: Explain conditional choices and event timing

## Next Steps

Now that you understand the basics:
- Dive deeper into [Syntax Reference](./syntax-reference.md)
- Learn about [Advanced Features](./advanced-features.md)
- See practical [Examples](./examples/basic-dialogue.md)