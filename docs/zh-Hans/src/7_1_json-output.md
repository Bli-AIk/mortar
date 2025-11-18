# JSON 输出说明

Mortar 编译后生成的是标准 JSON 格式，这一章详细说明 JSON 的结构。

## 整体结构

```json
{
  "nodes": { ... },
  "functions": [ ... ],
  "metadata": { ... }
}
```

有三个顶层字段：

- **nodes** - 所有对话节点
- **functions** - 函数声明列表
- **metadata** - 元数据（版本等）

## nodes 字段

一个字典（对象），键是节点名，值是节点数据：

```json
{
  "nodes": {
    "节点名1": { ... },
    "节点名2": { ... }
  }
}
```

### 节点结构

每个节点包含：

```json
{
  "texts": [ ... ],      // 文本块列表
  "choices": [ ... ],    // 选项列表（可选）
  "next_node": "..."     // 下一个节点（可选）
}
```

## texts 字段

文本块列表，每个文本块的结构：

```json
{
  "content": "对话内容",
  "interpolated": false,
  "events": [ ... ]
}
```

### 字段说明

- **content** (string) - 文本内容
- **interpolated** (boolean) - 是否包含插值（`$"...{...}..."`）
- **events** (array) - 事件列表

### 示例

#### 简单文本

```mortar
text: "你好！"
```

生成：

```json
{
  "content": "你好！",
  "interpolated": false,
  "events": []
}
```

#### 带插值的文本

```mortar
text: $"你好，{get_name()}！"
```

生成：

```json
{
  "content": "你好，{get_name()}！",
  "interpolated": true,
  "events": []
}
```

注意：插值的实际处理由游戏代码负责。

#### 带事件的文本

```mortar
text: "欢迎回来！"
events: [
    0, play_sound("welcome.wav"),
    3, fade_in()
]
```

生成：

```json
{
  "content": "欢迎回来！",
  "interpolated": false,
  "events": [
    {
      "index": 0,
      "calls": [
        {
          "function": "play_sound",
          "args": ["welcome.wav"]
        }
      ]
    },
    {
      "index": 3,
      "calls": [
        {
          "function": "fade_in",
          "args": []
        }
      ]
    }
  ]
}
```

## events 字段详解

事件列表的结构：

```json
{
  "index": 0.0,           // 触发位置（数字）
  "calls": [              // 函数调用列表
    {
      "function": "函数名",
      "args": [ ... ]     // 参数列表
    }
  ]
}
```

### 多个事件在同一位置

```mortar
events: [
    0, effect_a(),
    0, effect_b(),
    0, effect_c()
]
```

生成：

```json
{
  "index": 0,
  "calls": [
    {"function": "effect_a", "args": []},
    {"function": "effect_b", "args": []},
    {"function": "effect_c", "args": []}
  ]
}
```

### 小数索引

```mortar
events: [
    0.5, sound_a(),
    1.25, sound_b()
]
```

生成：

```json
[
  {
    "index": 0.5,
    "calls": [{"function": "sound_a", "args": []}]
  },
  {
    "index": 1.25,
    "calls": [{"function": "sound_b", "args": []}]
  }
]
```

### 函数参数

```mortar
events: [
    0, play_sound("bgm.ogg"),
    1, set_color("#FF0000"),
    2, set_volume(0.8)
]
```

生成：

```json
[
  {
    "index": 0,
    "calls": [{
      "function": "play_sound",
      "args": ["bgm.ogg"]
    }]
  },
  {
    "index": 1,
    "calls": [{
      "function": "set_color",
      "args": ["#FF0000"]
    }]
  },
  {
    "index": 2,
    "calls": [{
      "function": "set_volume",
      "args": [0.8]
    }]
  }
]
```

参数类型在 JSON 中：
- 字符串 → `"string"`
- 数字 → `0.8`（JSON number）
- 布尔 → `true` / `false`

## choices 字段

选项列表，每个选项的结构：

