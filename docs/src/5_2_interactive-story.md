# Creating Interactive Stories

Now let's create a complete interactive short story with multiple branches and endings.

## Story Outline

**"The Mysterious Forest"** - An adventurer's tale:
- Player encounters a mysterious magical spring in the forest
- Different choices lead to different endings
- Features conditional logic and multi-layered nested choices

## Complete Code

```mortar
// ========== Opening Scene ==========
node OpeningScene {
    text: "Night falls as you walk alone through the dark forest."
    events: [
        0, play_ambient("forest_night.ogg"),
        5, fade_in_music()
    ]
    
    text: "Suddenly, a strange blue light flickers ahead."
    events: [
        10, flash_effect("#0088FF")
    ]
    
    text: "You approach and see a pool of glowing spring water..."
    
    choice: [
        "Observe cautiously" -> ObserveSpring,
        "Drink directly" -> DirectDrink,
        "Leave this place" -> ChooseLeave
    ]
}

// ========== Observation Branch ==========
node ObserveSpring {
    text: "You crouch down and carefully examine the spring."
    text: "Ancient text appears on the water's surface..."
    events: [
        0, show_text_effect("ancient_runes")
    ]
    
    text: "The text reads: 'Those who drink this sacred spring shall gain true knowledge and power.'"
    
    choice: [
        "Then I'll drink it" -> CautiousDrink,
        "Feels scary, better leave" -> ChooseLeave,
        
        // Special option for players with equipment
        "Collect water with magic bottle" when has_magic_bottle() -> CollectWater
    ]
}

node CautiousDrink {
    text: "You carefully cup some spring water and take a small sip."
    events: [
        35, play_sound("drink_water.wav")
    ]
    
    text: "A cool energy surges through your body!"
    events: [
        0, screen_flash("#00FFFF"),
        0, play_sound("power_up.wav")
    ]
    
    text: $"You feel your power growing... Power increased by {get_power_bonus()} points!"
    
} -> GoodEndingPower

node CollectWater {
    text: "You take out your precious magic bottle and carefully collect the spring water."
    events: [
        0, play_sound("bottle_fill.wav"),
        44, show_item_obtained("holy_water")
    ]
    
    text: "This is a priceless treasure that could save your life at a critical moment!"
    
} -> GoodEndingWisdom

// ========== Direct Drink Branch ==========
node DirectDrink {
    text: "Without hesitation, you take a big gulp!"
    events: [
        23, play_sound("gulp.wav")
    ]
    
    text: "Gulp gulp—"
    
    // Check if player has enough resistance
    choice: [
        // Has resistance: safe
        "(Continue)" when has_magic_resistance() -> DirectDrinkSuccess,
        
        // No resistance: trouble
        "(Continue)" -> DirectDrinkFail
    ]
}

node DirectDrinkSuccess {
    text: "Thanks to your strong magic resistance, the spring's power is perfectly absorbed!"
    events: [
        0, play_sound("success.wav")
    ]
    
    text: "You feel more powerful than ever!"
    
} -> GoodEndingPower

node DirectDrinkFail {
    text: "Oh no! The magic is too strong, your body can't handle it!"
    events: [
        0, screen_shake(),
        0, play_sound("magic_overload.wav")
    ]
    
    text: "Your vision goes black as you collapse to the ground..."
    
} -> BadEndingUnconscious

// ========== Leave Branch ==========
node ChooseLeave {
    text: "You decide to stay cautious and leave this mysterious place."
    
    text: "After a few steps, you look back..."
    
    text: "The spring's glow gradually dims, as if saying: 'The opportunity is lost.'"
    events: [
        63, fade_out_effect()
    ]
    
} -> NormalEndingCautious

// ========== Ending Nodes ==========
node GoodEndingPower {
    text: "=== Ending: Power Awakening ==="
    events: [
        0, play_music("victory_theme.ogg")
    ]
    
    text: "You have received the spring's blessing and become a powerful warrior!"
    text: $"Final power: {get_final_power()}"
    text: "From now on, you are unstoppable in your adventures."
    
    text: "[Game Over]"
}

node GoodEndingWisdom {
    text: "=== Ending: Path of the Wise ==="
    events: [
        0, play_music("wisdom_theme.ogg")
    ]
    
    text: "You have shown true wisdom, knowing how to use treasures."
    text: "The sacred spring water has become your most precious possession."
    text: "In later adventures, this bottle saved you countless times."
    
    text: "[Game Over]"
}

node BadEndingUnconscious {
    text: "=== Ending: Price of Greed ==="
    events: [
        0, play_music("bad_ending.ogg"),
        0, screen_fade_black()
    ]
    
    text: "When you wake up, it's already the next morning."
    text: "The spring has disappeared, and so has your power."
    text: "You regret not being more cautious..."
    
    text: "[Game Over]"
}

node NormalEndingCautious {
    text: "=== Ending: Ordinary Path ==="
    events: [
        0, play_music("normal_ending.ogg")
    ]
    
    text: "You chose safety over adventure."
    text: "While you didn't gain power, you didn't encounter danger either."
    text: "A plain and simple life is also a way of living."
    
    text: "[Game Over]"
}

// ========== Function Declarations ==========
// Audio and Visual Effects
fn play_ambient(filename: String)
fn play_sound(file_name: String)
fn play_music(filename: String)
fn fade_in_music()
fn fade_out_effect()
fn screen_fade_black()

// Special Effects
fn flash_effect(color: String)
fn screen_flash(color: String)
fn screen_shake()
fn show_text_effect(effect_name: String)
fn show_item_obtained(item_name: String)

// Conditional Checks
fn has_magic_bottle() -> Bool
fn has_magic_resistance() -> Bool

// Value Getters
fn get_power_bonus() -> Number
fn get_final_power() -> Number
```

