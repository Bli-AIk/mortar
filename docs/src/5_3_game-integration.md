# Integrating with Your Game (WIP)

> ⚠️ This chapter will be refactored. Content is for limited reference only.
>
> Mortar will provide official Bevy and Unity bindings in the future.

This chapter will guide you step-by-step on how to truly use Mortar—from compilation to running in your game.

## Complete Process Overview

```
1. Write Mortar script (.mortar)
         ↓
2. Use compiler to generate JSON
         ↓
3. Game loads JSON file
         ↓
4. Parse JSON data structure
         ↓
5. Implement function call interface
         ↓
6. Write dialogue execution engine
         ↓
7. Run dialogue and respond to events
```

## Example: A Simple Dialogue

Let's start with the simplest example.

### Step 1: Write Mortar File

Create `simple.mortar`:

```mortar
node StartScene {
    text: "Hello!"
    with events: [
        0, play_sound("hi.wav")
    ]
    
    text: $"Are you {get_name()}?"
    
    choice: [
        "Yes" -> Confirm,
        "No" -> Deny
    ]
}

node Confirm {
    text: "Nice to meet you!"
}

node Deny {
    text: "Oh, sorry, wrong person."
}

fn play_sound(file: String)
fn get_name() -> String
```

### Step 2: Compile to JSON

```bash
mortar simple.mortar -o simple.json --pretty
```

The generated `simple.json` looks roughly like this:

```json
{
  "nodes": {
    "StartScene": {
      "texts": [
        {
          "content": "Hello!",
          "events": [
            {
              "index": 0,
              "function": "play_sound",
              "args": ["hi.wav"]
            }
          ]
        },
        {
          "content": "Are you {get_name()}?",
          "interpolated": true,
          "events": []
        }
      ],
      "choices": [
        {
          "text": "Yes",
          "target": "Confirm"
        },
        {
          "text": "No",
          "target": "Deny"
        }
      ]
    },
    "Confirm": {
      "texts": [
        {
          "content": "Nice to meet you!",
          "events": []
        }
      ]
    },
    "Deny": {
      "texts": [
        {
          "content": "Oh, sorry, wrong person.",
          "events": []
        }
      ]
    }
  },
  "functions": [
    {
      "name": "play_sound",
      "params": [{"name": "file", "type": "String"}],
      "return_type": null
    },
    {
      "name": "get_name",
      "params": [],
      "return_type": "String"
    }
  ]
}
```

## Unity C# Integration Example

### Step 1: Create Data Structures

```csharp
using System;
using System.Collections.Generic;
using UnityEngine;

[Serializable]
public class MortarDialogue {
    public Dictionary<string, NodeData> nodes;
    public List<FunctionDeclaration> functions;
}

[Serializable]
public class NodeData {
    public List<TextBlock> texts;
    public List<Choice> choices;
    public string next_node;
}

[Serializable]
public class TextBlock {
    public string content;
    public bool interpolated;
    public List<Event> events;
}

[Serializable]
public class Event {
    public float index;
    public string function;
    public List<object> args;
}

[Serializable]
public class Choice {
    public string text;
    public string target;
    public string condition;
}

[Serializable]
public class FunctionDeclaration {
    public string name;
    public List<Param> @params;
    public string return_type;
}

[Serializable]
public class Param {
    public string name;
    public string type;
}
```

### Step 2: Load JSON

```csharp
using System.IO;
using UnityEngine;

public class DialogueManager : MonoBehaviour {
    private MortarDialogue dialogue;
    
    public void LoadDialogue(string jsonPath) {
        string json = File.ReadAllText(jsonPath);
        dialogue = JsonUtility.FromJson<MortarDialogue>(json);
        Debug.Log($"Loaded {dialogue.nodes.Count} dialogue nodes");
    }
}
```

### Step 3: Implement Function Interface

```csharp
using System.Collections.Generic;
using UnityEngine;

public class DialogueFunctions : MonoBehaviour {
    private Dictionary<string, System.Delegate> functionMap;
    
    void Awake() {
        InitializeFunctions();
    }
    
    void InitializeFunctions() {
        functionMap = new Dictionary<string, System.Delegate>();
        
        // Register all functions
        functionMap["play_sound"] = new System.Action<string>(PlaySound);
        functionMap["get_name"] = new System.Func<string>(GetName);
        functionMap["has_item"] = new System.Func<bool>(HasItem);
    }
    
    // ===== Actual function implementations =====
    
    void PlaySound(string filename) {
        AudioClip clip = Resources.Load<AudioClip>($"Sounds/{filename}");
        if (clip != null) {
            AudioSource.PlayClipAtPoint(clip, Camera.main.transform.position);
        }
    }
    
    string GetName() {
        return PlayerData.Instance.playerName;
    }
    
    bool HasItem() {
        return Inventory.Instance.HasItem("magic_map");
    }
    
    // ===== Generic method to call functions =====
    
    public object CallFunction(string funcName, List<object> args) {
        if (!functionMap.ContainsKey(funcName)) {
            Debug.LogError($"Function not defined: {funcName}");
            return null;
        }
        
        var func = functionMap[funcName];
        
        // Call based on parameter count
        if (args == null || args.Count == 0) {
            if (func is System.Func<string>) {
                return ((System.Func<string>)func)();
            } else if (func is System.Func<bool>) {
                return ((System.Func<bool>)func)();
            } else if (func is System.Action) {
                ((System.Action)func)();
                return null;
            }
        } else if (args.Count == 1) {
            if (func is System.Action<string>) {
                ((System.Action<string>)func)((string)args[0]);
                return null;
            }
        }
        
        Debug.LogError($"Function call failed: {funcName}");
        return null;
    }
}
```

