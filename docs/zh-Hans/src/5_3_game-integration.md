# 接入你的游戏 （WIP）

> ⚠️ 本章节的内容将会重构。内容仅供有限参考。
>
> Mortar 在未来会提供官方的 Bevy 与 Unity 绑定。

这一章会手把手教你如何把 Mortar 真正用起来——从编译到在游戏中运行。

## 完整流程概览

```
1. 编写 Mortar 脚本 (.mortar)
         ↓
2. 使用编译器生成 JSON
         ↓
3. 游戏加载 JSON 文件
         ↓
4. 解析 JSON 数据结构
         ↓
5. 实现函数调用接口
         ↓
6. 编写对话执行引擎
         ↓
7. 运行对话并响应事件
```

## 示例：一个简单的对话

先从最简单的例子开始。

### 步骤1：编写 Mortar 文件

创建 `simple.mortar`：

```mortar
node StartScene {
    text: "你好！"
    events: [
        0, play_sound("hi.wav")
    ]
    
    text: $"你是{get_name()}吗？"
    
    choice: [
        "是的" -> Confirm,
        "不是" -> Deny
    ]
}

node Confirm {
    text: "很高兴见到你！"
}

node Deny {
    text: "哦，抱歉认错人了。"
}

fn play_sound(file: String)
fn get_name() -> String
```

### 步骤2：编译成 JSON

```bash
mortar simple.mortar -o simple.json --pretty
```

生成的 `simple.json` 大致长这样：

```json
{
  "nodes": {
    "开始": {
      "texts": [
        {
          "content": "你好！",
          "events": [
            {
              "index": 0,
              "function": "play_sound",
              "args": ["hi.wav"]
            }
          ]
        },
        {
          "content": "你是{get_name()}吗？",
          "interpolated": true,
          "events": []
        }
      ],
      "choices": [
        {
          "text": "是的",
          "target": "确认"
        },
        {
          "text": "不是",
          "target": "否认"
        }
      ]
    },
    "确认": {
      "texts": [
        {
          "content": "很高兴见到你！",
          "events": []
        }
      ]
    },
    "否认": {
      "texts": [
        {
          "content": "哦，抱歉认错人了。",
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

## Unity C# 集成示例

### 第一步：创建数据结构

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

### 第二步：加载 JSON

```csharp
using System.IO;
using UnityEngine;

public class DialogueManager : MonoBehaviour {
    private MortarDialogue dialogue;
    
    public void LoadDialogue(string jsonPath) {
        string json = File.ReadAllText(jsonPath);
        dialogue = JsonUtility.FromJson<MortarDialogue>(json);
        Debug.Log($"加载了 {dialogue.node Dialogue节点");
    }
}
```

### 第三步：实现函数接口

```csharp
using System.Collections.Generic;
using UnityEngine;

public class DialogueFunctions : MonoBehaviour {
    // 存储实际的函数实现
    private Dictionary<string, System.Delegate> functionMap;
    
    void Awake() {
        InitializeFunctions();
    }
    
    void InitializeFunctions() {
        functionMap = new Dictionary<string, System.Delegate>();
        
        // 注册所有函数
        functionMap["play_sound"] = new System.Action<string>(PlaySound);
        functionMap["get_name"] = new System.Func<string>(GetName);
        functionMap["has_item"] = new System.Func<bool>(HasItem);
    }
    
    // ===== 实际的函数实现 =====
    
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
    
    // ===== 调用函数的通用方法 =====
    
    public object CallFunction(string funcName, List<object> args) {
        if (!functionMap.ContainsKey(funcName)) {
            Debug.LogError($"函数未定义: {funcName}");
            return null;
        }
        
        var func = functionMap[funcName];
        
        // 根据参数数量调用
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
        
