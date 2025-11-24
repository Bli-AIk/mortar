# Branch Interpolation

Mortar borrows from Fluent’s “non-symmetric localization” so writers can embed rich, language-specific variations without branching entire nodes. The v0.4 implementation (see `examples/branch_interpolation.mortar`) centers on **branch variables** that you declare alongside other `let` bindings.

## Declaring Branch Variables

Branch variables can be driven by booleans or enums.

```mortar
let is_forest: Bool = true
let is_city: Bool
let current_location: Location

enum Location {
    forest
    city
    town
}

let place: branch [
    is_forest, "forest"
    is_city, "city"
]

let location: branch<current_location> [
    forest, "deep forest"
    city, "bustling city"
    town, "quiet town"
]
```

You can also define ad-hoc branch blocks inside a node for single-use placeholders, but hoisting them to the top level keeps translations centralized and mirrors the structure from the official example.

## Using Branches in Nodes

Insert branch variables with the usual interpolation syntax:

```mortar
node LocationDesc {
    text: $"Welcome to the {place}! You are currently in the {location}."
}
```

Each branch variable becomes an entry in the node’s `branches` array. Engines resolve the correct case at runtime using the boolean or enum values you manage through [variables](./4_5_variables.md) and [control flow](./4_8_control-flow.md).

## Branch Events

Branch values have their own event lists and indices. Inline events (`with events`) operate on literal characters in the surrounding text, while branch-specific `events` arrays count from zero inside the placeholder. This matches the behavior in `examples/branch_interpolation.mortar`.

```mortar
text: $"You look toward the {object} and gasp!"
with events: [
    4, set_color("#33CCFF")
    18, set_color("#FF6B6B")
]

let object: branch<current_location> [
    forest, "ancient tree", events: [
        0, set_color("#228B22")
    ]
    city, "towering skyline", events: [
        0, set_color("#A9A9A9")
        9, set_color("#696969")
    ]
]
```

## Branches and Game Logic

Use branch selectors to keep localized snippets in sync with game state:

- Booleans toggle between quick phrases like `"sir"` vs `"ma'am"`.
- Enum-driven branches swap entire segments (“forest outskirts” vs “city plaza”).
- Branch events let each variant own its color cues, sound effects, or motion.

Because branch data is exported alongside nodes, your engine can cache every case, present preview tooling, or run automated localization checks without duplicating nodes or choices.
