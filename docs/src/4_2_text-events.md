# Text and Events: The Art of Separation

This is what makes Mortar unique: **text and events are written separately but precisely associated**.

## Why Separate?

Imagine you're writing game dialogue with events, the traditional way might be:

```
"Hello<sound=hi.wav>, welcome<anim=wave> to<color=red>here</color>!"
```

Problems arise:

- 😰 Writers see a bunch of "markup", hard to focus on the text itself
- 😰 Programmers need to parse complex markup, error-prone
- 😰 Adding or removing event parameters is quite cumbersome

Mortar's approach:

```mortar
text: "Hello, welcome to here!"
with events: [
    0, play_sound("hi.wav")
    7, show_animation("wave")
    18, set_color("red")
    22, set_color("normal")
]
```

Clean! Clear! Maintainable!

## Text Block Basics

### Simplest Text

```mortar
node Example {
    text: "This is a text segment."
}
```

### Multiple Text Segments

```mortar
node Dialogue {
    text: "First sentence."
    text: "Second sentence."
    text: "Third sentence."
}
```

They will display in order.

### Using Quotes

Both single and double quotes work:

```mortar
text: "Double quotes"
text: 'Single quotes'
```

### Escape Sequences

Mortar supports standard escape sequences within strings:

| Escape | Character       |
|--------|-----------------|
| `\n`   | Newline         |
| `\t`   | Tab             |
| `\r`   | Carriage return |
| `\\`   | Backslash       |
| `\"`   | Double quote    |
| `\'`   | Single quote    |
| `\0`   | Null character  |

**Examples:**

```mortar
node Dialogue {
    text: "Line 1\nLine 2"           // Two lines
    text: "Name:\tAlice"             // With tab
    text: "She said \"Hello!\""      // With quotes
    text: "Path: C:\\Users\\Alice"   // With backslashes
}
```

## Event System

### Basic Syntax

```mortar
with events: [
    index, function_call
    index, function_call
]
```

`with events` attaches the event list to the most recent `text` statement. Indices start from 0, type is Number, and
support integers or decimals. Your engine decides how to interpret these indices (typewriter steps, timeline positions,
etc.).

### Simple Example

Using character indices as an example:

```mortar
text: "Hello world!"
with events: [
    0, sound_a()  // At "H"
    6, sound_b()  // At "w"
    11, sound_c()  // At "!"
]
```

**Character indices**:

- "H" = position 0
- "e" = position 1
- "l" = position 2
- "l" = position 3
- "o" = position 4
- " " = position 5
- "w" = position 6
- ...

### Method Chaining

You can call multiple functions at the same position:

```mortar
with events: [
    0, play_sound("boom.wav").shake_screen().flash_white()
]
```

Or write them separately:

```mortar
with events: [
    0, play_sound("boom.wav")
    0, shake_screen()
    0, flash_white()
]
```

Both ways have the same effect.

## Decimal Indices

Indices can be decimals, which is especially useful for voice synchronization:

```mortar
text: "Hello, world!"
with events: [
    0.0, start_voice("hello.wav")   // Start playing voice
    1.5, blink_eyes()               // Blink at 1.5 seconds
    3.2, show_smile()               // Smile at 3.2 seconds
    5.0, stop_voice()               // End at 5 seconds
]
```

**When to use decimals?**

Our recommendation:

- Typewriter effect: use integers (one trigger per character)
- Voice sync: use decimals (trigger by timeline)
- Video sync: use decimals (precise to frames)

## String Interpolation

Want to insert variables or function return values into text? Use `$` and `{}`:

```mortar
text: $"Hello, {get_player_name()}!"
text: $"You have {get_gold()} gold."
text: $"Today is {get_date()}."
```

**Note**:

- Add `$` before the string to declare it as an "interpolated string"
- Put variables/functions inside `{}`
- Functions must be declared in advance

## Practical Examples

### Typewriter Effect with Sound

```mortar
node Typewriter {
    text: "Ding! Ding! Ding!"
    with events: [
        0, play_sound("ding.wav")  // First "Ding"
        6, play_sound("ding.wav")  // Second "Ding"
        12, play_sound("ding.wav")  // Third "Ding"
    ]
}
```

### Narration with Background Music

```mortar
node Narration {
    text: "In a distant kingdom..."
    with events: [
        0, fade_in_bgm("story_theme.mp3")
        0, dim_lights()
    ]
    
    text: "There lived a brave knight."
}
```

### Voice Synchronized Animation

```mortar
node Dialogue {
    text: "I'll tell you a secret..."
    with events: [
        0.0, play_voice("secret.wav")
        0.0, set_expression("serious")
        2.5, lean_closer()
        4.0, whisper_effect()
        6.0, set_expression("normal")
    ]
}
```

## Event Function Declarations

All functions used must be declared first:

```mortar
// Declare at end of file
fn play_sound(file: String)
fn shake_screen()
fn flash_white()
fn set_expression(expr: String)
fn get_player_name() -> String
fn get_gold() -> Number
```

See [Functions: Connecting to Game World](./4_4_functions.md) for details.

## Best Practices

### ✅ Good Practices

```mortar
// Clear structure
text: "Hello, world!"
with events: [
    0, greeting_sound()
    7, sparkle_effect()
]
```

### ❌ Bad Practices

```mortar
text: "Hello"
text: "world"
with events: [
    0, say_hello()  // Associated text is wrong!
]
```

### Recommendations

1. **Follow-up principle**: events immediately follow the corresponding text
2. **Use moderately**: not every sentence needs events
3. **Ordered arrangement**: write events in ascending order by position (though not mandatory)
4. **Meaningful naming**: function names should be self-explanatory

## Common Questions

### Q: What happens if position exceeds text length?

Compiler will warn, but won't error. Runtime behavior depends on your game.

### Q: Can there be no events?

Of course! Not every text segment needs events. But events must be attached to text.

```mortar
text: "This is pure text."
// No events, completely fine
```

### Q: Execution order of multiple events at same position?

Executes in written order:

```mortar
with events: [
    0, first()   // Executes first
    0, second()  // Then this
    0, third()   // Finally this
]
```

## Next Steps

- Learn about [Choice System](./4_3_choices.md)
- Study [Function Declarations](./4_4_functions.md)
- See [Complete Examples](./5_1_basic-dialogue.md)
