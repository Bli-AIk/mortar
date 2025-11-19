# Writing Simple Dialogue

Let's start with the simplest scenario: a brief conversation between an NPC and the player.

## Scene Setup

Imagine you're making an RPG game with a villager NPC who greets the player and asks if they need help.

## Version 1: Pure Text Dialogue

The simplest version, just write out the dialogue:

```mortar
// Villager's greeting
node VillagerGreeting {
    text: "Hello there, adventurer!"
    text: "Welcome to our little village."
    text: "Do you need any help?"
    
    choice: [
        "I need help" -> OfferHelp,
        "No thanks" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "Great! Let me see what I can do for you..."
    text: "Here's a map, hope it helps!"
}

node PoliteFarewell {
    text: "Alright, have a pleasant journey!"
}
```

**Expected result**:
1. Display three text segments
2. Player makes a choice
3. Jump to different nodes based on choice

## Version 2: Adding Sound Effects

Now let's make the dialogue more lively by adding sound effects:

```mortar
node VillagerGreeting {
    text: "Hello there, adventurer!"
    events: [
        // Play greeting sound when "H" appears
        0, play_sound("greeting.wav")
    ]
    
    text: "Welcome to our little village."
    events: [
        // Play warm music at "little village"
        15, play_music("village_theme.ogg")
    ]
    
    text: "Do you need any help?"
    
    choice: [
        "I need help" -> OfferHelp,
        "No thanks" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "Great! Let me see what I can do for you..."
    
    text: "Here's a map, hope it helps!"
    events: [
        // Item get sound effect
        0, play_sound("item_get.wav"),
        // Show item icon at the same time
        0, show_item_icon("map")
    ]
}

node PoliteFarewell {
    text: "Alright, have a pleasant journey!"
    events: [
        0, play_sound("farewell.wav")
    ]
}

// Function declarations
fn play_sound(file_name: String)
fn play_music(filename: String)
fn show_item_icon(item_name: String)
```

**New additions**:
- Each dialogue segment has appropriate sound effects
- Special effects when getting items
- All used functions are declared

## Version 3: Dynamic Content

Make dialogue more personalized by greeting with player's name:

```mortar
node VillagerGreeting {
    // Use string interpolation to dynamically insert player name
    text: $"Hello there, {get_player_name()}!"
    events: [
        0, play_sound("greeting.wav")
    ]
    
    text: "Welcome to our little village."
    events: [
        15, play_music("village_theme.ogg")
    ]
    
    text: "Do you need any help?"
    
    choice: [
        "I need help" -> OfferHelp,
        "No thanks" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "Great! Let me see what I can do for you..."
    
    text: "Here's a map, hope it helps!"
    events: [
        0, play_sound("item_get.wav"),
        0, show_item_icon("map")
    ]
    
    text: $"Good luck, {get_player_name()}!"
}

node PoliteFarewell {
    text: "Alright, have a pleasant journey!"
    events: [
        0, play_sound("farewell.wav")
    ]
}

// Function declarations
fn play_sound(file_name: String)
fn play_music(filename: String)
fn show_item_icon(item_name: String)
fn get_player_name() -> String  // Returns player name
```

**New additions**:
- String interpolation using `$"..."` syntax
- `{get_player_name()}` gets replaced with actual player name
- More personalized feel

## Version 4: Conditional Options

Some players might already have a map, let's add conditional logic:

```mortar
node VillagerGreeting {
    text: $"Hello there, {get_player_name()}!"
    events: [
        0, play_sound("greeting.wav")
    ]
    
    text: "Welcome to our little village."
    events: [
        15, play_music("village_theme.ogg")
    ]
    
    text: "Do you need any help?"
    
    choice: [
        // Only show this option when player needs map
        "I need help" when need_map() -> OfferHelp,
        
        // Players with map see this
        "I already have a map" when has_map() -> AlreadyHasMap,
        
        // This option always shows
        "No thanks" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "Great! Let me see what I can do for you..."
    text: "Here's a map, hope it helps!"
    events: [
        0, play_sound("item_get.wav"),
        0, show_item_icon("map")
    ]
    text: $"Good luck, {get_player_name()}!"
}

node AlreadyHasMap {
    text: "Oh, looks like you're well prepared!"
    text: "Then have a safe journey!"
}

node PoliteFarewell {
    text: "Alright, have a pleasant journey!"
    events: [
        0, play_sound("farewell.wav")
    ]
}

// Function declarations
fn play_sound(file_name: String)
fn play_music(filename: String)
fn show_item_icon(item_name: String)
fn get_player_name() -> String
fn need_map() -> Bool      // Check if needs map
fn has_map() -> Bool       // Check if already has map
```

**New additions**:
- Options with `when` conditions
- Different options based on player state
- More realistic game logic

## Compiling and Using

Save as `village_npc.mortar`, then compile:

```bash
# Compile to JSON
mortar village_npc.mortar

# For formatted JSON
mortar village_npc.mortar --pretty

# Specify output filename
mortar village_npc.mortar -o npc_dialogue.json
```

## Implementing in Game

Your game needs to:

1. **Read JSON**: Parse the compiled JSON file
2. **Implement functions**: Implement all declared functions
   ```csharp
   // For example in Unity C#
   void play_sound(string filename) {
       AudioSource.PlayOneShot(Resources.Load<AudioClip>(filename));
   }
   
   string get_player_name() {
       return PlayerData.name;
   }
   
   bool has_map() {
       return PlayerInventory.HasItem("map");
   }
   ```

3. **Execute dialogue**: Display text, trigger events, handle choices according to JSON

## Summary

This example demonstrates:
- ✅ Basic nodes and text
- ✅ Event binding
- ✅ Choice jumping
- ✅ String interpolation
- ✅ Conditional logic
- ✅ Function declarations

From this simple example, you can create more complex dialogue systems!

## Next Steps

- Want to learn more complex branching narratives? See [Creating Interactive Stories](./5_2_interactive-story.md)
- Want to know specific integration methods? See [Integrating with Your Game](./5_3_game-integration.md)
- Want to understand syntax deeply? See [Core Concepts](../4_0_core-concepts.md)