        Debug.LogError($"函数调用失败: {funcName}");
        return null;
    }
}
```

### 第四步：实现对话引擎

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
    
    public void StartDialogue(MortarDialogue dlg, string startNode) {
        dialogue = dlg;
        currentNode = startNode;
        currentTextIndex = 0;
        ShowCurrentText();
    }
    
    void ShowCurrentText() {
        var node = dialogue.nodes[currentNode];
        
        if (currentTextIndex < node.texts.Count) {
            var textBlock = node.texts[currentTextIndex];
            StartCoroutine(DisplayText(textBlock));
        } else {
            ShowChoices();
        }
    }
    
    IEnumerator DisplayText(TextBlock textBlock) {
        string text = textBlock.content;
        
        // 处理字符串插值
        if (textBlock.interpolated) {
            text = ProcessInterpolation(text);
        }
        
        // 准备事件
        Dictionary<int, List<Event>> eventMap = new Dictionary<int, List<Event>>();
        foreach (var evt in textBlock.events) {
            int index = Mathf.FloorToInt(evt.index);
            if (!eventMap.ContainsKey(index)) {
                eventMap[index] = new List<Event>();
            }
            eventMap[index].Add(evt);
        }
        
        // 打字机效果
        dialogueText.text = "";
        for (int i = 0; i < text.Length; i++) {
            dialogueText.text += text[i];
            
            // 触发事件
            if (eventMap.ContainsKey(i)) {
                foreach (var evt in eventMap[i]) {
                    functions.CallFunction(evt.function, evt.args);
                }
            }
            
            yield return new WaitForSeconds(0.05f);
        }
        
        yield return new WaitForSeconds(0.5f);
        
        // 下一段文本
        currentTextIndex++;
        ShowCurrentText();
    }
    
    string ProcessInterpolation(string text) {
        // 简单的插值处理：找到 {function_name()} 并替换
        // 实际项目中需要更完善的解析
        int start = text.IndexOf('{');
        while (start >= 0) {
            int end = text.IndexOf('}', start);
            if (end < 0) break;
            
            string funcCall = text.Substring(start + 1, end - start - 1);
            // 移除 () 获取函数名
            string funcName = funcCall.Replace("()", "");
            
            object result = functions.CallFunction(funcName, null);
            text = text.Replace($"{{{funcCall}}}", result?.ToString() ?? "");
            
            start = text.IndexOf('{', start + 1);
        }
        return text;
    }
    
    void ShowChoices() {
        var node = dialogue.nodes[currentNode];
        
        if (node.choices == null || node.choices.Count == 0) {
            // 没有选项，检查是否有下一个节点
            if (!string.IsNullOrEmpty(node.next_node)) {
                currentNode = node.next_node;
                currentTextIndex = 0;
                ShowCurrentText();
            }
            return;
        }
        
        // 清空旧选项
        foreach (Transform child in choiceContainer) {
            Destroy(child.gameObject);
        }
        
        // 创建选项按钮
        foreach (var choice in node.choices) {
            // 检查条件
            if (!string.IsNullOrEmpty(choice.condition)) {
                bool conditionMet = (bool)functions.CallFunction(choice.condition, null);
                if (!conditionMet) continue;
            }
            
            GameObject btn = Instantiate(choiceButtonPrefab, choiceContainer);
            btn.GetComponentInChildren<TextMeshProUGUI>().text = choice.text;
            
            string targetNode = choice.target;
            btn.GetComponent<UnityEngine.UI.Button>().onClick.AddListener(() => {
                OnChoiceSelected(targetNode);
            });
        }
    }
    
    void OnChoiceSelected(string targetNode) {
        if (targetNode == "return") {
            // 结束对话
            gameObject.SetActive(false);
            return;
        }
        
        currentNode = targetNode;
        currentTextIndex = 0;
        ShowCurrentText();
    }
}
```

### 第五步：使用

```csharp
public class GameController : MonoBehaviour {
    public DialogueManager dialogueManager;
    public DialogueEngine dialogueEngine;
    
    void Start() {
        // 加载对话文件
        dialogueManager.LoadDialogue("Assets/Dialogues/simple.json");
        
        // 开始对话
        dialogueEngine.StartDialogue(dialogueManager.GetDialogue(), "开始");
    }
}
```

## Godot GDScript 集成示例

### 简化版实现

