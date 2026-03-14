# Control Flow in Nodes

Mortar now supports `if`/`else` blocks inside nodes so you can gate text or events based on variables, function calls, or enum comparisons. This feature is intentionally lightweight—there are no loops yet—so dialogue remains readable.

## Syntax Overview

```mortar
let player_score: Number = 123

node ExampleNode {
    if player_score > 100 {
        text: "Perfect score!"
    } else {
        text: "Keep pushing."
    }
}
```

Each branch may contain any sequence of valid node statements: text, assignments, choices, or even nested `if` chains. The serializer flattens these blocks into `content` entries with a `condition` field, making the `.mortared` file declarative for your game engine.

## Supported Expressions

You can compare numbers, check booleans, and call functions (with or without arguments):

```mortar
fn has_map() -> Bool
fn get_hp() -> Number
fn get_hp_max() -> Number
fn has_item(name: String) -> Bool

node ExampleNode {
    // Boolean function call as condition
    if has_map() {
        text: "You spread the map across the stump."
    }

    // Function call in comparison
    if get_hp() > 0 {
        text: "Still standing!"
    }

    // Function calls on both sides of comparison
    if get_hp() >= get_hp_max() {
        text: "Your HP is already full."
    }

    // Function call with arguments
    if has_item("sword") {
        text: "You draw your sword."
    }

    // Negation and logical operators
    if get_hp() > 0 && !has_map() {
        text: "You're alive, but lost."
    }
}
```

Under the hood, expressions become AST nodes (binary operators, unary operators, identifiers, or literals). Use helper functions (`fn has_map() -> Bool`) whenever the condition depends on game-side data that Mortar itself cannot calculate.

## Best Practices

- Keep branches short. If you need radically different conversations, jump to distinct nodes using `choice` or `next`.
- Update state before the condition so both sides can reference the latest values.
- Combine with [variables](./4_5_variables.md) and [enums](./4_10_enums.md) to track structural progress (chapter, route, etc.).

Future releases plan to add `while` loops and more expressive statements, but today’s `if`/`else` building block already covers most dynamic text scenarios without compromising Mortar’s clarity.
