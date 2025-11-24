# Variables, Constants, and Initial State

Mortar v0.4 introduced workspace-wide state so dialogue can react to progress. All declarations must live at the top level of your script (outside `node` blocks or functions), which keeps Mortar’s runtime deterministic and easy to serialize.

## Declaring Variables

Use `let` followed by a name, type, and optional initializer. Mortar supports three primitive types: `String`, `Number`, and `Bool`.

```mortar
let player_name: String
let player_score: Number = 0
let is_live: Bool = true
```

Key rules:

- No `null`, default value (empty string, 0, `false`) will be provided if not assigned.
- Reassignment happens inside nodes: `player_score = player_score + 10`.
- Keep declarations outside nodes and functions so the compiler can include them in the top-level `variables` array of the `.mortared` file.

Every variable becomes a key in the exported JSON, making it trivial for your game to hydrate a hash map or dictionary.

## Public Constants for Key-Value Text

Non-dialogue UI strings or configuration can be defined via `pub const`. These entries are immutable and marked as `public` in the JSON output so localization pipelines or scripting layers can expose them.

```mortar
pub const welcome_message: String = "Welcome to the expedition!"
pub const continue_label: String = "Continue"
pub const exit_label: String = "Exit"
```

Constants are ideal when you need consistent button labels, notifications, or metadata for every language variant.

## Runtime Usage

Inside a node you can freely mix variable assignments with text:

```mortar
node AwardPoints {
    player_score = player_score + 5
    text: $"Score updated: {player_score}"
}
```

The serializer records those assignments under `pre_statements` or inline `text` blocks, ensuring the execution order mirrors the script. Use this mechanism to keep gameplay state close to the dialogue flow without turning Mortar into a general-purpose programming language. For richer domain modeling, pair variables with [enums](./4_10_enums.md) and [branch interpolation](./4_6_branch-interpolation.md).
