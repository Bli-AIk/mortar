# Understanding Mortar Language

Now that you've learned the basics, let's dive deeper into Mortar's core philosophy.

## Three Key Components of Mortar

Writing a Mortar script mainly involves handling these things:

### 1. Nodes
Think of nodes as "scenes" or "segments" in a dialogue. Each node can contain:
- Multiple text segments
- Associated events
- Player choices
- Next node

### 2. Text & Events
This is Mortar's core feature:
- **Text**: Pure dialogue content, without any rich text or technical markup
- **Events**: Actions triggered at specific character positions (sound effects, animations, etc.)
- They are written separately but linked through indices.

### 3. Choices
The key to player participation:
- List multiple options
- Each option can jump to a different node
- Can set conditions (e.g., must have certain items to display)

## Mortar's Design Philosophy

### Separation of Concerns

Traditional dialogue systems might look like this:
```
"Hello<sound=greeting.wav>, welcome<anim=wave> here!"
```

Looks messy, right? Writers need to remember various markup, and programmers find it hard to maintain.

Mortar's approach:
```mortar
text: "Hello, welcome here!"
with events: [
    0, play_sound("greeting.wav")
    7, show_animation("wave")
]
```

Text is text, events are events. Clear and simple!

### Position as Time

Mortar uses "character position" to control when events occur:

```mortar
text: "Hello world!"
with events: [
    0, sound_a()  // Triggers at "H"
    6, sound_b()  // Triggers at "w"
    11, sound_c()  // Triggers at "!"
]
```

This position can be:
- **Integer**: Suitable for typewriter effects (displaying one character at a time)
- **Decimal**: Suitable for voice synchronization (e.g., at 2.5 seconds into a line)

### Declarative Syntax

You only need to describe "what to do", not "how to do it":

```mortar
choice: [
    "Option A" -> NodeA,
    "Option B" when has_key() -> NodeB
]
```
The code implementation of `has_key()` is entirely done by the programming team.

## Data Flow: From Mortar to Game

Let's look at the complete process:

```
Write Mortar    You write dialogues in Mortar language
   │
   ▼
Compile to JSON    mortar command compiles it to JSON
   │
   ▼
Game Reads and Executes    Your game engine reads the JSON and executes accordingly
```

## Detailed Component Explanations

Want to learn more about each part?

- [Nodes: Building Blocks of Dialogue](./4_1_nodes.md) - All node usage
- [Text and Events: The Art of Separation](./4_2_text-events.md) - How to elegantly associate text and events
- [Choices: Let Players Decide](./4_3_choices.md) - Creating branching dialogues
- [Functions: Connecting to Game World](./4_4_functions.md) - Declaring and using functions
- [Variables and Constants](./4_5_variables.md) - Track state and expose key-value strings
- [Branch Interpolation](./4_6_branch-interpolation.md) - Build Fluent-style snippets with per-branch events
- [Localization Strategy](./4_7_localization.md) - Structure repositories for multilingual builds
- [Control Flow in Nodes](./4_8_control-flow.md) - Use `if/else` to gate dialogue
- [Event System and Timelines](./4_9_event-system.md) - Reuse named cues and cinematic playlists
- [Enums and Structured Choices](./4_10_enums.md) - Model discrete states for cleaner branching

## Tips

- **Start Simple**: Write pure text dialogues first, then gradually add events and choices
- **Use Comments**: Use `//` to leave notes for yourself
- **Name Wisely**: "Node names" and function names should be self-explanatory
- **Keep it Clean**: Mortar's advantage is cleanliness, don't make logic too complex

Ready to dive deeper? Start with [Nodes](./4_1_nodes.md)!