```gdscript
extends Node

# 对话数据
var dialogue_data = {}
var current_node = ""
var current_text_index = 0

# UI 引用
onready var dialogue_label = $DialogueLabel
onready var choice_container = $ChoiceContainer

func load_dialogue(json_path: String):
    var file = File.new()
    file.open(json_path, File.READ)
    var json = file.get_as_text()
    file.close()
    
    dialogue_data = JSON.parse(json).result
    print("加载了 %d 个节点" % dialogue_data.nodes.size())

func start_dialogue(start_node: String):
    current_node = start_node
    current_text_index = 0
    show_current_text()

func show_current_text():
    var node = dialogue_data.nodes[current_node]
    
    if current_text_index < node.texts.size():
        var text_block = node.texts[current_text_index]
        display_text(text_block)
    else:
        show_choices()

func display_text(text_block):
    var text = text_block.content
    
    # 处理插值
    if text_block.get("interpolated", false):
        text = process_interpolation(text)
    
    # 打字机效果
    dialogue_label.text = ""
    for i in range(text.length()):
        dialogue_label.text += text[i]
        
        # 触发事件
        for event in text_block.events:
            if int(event.index) == i:
                call_function(event.function, event.args)
        
        yield(get_tree().create_timer(0.05), "timeout")
    
    current_text_index += 1
    yield(get_tree().create_timer(0.5), "timeout")
    show_current_text()

func process_interpolation(text: String) -> String:
    # 简单的插值处理
    var regex = RegEx.new()
    regex.compile("\\{([^}]+)\\}")
    
    for result in regex.search_all(text):
        var func_call = result.get_string(1).replace("()", "")
        var value = call_function(func_call, [])
        text = text.replace(result.get_string(), str(value))
    
    return text

func show_choices():
    var node = dialogue_data.nodes[current_node]
    
    # 清空旧选项
    for child in choice_container.get_children():
        child.queue_free()
    
    if not node.has("choices"):
        return
    
    # 创建选项按钮
    for choice in node.choices:
        # 检查条件
        if choice.has("condition") and choice.condition != "":
            if not call_function(choice.condition, []):
                continue
        
        var button = Button.new()
        button.text = choice.text
        button.connect("pressed", self, "on_choice_selected", [choice.target])
        choice_container.add_child(button)

func on_choice_selected(target: String):
    if target == "return":
        queue_free()
        return
    
    current_node = target
    current_text_index = 0
    show_current_text()

# ===== 函数调用 =====

func call_function(func_name: String, args: Array):
    match func_name:
        "play_sound":
            return play_sound(args[0])
        "get_name":
            return get_player_name()
        "has_item":
            return check_has_item()
        _:
            print("未知函数: ", func_name)
            return null

func play_sound(filename: String):
    var audio = AudioStreamPlayer.new()
    audio.stream = load("res://sounds/" + filename)
    add_child(audio)
    audio.play()

func get_player_name() -> String:
    return PlayerData.player_name

func check_has_item() -> bool:
    return PlayerData.has_item("magic_map")
```

## 通用实现要点

无论用什么引擎，都需要实现这些核心功能：

### 1. JSON 解析
- 读取文件
- 解析成对象结构
- 处理节点、文本、事件、选项

### 2. 函数映射
```
"play_sound" → 你的音效播放函数
"get_name" → 你的获取玩家名函数
...
```

### 3. 对话流程控制
- 显示当前文本
- 触发对应事件
- 处理玩家选择
- 跳转到下一个节点

### 4. 事件触发
- 根据索引位置触发
- 支持同时触发多个事件
- 处理整数和小数索引

### 5. 条件判断
- 检查选项的条件
- 只显示满足条件的选项

### 6. 字符串插值
- 识别 `{}` 标记
- 调用对应函数
- 替换成返回值

## 性能优化建议

1. **预加载资源**：对话开始前加载所有音效、图片
2. **对象池**：选项按钮使用对象池，避免频繁创建销毁
3. **异步加载**：大型对话文件使用异步加载
4. **缓存结果**：字符串插值的结果可以缓存

## 常见问题

### Q: JSON 太大怎么办？
A: 按场景/章节拆分成多个文件，按需加载。

### Q: 怎么实现存档？
A: 保存当前"NodeName"和相关变量即可恢复对话进度。

### Q: 怎么支持快进？
A: 跳过打字机效果，直接显示完整文本，快速触发所有事件。

### Q: 可以中途打断对话吗？
A: 可以，记录当前状态，下次从断点继续。

## 小结

集成 Mortar 的关键步骤：
1. ✅ 编译 Mortar 生成 JSON
2. ✅ 创建数据结构匹配 JSON
3. ✅ 实现函数调用映射
4. ✅ 编写对话执行引擎
5. ✅ 处理事件和选择

从简单例子开始，逐步增加功能，很快就能上手！

## 接下来

- 了解 JSON 详细结构：[JSON 输出说明](../7_1_json-output.md)
- 回到示例总览：[完整示例与讲解](./5_0_examples)
- 查看常见问题：[FAQ](../7_2_faq.md)