```json
{
  "text": "选项文字",
  "target": "目标节点",
  "condition": "条件函数"  // 可选
}
```

### 简单选项

```mortar
choice: [
    "选项A" -> NodeA,
    "选项B" -> NodeB
]
```

生成：

```json
[
  {
    "text": "选项A",
    "target": "NodeA"
  },
  {
    "text": "选项B",
    "target": "NodeB"
  }
]
```

### 带条件的选项

```mortar
choice: [
    "选项A" -> NodeA,
    "选项B" when has_key() -> NodeB
]
```

生成：

```json
[
  {
    "text": "选项A",
    "target": "NodeA"
  },
  {
    "text": "选项B",
    "target": "NodeB",
    "condition": "has_key"
  }
]
```

游戏需要检查 `condition` 字段，调用对应函数判断是否显示该选项。

### 嵌套选项

```mortar
choice: [
    "主选项" -> [
        "子选项A" -> NodeA,
        "子选项B" -> NodeB
    ]
]
```

生成：

```json
[
  {
    "text": "主选项",
    "nested": [
      {
        "text": "子选项A",
        "target": "NodeA"
      },
      {
        "text": "子选项B",
        "target": "NodeB"
      }
    ]
  }
]
```

嵌套选项用 `nested` 字段表示。

### 特殊目标

```mortar
choice: [
    "返回" -> return,
    "终止" -> break
]
```

生成：

```json
[
  {
    "text": "返回",
    "target": "return"
  },
  {
    "text": "终止",
    "target": "break"
  }
]
```

`return` 和 `break` 是特殊关键字，游戏需要特殊处理。

## functions 字段

函数声明列表：

```json
[
  {
    "name": "函数名",
    "params": [ ... ],
    "return_type": "返回类型"  // 可选
  }
]
```

### 无参数无返回

```mortar
fn simple_func()
```

生成：

```json
{
  "name": "simple_func",
  "params": [],
  "return_type": null
}
```

### 有参数无返回

```mortar
fn play_sound(filename: String)
```

生成：

```json
{
  "name": "play_sound",
  "params": [
    {
      "name": "filename",
      "type": "String"
    }
  ],
  "return_type": null
}
```

### 有参数有返回

```mortar
fn get_score(player_id: Number) -> Number
```

生成：

```json
{
  "name": "get_score",
  "params": [
    {
      "name": "player_id",
      "type": "Number"
    }
  ],
  "return_type": "Number"
}
```

### 多参数

```mortar
fn complex_func(a: String, b: Number, c: Bool) -> Bool
```

生成：

```json
{
  "name": "complex_func",
  "params": [
    {"name": "a", "type": "String"},
    {"name": "b", "type": "Number"},
    {"name": "c", "type": "Bool"}
  ],
  "return_type": "Bool"
}
```

## metadata 字段

元数据，包含编译信息：

```json
{
  "version": "0.3.0",
  "compiler": "mortar_cli",
  "compiled_at": "2024-01-15T10:30:00Z"
}
```

## 完整示例

### Mortar 源文件

```mortar
node 开始 {
    text: "你好！"
    events: [
        0, play_sound("hi.wav")
    ]
    
    text: $"你的名字是{get_name()}吗？"
    
    choice: [
        "是的" -> 确认,
        "不是" -> 否认
    ]
}

node 确认 {
    text: "很高兴认识你！"
}

node 否认 {
    text: "哦，那你叫什么？"
}

fn play_sound(file: String)
fn get_name() -> String
```

### 编译后的 JSON