### Step 4: Implement Dialogue Engine

```csharp
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class DialogueEngine : MonoBehaviour {
    public TextMeshProUGUI dialogueText;
    public GameObject choiceButtonPrefab;
    public Transform choiceContainer;
    
    private MortarDialogue dialogue;
    private DialogueFunctions functions;
    private string currentNode;
    private int currentTextIndex;
    
    void Start() {
        functions = GetComponent<DialogueFunctions>();
    }
    
    public void StartDialogue(MortarDialogue dialogueData, string startNode) {
        dialogue = dialogueData;
        currentNode = startNode;
        currentTextIndex = 0;
        ShowNextText();
    }
    
    void ShowNextText() {
        if (!dialogue.nodes.ContainsKey(currentNode)) {
            Debug.LogError($"Node not found: {currentNode}");
            return;
        }
        
        NodeData node = dialogue.nodes[currentNode];
        
        if (currentTextIndex < node.texts.Count) {
            TextBlock textBlock = node.texts[currentTextIndex];
            StartCoroutine(DisplayText(textBlock));
        } else {
            // All text displayed, show choices
            if (node.choices != null && node.choices.Count > 0) {
                ShowChoices(node.choices);
            } else if (!string.IsNullOrEmpty(node.next_node)) {
                // Jump to next node
                currentNode = node.next_node;
                currentTextIndex = 0;
                ShowNextText();
            } else {
                // Dialogue ends
                EndDialogue();
            }
        }
    }
    
    IEnumerator DisplayText(TextBlock textBlock) {
        string content = textBlock.content;
        
        // Handle string interpolation
        if (textBlock.interpolated) {
            content = ProcessInterpolation(content);
        }
        
        // Prepare events for typewriter effect
        Dictionary<int, List<Event>> eventMap = new Dictionary<int, List<Event>>();
        if (textBlock.events != null) {
            foreach (var evt in textBlock.events) {
                int index = Mathf.FloorToInt(evt.index);
                if (!eventMap.ContainsKey(index)) {
                    eventMap[index] = new List<Event>();
                }
                eventMap[index].Add(evt);
            }
        }
        
        // Typewriter effect
        dialogueText.text = "";
        for (int i = 0; i < content.Length; i++) {
            dialogueText.text += content[i];
            
            // Trigger events at specific character index
            if (eventMap.ContainsKey(i)) {
                foreach (var evt in eventMap[i]) {
                    functions.CallFunction(evt.function, evt.args);
                }
            }
            
            yield return new WaitForSeconds(0.05f); // Adjust speed here
        }
        
        yield return new WaitForSeconds(0.5f); // Wait after text is shown
        
        currentTextIndex++;
        ShowNextText();
    }
    
    string ProcessInterpolation(string text) {
        // A more robust implementation would use regex or a proper parser
        int start = text.IndexOf('{');
        while (start >= 0) {
            int end = text.IndexOf('}', start);
            if (end < 0) break;
            
            string funcCall = text.Substring(start + 1, end - start - 1);
            string funcName = funcCall.Replace("()", ""); // Simplified parsing
            
            object result = functions.CallFunction(funcName, null);
            text = text.Replace($"{{{funcCall}}}", result?.ToString() ?? "");
            
            start = text.IndexOf('{', start + 1);
        }
        return text;
    }
    
    void ShowChoices(List<Choice> choices) {
        // Clear old choices
        foreach (Transform child in choiceContainer) {
            Destroy(child.gameObject);
        }
        
        foreach (var choice in choices) {
            // Check condition
            if (!string.IsNullOrEmpty(choice.condition)) {
                bool conditionMet = (bool)functions.CallFunction(choice.condition, null);
                if (!conditionMet) continue;
            }
            
            GameObject button = Instantiate(choiceButtonPrefab, choiceContainer);
            button.GetComponentInChildren<TextMeshProUGUI>().text = choice.text;
            
            string target = choice.target;
            button.GetComponent<UnityEngine.UI.Button>().onClick.AddListener(() => {
                OnChoiceSelected(target);
            });
        }
    }
    
    void OnChoiceSelected(string target) {
        if (target == "return") {
            EndDialogue();
            return;
        }
        
        // Clear choices before showing next text
        foreach (Transform child in choiceContainer) {
            Destroy(child.gameObject);
        }
        
        currentNode = target;
        currentTextIndex = 0;
        ShowNextText();
    }
    
    void EndDialogue() {
        dialogueText.text = "";
        // Clear choices
        foreach (Transform child in choiceContainer) {
            Destroy(child.gameObject);
        }
    }
}
```

