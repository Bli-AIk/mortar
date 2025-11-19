# Welcome to Mortar 🦀

Hello and welcome to the world of Mortar!

## What is Mortar?

Imagine you're writing dialogue scripts for a game. You might encounter these frustrations:

- Text cluttered with various technical markup, making it messy
- Wanting to play sound effects when certain characters appear, but not knowing how to annotate
- Writers and programmers constantly "stepping on each other's toes"

**Mortar was created to solve these problems.**

It's a language specifically designed for writing game dialogue, with the core principle of achieving **strict separation between text content and event logic**.

In essence, our philosophy is simple:

> **Let text be text, let code be code.**

You can focus on writing stories, while the program focuses on handling game logic—the two don't interfere with each other, yet work together perfectly.

## What Can It Do?

Mortar is particularly suitable for these scenarios:

- 🎮 **Game Dialogue Systems**: RPG dialogues, visual novels
- 📖 **Interactive Fiction**: Text adventures, branching narratives
- 📚 **Educational Content**: Interactive tutorials, guided learning scenarios
- 🤖 **Chat Scripts**: Structured dialogue logic
- 🖼️ **Multimedia Presentation**: Synchronization of text and media events

## Why Choose Mortar?

Compared to other dialogue systems, Mortar has these features:

- **Clean and Clear**: No messy markup in the text
- **Precise Control**: Can specify triggering events at specific character positions (like playing sound effects)
- **Easy to Understand**: Syntax designed to feel as natural as everyday writing
- **Easy Integration**: Compiles to JSON format, usable by any game engine

## Quick Glance

This is what Mortar code looks like:

```mortar
node OpeningScene {
    text: "Welcome to the magical world!"
    events: [
        0, play_sound("magic_sound.wav")
        7, sparkle_effect()
    ]
    
    text: "Ready to start your adventure?"
    
    choice: [
        "Yes, I'm ready!" -> AdventureStart,
        "Let me think about it..." -> Hesitate
    ]
}
```

Looks intuitive, doesn't it?

## Next Steps

- Want to try it right away? Check out [Quick Start Guide](./2_quick-start.md)
- Need to install tools? Go to [Installation](./3_installation.md)
- Want to learn more? Start with [Core Concepts](./4_0_core-concepts.md)

---

*Mortar is an open-source project, licensed under MIT/Apache-2.0 dual license ❤️*
