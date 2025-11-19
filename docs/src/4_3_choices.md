# Choices: Let Players Decide

Choices are the key to making dialogues interactive! Players can choose different paths, leading to different endings.

## Simplest Choice

```mortar
node ChoiceExample {
    text: "Where do you want to go?"
    
    choice: [
        "Forest" -> ForestScene,
        "Town" -> TownScene
    ]
}

node ForestScene {
    text: "You arrived at the forest."
}

node TownScene {
    text: "You arrived at the town."
}
```

The syntax is simple:
- `choice:` keyword
- Inside square brackets `[]` is the list of options
- Each option: `"text" -> target node`

## Various Ways to Write Choices

### Basic Jump

```mortar
choice: [
    "Option A" -> NodeA,
    "Option B" -> NodeB,
    "Option C" -> NodeC
]
```

### Conditional Choices

Only displayed when condition is met:

```mortar
choice: [
    "Normal option" -> Node1,
    "Show only with key" when has_key() -> Node2,
    "Show only level>=10" when is_level_enough() -> Node3
]
```

**Two ways to write conditions**:

```mortar
// Function style: when separated by space

"Option text" when condition_function() -> target

// Chain style: wrapped in parentheses

("Option text").when(condition_function()) -> target
```

### Special Behaviors

#### Return - End Node

```mortar
choice: [
    "Continue" -> NextNode,
    "Exit" -> return  // Directly end node
]
```

#### Break - Interrupt Choice

```mortar
choice: [
    "Option 1" -> Node1,
    "Option 2" -> Node2,
    "Never mind" -> break  // Exit choice, continue current node
]

text: "Okay, let's continue."  // Only reach here if chose break
```

### Nested Choices

Choices can contain more choices!

```mortar
choice: [
    "Eat something" -> [
        "Apple" -> EatApple,
        "Bread" -> EatBread,
        "Never mind" -> return
    ],
    "Do something" -> [
        "Rest" -> Rest,
        "Explore" -> Explore
    ],
    "Leave" -> return
]
```

Player first chooses "Eat something", then sees the second layer of options.

## Practical Examples

### Simple Branch

```mortar
node ChatWithNPC {
    text: "A merchant approaches you."
    text: "He says: 'Need to buy anything?'"
    
    choice: [
        "Browse goods" -> EnterShop,
        "Ask for news" -> AskSth,
        "Politely decline" -> SayNO
    ]
}
```

### Complex Choice with Conditions

```mortar
node Door {
    text: "You face a mysterious door."
    
    choice: [
        "Push directly" -> PushDoor,
        "Open with key" when has_key() -> OpenDoorWithKey,
        "Use magic" when can_use_magic() -> OpenDoorWithMagic,
        "Leave" -> return
    ]
}
```

### Multi-level Nesting

```mortar
node Restaurant {
    text: "Welcome! What would you like to eat?"
    
    choice: [
        "Chinese" -> [
            "Fried rice" -> End1,
            "Noodles" -> End2,
            "Go back" -> break
        ],
        "Western" -> [
            "Steak" -> End3,
            "Pasta" -> End4,
            "Go back" -> break
        ],
        "Nothing" -> return
    ],
    
    text: "Think about it then~"  // Reach here if chose break
}
```

### Clever Use of Break

```mortar
node ImportantChoice {
    text: "This is an important decision."
    text: "Are you sure?"
    
    choice: [
        "Yes!" -> ConfirmRoute,
        "Let me think..." -> break  // Exit choice
    ],
    
    text: "Okay, take your time."
    
    // Can have another choice
    choice: [
        "Now I'm sure" -> ConfirmRoute,
        "Forget it" -> return
    ]
}
```

## Return vs Break

Easy to confuse, let's clarify:

| Keyword | Effect | Subsequent Execution |
|---------|--------|---------------------|
| `return` | End current node | Won't execute following content |
| `break` | Exit current choice | Will continue executing following content |

### Return Example

```mortar
node Test {
    text: "Start"
    
    choice: [
        "Exit" -> return
    ]
    
    text: "This won't execute"  // Because returned above
}
```

### Break Example

```mortar
node Test {
    text: "Start"
    
    choice: [
        "Break" -> break
    ]
    
    text: "This will execute"  // Continue after break
}
```

## Condition Functions

All functions used after `when` must:
- Return `Bool` (or `Boolean`) type
- Be declared in advance

```mortar
// Declare these functions in the file
fn has_key() -> Bool
fn can_use_magic() -> Boolean
fn is_level_enough() -> Bool
```

See [Functions: Connecting to Game World](./4_4_functions.md) for details.

## Best Practices

### ✅ Good Practices

```mortar
// Clear option text
choice: [
    "Greet friendly" -> Friendly,
    "Stay alert" -> Alert,
    "Turn and leave" -> Leave
]
```

```mortar
// Reasonable use of conditions
choice: [
    "Normal option" -> Node1,
    "Special option" when special_unlock() -> Node2
]
```

### ❌ Bad Practices

```mortar
// Option text too long
choice: [
    "I think we should first go to the forest and look around, then decide what to do next..." -> Node1
]
```

```mortar
// All options have conditions (might all not display!)
choice: [
    "Option 1" when cond1() -> A,
    "Option 2" when cond2() -> B,
    "Option 3" when cond3() -> C
]
```

### Recommendations

1. **At least one unconditional option**: Ensure players always have a choice
2. **Keep option text concise**: Generally no more than 20 characters
3. **Clear logic**: Don't nest too deep (suggest max 2-3 levels)
4. **Provide fallback**: Give players "return" or "cancel" options

## Common Questions

### Q: Can choices jump to themselves?
Yes! This creates loops:

```mortar
node Cycle {
    text: "Continue?"
    
    choice: [
        "Yes" -> Cycle,  // Jump back to itself
        "No" -> return
    ]
}
```

### Q: What if all options have conditions but none are met?
Choice condition checking essentially **marks options**.

Simply put:
* When condition is met, option availability is marked as "selectable".
* When condition is not met, option availability is marked as "not selectable".
* **Specific display effects** (grayed out, hidden, tooltip text, etc.) are determined by program implementation, not directly controlled by mortar.

> ⚠️ If all option conditions are not met, it's recommended to keep at least one unconditional option in the DSL to avoid "no options available" situations.

### Q: How deep can nested choices go?
Technically no limit, but suggest no more than 3 levels, otherwise everyone gets confused.

### Q: Can I use interpolation in option text?
Yes. Any string can use string interpolation.

## Next Steps

- Learn [Function Declarations](./4_4_functions.md)
- See [Complete Interactive Story Example](./5_2_interactive-story.md)
- Learn how to [Integrate with Your Game](./5_3_game-integration.md)