```json
{
  "nodes": {
    "开始": {
      "texts": [
        {
          "content": "你好！",
          "interpolated": false,
          "events": [
            {
              "index": 0,
              "calls": [
                {
                  "function": "play_sound",
                  "args": ["hi.wav"]
                }
              ]
            }
          ]
        },
        {
          "content": "你的名字是{get_name()}吗？",
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
          "content": "很高兴认识你！",
          "interpolated": false,
          "events": []
        }
      ]
    },
    "否认": {
      "texts": [
        {
          "content": "哦，那你叫什么？",
          "interpolated": false,
          "events": []
        }
      ]
    }
  },
  "functions": [
    {
      "name": "play_sound",
      "params": [
        {
          "name": "file",
          "type": "String"
        }
      ],
      "return_type": null
    },
    {
      "name": "get_name",
      "params": [],
      "return_type": "String"
    }
  ],
  "metadata": {
    "version": "0.3.0",
    "compiler": "mortar_cli"
  }
}
```

## 解析建议

### TypeScript 类型定义

```typescript
interface MortarDialogue {
  nodes: Record<string, Node>;
  functions: FunctionDecl[];
  metadata: Metadata;
}

interface Node {
  texts: TextBlock[];
  choices?: Choice[];
  next_node?: string;
}

interface TextBlock {
  content: string;
  interpolated: boolean;
  events: EventGroup[];
}

interface EventGroup {
  index: number;
  calls: FunctionCall[];
}

interface FunctionCall {
  function: string;
  args: any[];
}

interface Choice {
  text: string;
  target?: string;
  nested?: Choice[];
  condition?: string;
}

interface FunctionDecl {
  name: string;
  params: Param[];
  return_type: string | null;
}

interface Param {
  name: string;
  type: string;
}

interface Metadata {
  version: string;
  compiler: string;
  compiled_at?: string;
}
```

### Python 数据类

```python
from dataclasses import dataclass
from typing import List, Dict, Optional, Any

@dataclass
class FunctionCall:
    function: str
    args: List[Any]

@dataclass
class EventGroup:
    index: float
    calls: List[FunctionCall]

@dataclass
class TextBlock:
    content: str
    interpolated: bool
    events: List[EventGroup]

@dataclass
class Choice:
    text: str
    target: Optional[str] = None
    nested: Optional[List['Choice']] = None
    condition: Optional[str] = None

@dataclass
class Node:
    texts: List[TextBlock]
    choices: Optional[List[Choice]] = None
    next_node: Optional[str] = None

@dataclass
class Param:
    name: str
    type: str

@dataclass
class FunctionDecl:
    name: str
    params: List[Param]
    return_type: Optional[str]

@dataclass
class Metadata:
    version: str
    compiler: str
    compiled_at: Optional[str] = None

@dataclass
class MortarDialogue:
    nodes: Dict[str, Node]
    functions: List[FunctionDecl]
    metadata: Metadata
```

## 注意事项

### 1. 字符编码

JSON 文件始终是 UTF-8 编码，确保正确读取。

### 2. 节点顺序

`nodes` 是字典，**不保证顺序**。不要依赖节点的顺序！

### 3. 空数组 vs null

- 没有事件：`"events": []`（空数组）
- 没有选项：`"choices": null` 或不存在该字段

### 4. 插值处理

`interpolated: true` 的文本，需要游戏代码进行实际的替换：

```javascript
if (textBlock.interpolated) {
  let result = textBlock.content;
  // 找到 {function_name()} 并替换
  result = result.replace(/\{(\w+)\(\)\}/g, (_, funcName) => {
    return callFunction(funcName);
  });
  displayText(result);
}
```

### 5. 条件判断

带 `condition` 的选项，需要调用函数检查：

```python
for choice in node['choices']:
    if 'condition' in choice:
        if not call_function(choice['condition']):
            continue  # 不显示此选项
    show_choice(choice['text'])
```

## 小结

Mortar JSON 结构清晰：
- ✅ 标准 JSON 格式
- ✅ 分层明确
- ✅ 易于解析
- ✅ 类型明确

掌握了这个结构，就能轻松集成到任何游戏引擎！

## 接下来

- 看完整集成示例：[接入你的游戏](../5_3_game-integration.md)
- 了解常见问题：[FAQ](./7_2_faq.md)
- 查看贡献指南：[贡献指南](./7_3_contributing.md)