### Step 5: Usage

```csharp
public class GameController : MonoBehaviour {
    public DialogueEngine dialogueEngine;
    public DialogueManager dialogueManager;
    
    void Start() {
        // Load dialogue
        dialogueManager.LoadDialogue("Assets/Dialogues/simple.json");
        
        // Start dialogue
        dialogueEngine.StartDialogue(dialogueManager.dialogue, "StartScene");
    }
}
```

## Godot GDScript Integration Example

### Simplified Implementation

```gdscript
extends Node

# Dialogue data
var dialogue_data = {}
var current_node = ""
var current_text_index = 0

# UI references
@onready var dialogue_label = $DialogueLabel
@onready var choice_container = $ChoiceContainer

func load_dialogue(json_path: String):
    var file = FileAccess.open(json_path, FileAccess.READ)
    var json_string = file.get_as_text()
    file.close()
    
    var json = JSON.new()
    var error = json.parse(json_string)
    if error == OK:
        dialogue_data = json.data
        print("Loaded %d nodes" % dialogue_data.nodes.size())
    else:
        print("JSON parse error: ", json.get_error_message(), " at line ", json.get_error_line())

func start_dialogue(start_node: String):
    current_node = start_node
    current_text_index = 0
    show_next_text()

func show_next_text():
    if not current_node in dialogue_data.nodes:
        return
    
    var node = dialogue_data.nodes[current_node]
    
    if current_text_index < node.texts.size():
        var text_block = node.texts[current_text_index]
        display_text(text_block)
    else:
        if node.has("choices") and node.choices.size() > 0:
            show_choices(node.choices)
        elif node.has("next_node"):
            current_node = node.next_node
            current_text_index = 0
            show_next_text()
        else:
            end_dialogue()

func display_text(text_block: Dictionary):
    var content = text_block.content
    
    if text_block.get("interpolated", false):
        content = process_interpolation(content)
    
    # Typewriter effect
    dialogue_label.text = ""
    for i in range(content.length()):
        dialogue_label.text += content[i]
        
        # Trigger events
        if text_block.has("events"):
            for event in text_block.events:
                if int(event.index) == i:
                    call_function(event.function, event.args)
        
        await get_tree().create_timer(0.05).timeout
    
    current_text_index += 1
    await get_tree().create_timer(0.5).timeout
    show_next_text()

func process_interpolation(text: String) -> String:
    var regex = RegEx.new()
    regex.compile("\\{([^}]+)\\}")
    
    for match in regex.search_all(text):
        var func_call = match.get_string(1).replace("()", "")
        var value = call_function(func_call, [])
        text = text.replace(match.get_string(), str(value))
    
    return text

func show_choices(choices: Array):
    # Clear old choices
    for child in choice_container.get_children():
        child.queue_free()
    
    for choice in choices:
        # Check condition
        if choice.has("condition") and not choice.condition.is_empty():
            var condition_met = call_function(choice.condition, [])
            if not condition_met:
                continue
        
        var button = Button.new()
        button.text = choice.text
        button.pressed.connect(on_choice_selected.bind(choice.target))
        choice_container.add_child(button)

func on_choice_selected(target: String):
    if target == "return":
        end_dialogue()
        return
    
    current_node = target
    current_text_index = 0
    # Clear choices before showing next text
    for child in choice_container.get_children():
        child.queue_free()
    show_next_text()

func end_dialogue():
    dialogue_label.text = ""
    for child in choice_container.get_children():
        child.queue_free()

# ===== Function calling =====

func call_function(func_name: String, args: Array):
    if has_method(func_name):
        return call(func_name, args)
    else:
        print("Unknown function: ", func_name)
        return null

# Function implementations
func play_sound(args: Array):
    var filename = args[0]
    # Play sound logic here
    print("Playing sound: ", filename)

func get_name(args: Array) -> String:
    return "Player" # Replace with actual player data

func has_item(args: Array) -> bool:
    return true # Replace with actual inventory check
```

## General Implementation Points

### 1. JSON Parsing

