# Frequently Asked Questions

While using Mortar, you might have some questions. Here are the most common ones with answers.

## Basic Concepts

### How is Mortar different from other dialogue systems?

**The biggest difference is "separation of content and logic"**:

- **Traditional systems**: `"Hello<sound=hi.wav>, welcome<color=red>here</color>!"`
- **Mortar**: Text is pure text, events are written separately and linked by position

Benefits:
- Writers can focus on storytelling without technical markup
- Programmers can flexibly control events without breaking text
- Text content is easy to translate and modify

### Why use character positions to trigger events?

Character positions give you **precise control** over event timing:

```mortar
text: "Boom! A bolt of lightning streaks across the sky."
events: [
    0, shake_screen()      // At "B" for Boom, screen shakes
    5, flash_effect()      // At "!" for flash effect
    6, play_thunder()      // At "A" for A bolt, thunder sound
]
```

This is especially useful for:
- Typewriter effects (character-by-character display)
- Voice synchronization
- Sound effect coordination

### Can I write dialogues without events?

**Absolutely!** Events are optional:

```mortar
node SimpleDialogue {
    text: "Hello!"
    text: "Welcome!"
    
    choice: [
        "Thanks" -> Thanks,
        "Bye" -> return
    ]
}
```

This is perfectly valid and suitable for simple scenarios.

## Syntax

### Are semicolons and commas required?

**Mostly optional!** Mortar syntax is flexible:

```mortar
// All three work
text: "Hello"
text: "Hello",
text: "Hello";

events: [
    0, sound_a()
    1, sound_b()
]

events: [
    0, sound_a(),
    1, sound_b(),
]

events: [
    0, sound_a();
    1, sound_b();
]
```

But we recommend **staying consistent** with one style.

### Must strings use double quotes?

**Both single and double quotes work:**

```mortar
text: "Double quoted string"
text: 'Single quoted string'
```

### What's the difference between node and nd?

**They're identical!** `nd` is just shorthand for `node`:

```mortar
node OpeningScene { }
nd Opening { }      // Exactly the same
```

Similar shortcuts:
- `fn` = `function`
- `Bool` = `Boolean`

### How do I write comments?

Use `//` for single-line comments, `/* */` for multi-line:

```mortar
// Single line comment

/*
Multi-line
comment
*/

node Example {
    text: "Dialogue"  // Can also be at line end
}
```

## Nodes and Jumps

### What are the requirements for node names?

**Technically** you can use:
- English letters, numbers, underscores
- But cannot start with a number

**We strongly recommend using PascalCase**:

```mortar
// ✅ Recommended (PascalCase)
node OpeningScene { }
node ForestEntrance { }
node BossDialogue { }
node Chapter1Start { }

// ⚠️ Not recommended but works
node opening_scene { }  // snake_case is for functions
node forest_1 { }       // OK, but Forest1 is better

// ❌ Bad naming
node 开场 { }           // Avoid non-ASCII text
node 1node { }         // Cannot start with number
node node-1 { }        // Cannot use hyphens
```

**Why recommend PascalCase?**
- Consistent with mainstream programming language type naming
- Clear and readable, easy to recognize
- Avoids cross-platform encoding issues
- More standardized for team collaboration

### Can I jump to a non-existent node?

**No!** The compiler checks all jumps:

```mortar
node A {
    choice: [
        "Go to B" -> B,      // ✅ B exists, OK
        "Go to C" -> C       // ❌ C doesn't exist, error
    ]
}

node B { }
```

### How do I end dialogue?

Three ways:

1. **return** - End current node (if there is a subsequent node, it will continue)
2. **No subsequent jump** - Dialogue naturally ends
3. **Jump to special node** - Create a dedicated "ending" node

```mortar
// Method 1: Using return
node A {
    choice: [
        "End" -> return
    ]
}

// Method 2: Natural end
node B {
    text: "Goodbye!"
    // No jump, dialogue ends
}

// Method 3: Ending node
node C {
    choice: [
        "End" -> EndingNode
    ]
}

node EndingNode {
    text: "Thanks for playing!"
}
```