## Story Structure Diagram

```
                    Opening
                       │
           ┌───────────┼───────────┐
           │           │           │
      Observe      Direct      Choose
       Spring      Drink       Leave
           │           │           │
      ┌────┼────┐      │      Normal_Cautious
      │    │    │      │         Ending
  Cautious Collect Leave Check
   Drink   Water      Resistance
      │      │       /     \
      │      │    Success  Fail
      │      │       │       │
      │      │    Good    Bad_Unconscious
      └──────┴────Power     Ending
             │    Ending
          Good_Wisdom
           Ending
```

## Key Technique Analysis

### 1. Multi-layer Choices

Using conditional logic to show different options to different players:

```mortar
choice: [
    "Normal option" -> NormalNode,
    "Special option" when has_special_item() -> SpecialNode
]
```

### 2. Hidden Branches

The `DirectDrink` node handles this cleverly:

```mortar
choice: [
    // Both options show same text
    "(Continue)" when has_magic_resistance() -> Success,
    "(Continue)" -> Fail
]
```

Players can't see the difference, but outcomes vary—this is a hidden branch!

### 3. Clever Use of String Interpolation

Dynamically display values:

```mortar
text: $"Power increased by {get_power_bonus()} points!"
text: $"Final power: {get_final_power()}"
```

### 4. Event Synchronization

Trigger multiple events at the same position:

```mortar
events: [
    0, screen_flash("#00FFFF"),
    0, play_sound("power_up.wav")  // Trigger simultaneously
]
```

### 5. Chapter-style Organization

Use comments to separate different sections:

```mortar
// ========== Opening Scene ==========

// ========== Observation Branch ==========

// ========== Ending Nodes ==========
```

Makes code more readable!

## Compile and Test

```bash
# Compile
mortar forest_story.mortar -o story.json --pretty

# Check generated JSON structure
cat story.json
```

## Game Implementation Points

In your game, you need to implement:

### 1. Conditional Check Functions

```csharp
bool has_magic_bottle() {
    return Inventory.HasItem("magic_bottle");
}

bool has_magic_resistance() {
    return Player.Stats.MagicResistance >= 50;
}
```

### 2. Value Calculation Functions

```csharp
float get_power_bonus() {
    return Player.Level * 10 + 50;
}

float get_final_power() {
    return Player.Stats.Power;
}
```

### 3. Sound Effects and Visual Effects

```csharp
void play_sound(string filename) {
    AudioManager.Play(filename);
}

void screen_flash(string color) {
    ScreenEffects.Flash(ColorUtility.TryParseHtmlString(color, out Color c) ? c : Color.white);
}
```

## Extension Suggestions

You can build on this foundation:

1. **Add More Branches**
   - Add "touch the spring with hand" option
   - Include "make a wish to the spring" mysterious branch

2. **Add State Tracking**
   - Record player's choices
   - Display player's decision path in ending

3. **Multiple Ending Variants**
   - Unlock hidden endings based on previous game progress
   - Add "true ending" requiring specific conditions

4. **Integrate with Game Systems**
   - Endings affect subsequent storylines
   - Give different rewards

## Summary

This example demonstrates:
- ✅ Multi-branch narrative design
- ✅ Flexible use of conditional logic
- ✅ Hidden options and branches
- ✅ String interpolation
- ✅ Multiple ending implementation
- ✅ Sound and visual effect coordination

This is Mortar's true power—making complex branching narratives clear and manageable!

## Next Steps

- Want to learn about integration? See [Integrating with Your Game](./5_3_game-integration.md)
- Want to understand JSON structure? See [JSON Output Format](./7_1_json-output.md)
- Back to examples overview: [Complete Examples and Explanations](./5_0_examples.md)