Different engines have different JSON parsing methods:
- **Unity**: `JsonUtility` or `Newtonsoft.Json`
- **Godot**: Built-in `JSON` class
- **Unreal**: `FJsonObjectConverter`
- **Custom**: Use libraries like `rapidjson`, `nlohmann/json`

### 2. Function Mapping

Use dictionary/map to map function names to actual implementations:

```csharp
// C# example
Dictionary<string, Delegate> functions = new Dictionary<string, Delegate>();
functions["play_sound"] = new Action<string>(PlaySound);
```

```gdscript
# GDScript example
var functions = {
    "play_sound": play_sound,
    "get_name": get_name
}
```

### 3. Dialogue Flow Control

Core logic:
1. Load node
2. Display text sequentially
3. Trigger events
4. Show choices or jump to next node

### 4. Event Triggering

Events have **index** indicating trigger timing:
- **Integer index**: Suitable for typewriter effect (character by character)
- **Decimal index**: Suitable for voice sync (by timeline)

Implementation approaches:
- **Immediate**: Trigger all events at once
- **Progressive**: Trigger based on display progress
- **Timed**: Calculate trigger time based on index

### 5. Conditional Logic

Choices can have conditions, only displayed when condition is met:

```json
{
  "text": "Use magic",
  "target": "MagicPath",
  "condition": "can_use_magic"
}
```

Implementation:
1. Parse `condition` field
2. Call corresponding function
3. Display choice only if returns `true`

### 6. String Interpolation

Texts marked `interpolated: true` contain `{function()}`:

```json
{
  "content": "Hello, {get_player_name()}!",
  "interpolated": true
}
```

Implementation:
1. Find `{...}` patterns
2. Extract function names
3. Call functions to get values
4. Replace placeholders

## Performance Optimization Suggestions

### 1. Pre-parse and Preload
Parse JSON once at game start and cache it in memory. Preload related resources like audio and images before the dialogue begins.

### 2. Object Pooling
For UI elements like choice buttons, use an object pool to avoid frequent creation and destruction, which can cause performance overhead.

### 3. Lazy Loading
For very large dialogue files, consider loading nodes on-demand as the player progresses, rather than loading the entire file at once.

### 4. Asynchronous Operations
For complex string interpolations or conditional checks that might involve heavy computation, use asynchronous methods to avoid blocking the main game thread and causing frame drops.

### 5. Batching
Batch process events that trigger at the same time or in quick succession to reduce the number of individual calls.

## Common Questions

### Q: What if the JSON file is too large?
**A**: 
1.  **Split Files**: Break the dialogue into smaller files by chapter, scene, or character. Load them as needed.
2.  **Compression**: Compress the JSON files (e.g., using Gzip) and decompress them at runtime.
3.  **Binary Format**: For maximum performance, convert the JSON into a custom binary format for faster loading and parsing.
4.  **Streaming Load**: Load the JSON in chunks or stream it to parse it progressively without loading the whole file into memory.

### Q: How to implement a save/load feature?
**A**: You need to save the current state of the dialogue. The essential data to save includes:
```json
{
  "current_node": "ForestScene",
  "current_text_index": 2,
  "game_variables": {
    "has_magic_bottle": true,
    "player_power": 150
  }
}
```
When loading, restore the dialogue engine to this state and continue execution.

### Q: How to support fast-forwarding or skipping dialogue?
**A**:
-   **Text**: Immediately display the full text instead of using the typewriter effect.
-   **Events**: You can choose to either trigger all events in a text block instantly or skip them entirely.
-   **Choices**: The engine can jump directly to the next choice point, skipping all intermediate text.

### Q: Can a dialogue be interrupted (e.g., by combat)?
**A**: Yes. The key is to save the dialogue state before switching to another game system (like combat). After the interruption is over, you can restore the state to resume the dialogue exactly where it left off.

### Q: How to handle localization?
**A**: Mortar itself is language-agnostic. You can manage localization in a few ways:
1.  **Separate Files**: Compile different `.mortar` files for each language (e.g., `story_en.mortar`, `story_zh.mortar`) and load the appropriate JSON based on the player's language setting.
2.  **External String Tables**: Use keys in your `.mortar` file (e.g., `text: "dialogue_key_001"`) and look up the translated text from a language-specific database or file at runtime. String interpolation can still be used with this method.


## Summary

Integrating Mortar into games involves these key steps:
1. ✅ Parse JSON structure
2. ✅ Implement function interface
3. ✅ Build dialogue engine
4. ✅ Handle events and choices
5. ✅ Manage state

The beauty of Mortar is its **simplicity**:
- Clean JSON structure
- Clear separation of concerns
- Easy to extend

## Next Steps

- Understand JSON structure: [JSON Output Format](./7_1_json-output.md)
- Common problems: [FAQ](./7_2_faq.md)
- Back to examples: [Complete Examples](./5_0_examples.md)
