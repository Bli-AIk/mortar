# Functions: Connecting to Game World

Functions are the bridge between Mortar and your game code. Through function declarations, you tell Mortar: "These features will be implemented by my game".

## Why Do We Need Function Declarations?

In Mortar scripts, you'll call various functions:

```mortar
text: "Boom!"
with events: [
    0, play_sound("boom.wav")
    2, shake_screen()
]
```

But where are these functions? They're in your game code!

**Function declarations** are a "contract":
- You tell Mortar: my game has these functions, what parameters they need, what they return
- Mortar checks types during compilation to ensure you use them correctly
- After compiling to JSON, your game implements these functions

## Function Naming Conventions

> **⚠️ Important: We recommend using snake_case**

**✅ Recommended naming style**:
```mortar
fn play_sound(file_name: String)         // snake_case: all lowercase, words separated by underscores
fn get_player_name() -> String           // Clear and readable
fn check_inventory_space() -> Bool       // Self-explanatory
fn calculate_damage(base: Number, modifier: Number) -> Number
```

**⚠️ Not recommended naming styles**:
```mortar
fn playSound() { }              // Avoid camelCase (that's other languages' style)
fn PlaySound() { }              // Don't use PascalCase (that's for nodes)
fn play-sound() { }             // Kebab-case not recommended
fn sonido_ñ() { }               // Non-ASCII text not recommended
fn playsound() { }              // All lowercase hard to read
```

**Parameter naming conventions**:
```mortar
// ✅ Good parameter naming
fn move_to(x: Number, y: Number)
fn load_scene(scene_name: String, fade_time: Number)

// ❌ Bad parameter naming
fn move_to(a: Number, b: Number)        // No semantics
fn load_scene(s: String, t: Number)     // Unclear abbreviations
```

**Naming suggestions**:
- Use English words, all lowercase
- Multiple words separated by underscore `_`
- Start with verb, describe function purpose: `get_`, `set_`, `check_`, `play_`, `show_`
- Parameter names should be descriptive
- Keep naming style consistent within project

## Basic Syntax

```mortar
fn function_name(param: Type) -> ReturnType
```

### No Parameters, No Return Value

```mortar
fn shake_screen()
fn clear_text()
fn show_menu()
```

### With Parameters, No Return Value

```mortar
fn play_sound(file: String)
fn set_color(color: String)
fn move_character(x: Number, y: Number)
```

### With Return Value

```mortar
fn get_player_name() -> String
fn get_gold() -> Number
fn has_key() -> Bool
```

### With Parameters and Return Value

```mortar
fn calculate(a: Number, b: Number) -> Number
fn find_item(name: String) -> Bool
```

## Supported Types

Mortar supports types from JSON:

| Type | Alias | Description | Example |
|------|-------|-------------|---------|
| `String` | - | String | `"Hello"`, `"file.wav"` |
| `Number` | - | Number (integer or decimal) | `42`, `3.14` |
| `Bool` | `Boolean` | Boolean | `true`, `false` |

**Note**: `Bool` and `Boolean` are the same, use whichever you prefer.

## Complete Example

```mortar
// A complete Mortar file

node StartScene {
    text: $"Welcome, {get_player_name()}!"
    with events: [
        0, play_bgm("theme.mp3")
    ]
    
    text: $"You have {get_gold()} gold."
    
    choice: [
        "Go to shop" when can_shop() -> Shop,
        "Go adventure" -> Adventure
    ]
}

node Shop {
    text: "Welcome to the shop!"
}

node Adventure {
    text: "Adventure begins!"
    with events: [
        0, start_battle("Goblin")
    ]
}

// ===== Function Declarations =====

// Play background music
fn play_bgm(music: String)

// Get player name
fn get_player_name() -> String

// Get gold amount
fn get_gold() -> Number

// Check if can shop
fn can_shop() -> Bool

// Start battle
fn start_battle(enemy: String)
```

## Using in Events

### Call Functions Without Parameters

```mortar
with events: [
    0, shake_screen()
    2, flash_white()
]

fn shake_screen()
fn flash_white()
```

### Call Functions With Parameters

