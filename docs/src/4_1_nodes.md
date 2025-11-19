# Nodes: Building Blocks of Dialogue

Nodes are the most basic units in Mortar. Think of them as a "scene" or "segment" in a dialogue.

## The Simplest Node

```mortar
node OpeningScene {
    text: "Hello, world!"
}
```

That's it! A node needs:
- `node` keyword (can also be shortened to `nd`)
- A name (here it's `OpeningScene`)
- Content inside curly braces `{}`

## Node Naming Conventions

> **⚠️ Important: We recommend using PascalCase**

**✅ Recommended naming style**:
```mortar
node OpeningScene { }       // PascalCase: first letter of each word capitalized
node ForestEntrance { }     // Clear and readable
node BossDialogue { }       // Self-explanatory
node Chapter1Start { }      // Can include numbers
```

**⚠️ Not recommended naming styles**:
```mortar
node 开场 { }              // Non-ASCII text not recommended
node opening_scene { }    // Don't use snake_case (that's for functions)
node openingscene { }     // All lowercase is hard to read
node opening-scene { }    // Kebab-case not recommended
node 1stScene { }         // Don't start with numbers
```

**We recommend the following naming conventions**:
- Use English word combinations
- Capitalize the first letter of each word
- Names should be meaningful, describing the node's purpose
- Avoid special characters and non-ASCII characters
- Keep the naming style consistent within the project

**The reasons for this are simple**:
* **Easier for team maintenance**: Using a unified naming convention makes it easier for team members to understand what each node does.
* **Fewer cross-platform issues**: Some special characters or non-ASCII text may display differently across operating systems and editors; using English words avoids these problems.
* **Aligns with common programming practices**: Most programming languages and open-source projects use this naming convention, making learning and communication smoother.
* **Better for code navigation**: Standard names make it easier for editors/IDEs to find related nodes, improving work efficiency.

## What Can Go Inside a Node?

A node can contain:

### 1. Text Blocks

```mortar
node Dialogue {
    text: "This is the first sentence."
    text: "This is the second sentence."
    text: "And a third one."
}
```

Multiple text segments will display in order.

### 2. Event Lists

```mortar
node Dialogue {
    text: "Hello!"
    events: [
        0, play_sound("hi.wav")
        5, show_smile()
    ]
}
```
The event list is associated with the text above it.

### 3. Choices

```mortar
node Choice {
    text: "Where do you want to go?"
    
    choice: [
        "Forest" -> ForestScene,
        "Town" -> TownScene
    ]
}
```

### 4. Mixed Usage

```mortar
node CompleteExample {
    // First text + events
    text: "Welcome to the Magic Academy!"
    events: [
        0, play_bgm("magic.mp3")
        11, sparkle()
    ]
    
    // Second text
    text: "Are you ready?"
    
    // Let player make a choice
    choice: [
        "Ready!" -> StartAdventure,
        "Wait..." -> Wait
    ]
}
```

## Node Jumping

### Method 1: Arrow Jumps

Use `->` after a node ends to specify the next node:

```mortar
node A {
    text: "This is node A"
} -> B  // After A completes, jump to B

node B {
    text: "This is node B"
}
```

### Method 2: Jump Through Choices

```mortar
node MainMenu {
    text: "Choose an option:"
    
    choice: [
        "Option 1" -> Node1,
        "Option 2" -> Node2
    ]
}
```

### Method 3: Return to End Node

```mortar
node End {
    text: "Goodbye!"
    
    choice: [
        "Exit" -> return  // End current node
    ]
}
```

Please note: `return` inside a node only ends the current node's execution. If the node has an arrow jump, that jump will still be executed after the node finishes.

```mortar
node A {
    text: "This is node A"
    
    choice: [
        "End current node" -> return  // This only ends the execution of node A's content
    ]
} -> B  // The return does not prevent this jump to B

node B {
    text: "This is node B"
}
```

**Explanation**:

*   `return`: Finishes the execution of the current node. It does not automatically jump to another node.
*   `-> B` outside the node: After node A has finished executing (even via a `return`), it will still jump to node B.

## Node Execution Flow

Let's look at an example:

```mortar
node Scene1 {
    text: "First sentence"    // Display this first
    text: "Second sentence"   // Then display this
    
    choice: [                // Then show choices
        "A" -> Scene2,
        "B" -> Scene3,
        "C" -> break
    ]
    
    text: "After choice"     // 4. Only reach here if chose break option
} -> Scene4                   // 5. If not interrupted, finally jump here
```

**Key points**:
- Text blocks execute in order
- When encountering `choice`, player needs to make a decision
- If choice has `return` or `break`, it affects subsequent flow
- Arrow at node end is the "default exit"

For the break keyword, see [Choices: Let Players Decide](./4_3_choices.md).

## Common Usage Patterns

### Pure Text Node (No Choices)

```mortar
node Start {
    text: "The story begins on a dark night..."
    text: "Suddenly, a loud bang!"
    text: "You decide to check it out."
} -> NextScene
```

### Pure Choice Node (No Text)

```mortar
node ChoiceExample {
    choice: [
        "Attack" -> Attack,
        "Flee" -> Escape
    ]
}
```

### Segmented Dialogue

```mortar
node Dialogue {
    text: "Hi, nice to meet you."
    
    // First choice point
    choice: [
        "Hello" -> break,
        "Goodbye" -> return
    ]
    
    text: "So..."  // Only see this if chose "Hello"
    text: "Let's chat."
}
```

## Common Questions

### Q: Can "node names" be duplicated?
No! Each node name must be unique.

### Q: Does node order matter?
No. You can define node B first, then node A, as long as jump relationships are correct.

### Q: Can a node be empty?
Technically yes, but it's meaningless:
```mortar
node EmptyNode {
}  // Compiler will warn you
```

### Q: Can you jump from node A back to node A?
Yes! Loops are allowed:
```mortar
node Cycle {
    text: "Want to go again?"
    
    choice: [
        "Again!" -> Cycle,  // Jump back to itself
        "No thanks" -> return
    ]
}
```

## Next Steps

- Learn how to use [Text and Events](./4_2_text-events.md) in nodes
- Learn more about [Choice System](./4_3_choices.md) usage
- Check out [Complete Examples](./5_1_basic-dialogue.md)