## Choice System

### Can choices be nested?

**Yes!** And to arbitrary depth:

```mortar
choice: [
    "What to eat?" -> [
        "Chinese" -> [
            "Rice" -> End1,
            "Noodles" -> End2
        ],
        "Western" -> [
            "Steak" -> End3,
            "Pasta" -> End4
        ]
    ]
]
```

### How do I write when conditions?

Two syntaxes:

```mortar
choice: [
    // Chain style
    ("Option A").when(has_key) -> A,
    
    // Function style
    "Option B" when has_key -> B
]
```

Condition functions must return `Bool`:

```mortar
fn has_key() -> Bool
```

### What if no choice conditions are met?

This is a **game logic** issue to handle. Mortar only compiles, doesn't manage runtime.

Suggestions:
- Keep at least one unconditional "default option"
- Check for available options in game code

## Event System

### Can event indices be decimals?

**Yes!** Decimals are especially suitable for voice sync:

```mortar
text: "This line has voice acting."
events: [
    0.0, start_voice()
    1.5, highlight_word()   // At 1.5 seconds
    3.2, another_effect()   // At 3.2 seconds
]
```

### Can multiple events be at the same position?

**Yes!** And they execute in order:

```mortar
events: [
    0, effect_a()
    0, effect_b()    // Also at position 0
    0, effect_c()    // Also at position 0
]
```

Game runtime will call these three functions in sequence.

### Must event functions be declared?

**Yes!** All used functions must be declared:

```mortar
node A {
    events: [
        0, my_function()   // Using function
    ]
}

// Must declare
fn my_function()
```

Not declaring will cause compilation error.

## Functions

### Are function declarations just placeholders?

**Yes!** Actual implementation is in your game code:

```mortar
// In Mortar file, just declare
fn play_sound(file: String)

// Real implementation in your game code (C#/C++/Rust etc.)
// For example in Unity:
// public void play_sound(string file) {
//     AudioSource.PlayClipAtPoint(file);
// }
```

Mortar is only responsible for:
- Checking function names are correct
- Checking parameter types match
- Generating JSON so game knows what to call

### What parameter types are supported?

Currently these basic types:

- `String` - String
- `Bool` / `Boolean` - Boolean (true/false)
- `Number` - Number (integer or decimal)

```mortar
fn example_func(
    name: String,
    age: Number,
    is_active: Bool
) -> String
```

### Can functions have no parameters?

**Yes!**

```mortar
fn simple_function()
fn another() -> String
```

### Can functions have multiple parameters?

**Yes!** Separate with commas:

```mortar
fn complex_function(
    param1: String,
    param2: Number,
    param3: Bool
) -> Bool
```

### What are function naming conventions?

**Strongly recommend using snake_case**:

```mortar
// ✅ Recommended (snake_case)
fn play_sound(file_name: String)
fn get_player_name() -> String
fn check_inventory() -> Bool
fn calculate_damage(base: Number, modifier: Number) -> Number

// ⚠️ Not recommended
fn playSound() { }          // camelCase is other languages' style
fn PlaySound() { }          // PascalCase is for nodes
fn 播放声音() { }           // Avoid non-ASCII text
```

**Parameter names should also use snake_case**:
```mortar
fn load_scene(scene_name: String, fade_time: Number)  // ✅
fn load_scene(SceneName: String, fadeTime: Number)    // ❌
```

## String Interpolation

### What is string interpolation?

Embedding variables or function calls in strings:

```mortar
text: $"Hello, {get_name()}! You have {get_score()} points."
```

Note the `$` before the string!

### Must interpolation be functions?

Currently Mortar interpolation mainly uses function calls. Content in interpolation is replaced with function return values.

### What happens without $?

Without `$` it's a plain string, `{}` is treated as regular characters:

```mortar
text: "Hello, {name}!"    // Displays "Hello, {name}!"
text: $"Hello, {name}!"   // name is replaced with actual value
```

## Compilation and Output

### What format is the compiled file?

**JSON format**, default is compressed (no spaces or line breaks):

```bash
mortar hello.mortar           # Generate compressed JSON
mortar hello.mortar --pretty  # Generate formatted JSON
```

### How do I specify output filename?

Use `-o` parameter:

```bash
mortar input.mortar -o output.json
```

Without specification, default is `input.mortared`

### What's the JSON structure?

Basic structure:

```json
{
  "nodes": {
    "NodeName": {
      "texts": [...],
      "events": [...],
      "choices": [...]
    }
  },
  "functions": [...]
}
```

See [JSON Output Format](./7_1_json-output.md) for details.

### How do I read compilation errors?

Mortar error messages are friendly, indicating:
- Error location (line, column)
- Error reason
- Related code snippet

```
Error: Undefined node 'Unknown'
  ┌─ hello.mortar:5:20
  │
5 │     choice: ["Go" -> Unknown]
  │                      ^^^^^^^ this node doesn't exist
  │
```

## Project Practice

### How to collaborate with multiple people?

Suggestions:
1. Use Git to manage Mortar files
2. Divide files by feature modules to reduce conflicts
3. Establish naming conventions
4. Write clear comments

### How to integrate with game engines?

Basic process:
1. Write Mortar files
2. Compile to JSON
3. Read JSON in game
4. Implement corresponding functions
5. Execute according to JSON instructions

See [Integrating with Your Game](./5_3_game-integration.md) for details.

### What types of games is this suitable for?

Especially suitable for:
- RPG dialogue systems
- Visual novels
- Text adventure games
- Interactive stories

Basically any game needing "structured dialogue"!

### Can it be used in non-game projects?

**Of course!** Any scenario needing structured text and events:
- Educational software
- Chatbots
- Interactive presentations
- Multimedia displays

## Advanced Topics

### Does it support variables?

Currently no built-in variable system, but you can:
- Maintain variables in game code
- Read/write variables through function calls

```mortar
// Mortar file
fn get_player_hp() -> Number
fn set_player_hp(hp: Number)

// Implement these functions in game code
```

### Does it support expressions?

Currently no complex expressions, but can implement through functions:

```mortar
// Not supported:
choice: [
    "Option" when hp > 50 && has_key -> Next
]

// Can do this:
choice: [
    "Option" when can_proceed() -> Next  
]

fn can_proceed() -> Bool  // Implement logic in game
```

This feature is coming soon.

### How to do localization (multiple languages)?

This feature is coming soon.

### Does it support modularity?

Currently each `.mortar` file is independent, cannot reference each other.

Suggestions:
- Write related dialogues in the same file
- Or load multiple JSON files in game and integrate

This feature is coming soon.

## Troubleshooting

### Getting "syntax error" during compilation?

1. Carefully check the location indicated by error message
2. Check for missing brackets, quotes
3. Check keyword spelling
4. Ensure node names and function names are valid

### "Undefined node" error?

Check:
- Does the jump target node exist
- Are node name cases consistent (case-sensitive!)
- Any typos

### "Type mismatch" error?

Check:
- Function declaration parameter types
- Do passed parameters match when calling
- Is return type correct

### Game can't read generated JSON?

1. Ensure JSON format is correct (check with `--pretty`)
2. Check game code parsing logic
3. Check for encoding issues (use UTF-8)

## Still Have Questions?

- 📖 Check [Example Code](./5_0_examples.md)
- 💬 Ask at [GitHub Discussions](https://github.com/Bli-AIk/mortar/discussions)
- 🐛 Report bugs at [GitHub Issues](https://github.com/Bli-AIk/mortar/issues)
- 📚 Read [Contributing Guide](./7_3_contributing.md)

We're happy to help! 🎉