```mortar
with events: [
    0, play_sound("boom.wav")
    2, set_color("#FF0000")
    4, move_to(100, 200)
]

fn play_sound(file: String)
fn set_color(hex: String)
fn move_to(x: Number, y: Number)
```

### Method Chaining

```mortar
with events: [
    0, play_sound("boom.wav").shake_screen().flash_white()
]

fn play_sound(file: String)
fn shake_screen()
fn flash_white()
```

## Using in Text Interpolation

Only functions with return values can be used in `${}`:

```mortar
text: $"Hello, {get_name()}!"
text: $"Level: {get_level()}"
text: $"Status: {get_status()}"

fn get_name() -> String
fn get_level() -> Number
fn get_status() -> String
```

**Note**: Functions in interpolation must return String!

```mortar
// ❌ Error: function has no return value
text: $"Result: {do_something()}"
fn do_something()  // No return value


// ❌ Error: return type is not String
text: $"Result: {get_hp()}"
fn get_hp() -> Number  // Wrong return type

// ✅ Correct
text: $"Result: {get_result()}"
fn get_result() -> String
```

## Using in Conditions

Functions after `when` must return `Bool` / `Boolean`:

```mortar
choice: [
    "Special option" when is_unlocked() -> SpecialNode
]

fn is_unlocked() -> Bool
```

## Position of Function Declarations

By convention, put all function declarations at the end of the file:

```mortar
// Node definitions
node A { ... }
node B { ... }
node C { ... }

// ===== Function Declarations =====
fn func1()
fn func2()
fn func3()
```

But position doesn't really matter, you can put them anywhere.

## Static Type Checking

Mortar checks types at compile time:

```mortar
// ✅ Correct
with events: [
    0, play_sound("file.wav")
]
fn play_sound(file: String)

// ❌ Error: wrong parameter type
with events: [
    0, play_sound(123)  // Passed number, but needs string
]
fn play_sound(file: String)
```

This helps you catch errors early!

## Implementing Functions (Game Side)

Mortar only handles declarations, actual implementation is in your game code.

The compiled JSON will contain function information:

```json
{
  "functions": [
    {
      "name": "play_sound",
      "params": [
        {"name": "file", "type": "String"}
      ]
    },
    {
      "name": "get_player_name",
      "return": "String"
    }
  ]
}
```

Your game reads the JSON and implements these functions.

See [Integrating with Your Game](./5_3_game-integration.md) for details.

## Best Practices

### ✅ Good Practices

```mortar
// Clear naming
fn play_background_music(file: String)
fn get_player_health() -> Number
fn is_quest_completed(quest_id: Number) -> Bool
```

```mortar
// Reasonable parameters
fn spawn_enemy(name: String, x: Number, y: Number)
fn set_weather(type: String, intensity: Number)
```

### ❌ Bad Practices

```mortar
// Unclear naming
fn psm(f: String)  // What does this mean?
fn x() -> Number   // What is x?
```

```mortar
// Too many parameters
fn do_complex_thing(a: Number, b: Number, c: String, d: Bool, e: Number, f: String)
```

### Recommendations

1. **Self-explanatory names**: Function names should say what they do
2. **Moderate parameters**: Generally no more than 7 parameters
3. **Clear types**: All parameters and return values should have types
4. **Organize by category**: Put related functions together with comments

## Common Questions

### Q: Must I declare all functions I use?
Yes! Using undeclared functions will cause errors.

### Q: Can I write `function` instead of `fn`?
Yes! Both are identical:

```mortar
fn play_sound(file: String)
function play_sound(file: String)  // Same thing
```

### Q: Can I declare but not use?
Yes. Declared but unused functions will get compiler warnings, but not errors.

### Q: Can functions be overloaded?
No. Each function name can only be declared once.

```mortar
// ❌ Error: duplicate declaration
fn test(a: String)
fn test(a: Number, b: Number)
```

### Q: Can parameters have default values?
Not currently supported. All parameters are required.

## Next Steps

- See [Complete Examples](./5_1_basic-dialogue.md)
- Learn how to [Integrate with Your Game](./5_3_game-integration.md)
- Check out [JSON Output Format](./7_1_json-output.md)
