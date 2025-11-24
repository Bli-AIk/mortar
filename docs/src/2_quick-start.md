# Quick Start Guide

Let's write your first Mortar dialogue! Don't worry, it's simpler than you think.

## Step 1: Create a File

Create a file named `hello.mortar` (use any text editor, or you can first read [Editor Support](./6_2_ide-support.md) to learn how to configure your editor).

## Step 2: Write Simple Dialogue

Write this in the file:

```mortar
// This is a comment, this line will be ignored by the compiler, no worries!
// I'll explain the mortar code in comments.

node StartScene {
    text: "Hello, welcome to this interactive story!"
}
```

That's it! You've written your first dialogue node.

**Explanation**:
- `node` declares a dialogue node (can also be shortened to `nd`)
- `StartScene` is the name of this node (using PascalCase naming)
- `text:` is followed by the dialogue content
- Don't forget the curly braces `{}`, they wrap the node's content

> **💡 Naming Convention Tips**:
> - "Node names" use **PascalCase**, like `StartScene`, `ForestPath`
> - Function names use **snake_case**, like `play_sound`, `get_player_name`
> - We recommend using only English, numbers, and underscores for identifiers

## Step 3: Add Some Sound Effects

Now let's make our dialogue more lively. Assume you've already discussed with the programming team—we want each sentence to print slowly like a typewriter:

```mortar
node StartScene {
    text: "Hello, welcome to this interactive story!"
    with events: [
        // Play a sound effect when the 'H' character appears
        0, play_sound("greeting.wav")
        // Show an animation when the 'w' of "welcome" appears
        7, show_animation("wave")
    ]
}

// Tell Mortar what functions are available
// The programming team needs to bind these functions in the project
fn play_sound(file_name: String)
fn show_animation(anim_name: String)
```

**Explanation**:

* `with events:` binds the event list to the text line right above it, wrapped in square brackets `[]`
* `0, play_sound("greeting.wav")` means: at index 0 (which corresponds to the character 'H' if using a typewriter effect), play the sound.
* The numbers are "indices," representing character positions starting from 0. Indices can be decimals (floating-point numbers).
* These indices are flexible. Some games might use a typewriter effect, while others might align with voice acting. How you count them depends entirely on your project's needs.
* The `fn` declarations at the bottom tell the compiler what parameters these functions require.
* It's recommended to use snake_case for function parameter names: `file_name`, `anim_name`

## Step 4: Add Multiple Dialogue Segments

A node can have several text segments:

```mortar
node StartScene {
    text: "Hello, welcome to this interactive story!"
    // ↕ This event list binds to the text above it
    with events: [
        0, play_sound("greeting.wav")
    ]
    
    // Second text segment
    text: "I think your name is Ferris, right?"
    
    // Third text segment
    text: "Let's get started then!"
}
```

These three text segments will display in sequence. The first text has events, while the latter two don't.

## Step 5: Let Players Make Choices

Now let's involve the player:

```mortar
node StartScene {
    text: "What would you like to do?"
    
    choice: [
        "Explore the forest" -> ForestScene,
        "Return to town" -> TownScene,
        "I'm done playing" -> return
    ]
}

node ForestScene {
    text: "You bravely ventured into the forest..."
}

node TownScene {
    text: "You returned to the warm town."
}
```

**Explanation**:
- `choice:` indicates options are here
- `"Explore the forest" -> ForestScene` means: display the "Explore the forest" option, if chosen jump to the node named `ForestScene`
- The benefit of using PascalCase for nodes shows here—easy to recognize and maintain!
- `return` indicates ending the current dialogue

## Step 6: Compile the File

Open command line (terminal/CMD), and enter:

```bash
mortar hello.mortar
```

This will generate a `hello.mortared` file containing JSON format data that your game can read.

**Seeing "command not found"?** That means you haven't installed the mortar compiler yet! Please read [Installation](./3_installation.md) to install it.

**JSON compressed to one line?** Add the `--pretty` parameter:

```bash
mortar hello.mortar --pretty
```

**Want to customize output filename and extension?** Use the `-o` parameter:

```bash
mortar hello.mortar -o my_dialogue.json
```

## Complete Example

Let's combine what we just learned and "add some details":

```mortar
node WelcomeScene {
    text: "Hello, welcome to the Magic Academy!"
    with events: [
        0, play_sound("magic_sound.wav")
        7, sparkle_effect()
    ]
    
    text: $"Your name is {get_player_name()}, right?"
    
    text: "Ready to start your adventure?"
    
    choice: [
        "Of course!" -> AdventureStart,
        "Let me think about it..." -> Hesitate,
        // Conditional option (only shows if has backpack)
        "Check backpack first" when has_backpack() -> CheckInventory
    ]
}

node AdventureStart {
    text: "Great! Let's go!"
}

node Hesitate {
    text: "No problem, take your time~"
}

node CheckInventory {
    text: "Your backpack has some basic items."
}

// Function declarations
fn play_sound(file: String)
fn sparkle_effect()
fn get_player_name() -> String
fn has_backpack() -> Bool
```

**New features explained**:
- `$"Your name is {get_player_name()}, right?"` This is called string interpolation, content in `{}` will be replaced with the function's return value
- `when` indicates this option is conditional, only "displayed" when `has_backpack()` returns true
- `-> String` and `-> Bool` indicate the function's return type. Mortar performs static type checking to prevent type mixing!

## What to Learn Next?

- Want to understand deeper? Check out [Core Concepts](./4_0_core-concepts.md)
- Want to see more examples? Go to [Practical Examples](./5_0_examples.md)
- Want to know all features? Browse [Functions](./4_4_functions.md) and [Choices](./4_3_choices.md)

Congratulations! You've learned the basics of Mortar 🎉
