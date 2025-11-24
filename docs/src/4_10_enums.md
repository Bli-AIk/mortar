# Enums and Structured Choices

Enums complement Mortar’s variable system by letting you represent a closed set of states—chapter progression, affinity tiers, weather, and more. They pair naturally with [branch interpolation](./4_6_branch-interpolation.md) and `if` statements.

## Declaring Enums

Define enums at the top level:

```mortar
enum GameState {
    start
    playing
    game_over
}
```

Every variant becomes a string literal in the serialized JSON, making it easy for your engine to switch or pattern-match.

## Using Enum Variables

Create variables whose type is the enum:

```mortar
let current_state: GameState
```

Assignments work like any other variable:

```mortar
node StateMachine {
    if boss_defeated() {
        current_state = game_over
    }
}
```

Inside text, combine them with `branch` placeholders to produce localized snippets:

```mortar
let status: branch<current_state> [
    start, "just getting started"
    playing, "in the thick of it"
    game_over, "wrapping up the journey"
]

node Status {
    text: $"Current state: {status}."
}
```

## Engine Integration

The `.mortared` file exposes enums under the top-level `enums` array so you can validate assignments or show debugging tools. Typical usage:

1. Load enums into a registry or generate native enums from them.
2. Track Mortar variables (such as `current_state`) alongside your gameplay state.
3. Feed those values back into Mortar via assignments or function calls.

By treating enums as first-class citizens you keep designer intent obvious while empowering the engine to enforce valid transitions.
