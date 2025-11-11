# 快速开始

让我们来编写您的第一个 Mortar 对话！本指南将带您创建一个简单的交互式对话。

## 您的第一个脚本

创建一个名为 `hello.mortar` 的文件，并添加以下内容：

```mortar
// 基本对话节点
node Start {
    // 文本内容 - 干净且可读
    text: "你好！欢迎来到这个互动故事。"
    
    // 事件 - 在特定字符位置触发
    events: [
        0, play_sound("greeting.wav")
        2, set_animation("wave")
        8, set_color("#FF6B6B")
    ]
    
    // 另一个带有字符串插值的文本块
    text: $"你的名字是 {get_player_name()}，对吗？"
    events: [
        5, set_color("#33CCFF")
    ]
    
    // 跳转到下一个节点
} -> ChoiceDemo

// 带有玩家选择的节点
node ChoiceDemo {
    text: "你想要做什么？"
    
    choice: [
        "探索世界" -> Exploration,
        "查看背包" when has_backpack -> Inventory,
        "说再见" -> return
    ]
}

node Exploration {
    text: "你大胆地走向未知..."
}

// 函数声明 - 这些连接到您的游戏代码
fn play_sound(file: String)
fn set_animation(name: String)  
fn set_color(color: String)
fn get_player_name() -> String
fn has_backpack() -> Bool
```

## 编译您的脚本

使用 Mortar CLI 来编译您的脚本：

```bash
# 基本编译
mortar hello.mortar

# 用于调试的格式化输出
mortar hello.mortar --pretty

# 自定义输出文件
mortar hello.mortar -o dialogue.json
```

这会生成一个 JSON 文件，您的游戏可以解析和执行。

## 理解结构

### 节点
节点是对话的构建块。每个节点可以包含：
- **文本块**: 实际的对话内容
- **事件**: 在特定字符位置触发的动作
- **选择**: 玩家决策点
- **导航**: 跳转到其他节点

### 事件和索引
事件基于文本中的字符位置触发：
```mortar
text: "你好世界！"
events: [
    0, play_sound("hello.wav")    // 在 '你' 处触发
    2, set_color("red")           // 在 '世' 处触发
]
```

### 选择
使用 `choice` 字段创建分支对话：
```mortar
choice: [
    "选项 1" -> NextNode,
    "条件选项" when condition -> AnotherNode,
    "退出" -> return
]
```

## 接下来做什么？

- 了解更多[基本概念](./basic-concepts.md)
- 探索完整的[语法参考](./syntax-reference.md)  
- 查看更多[示例](./examples/basic-dialogue.md)
- 设置 [IDE 支持](./ide-support.md)以获得更好的开发体验

## JSON 输出示例

您的 Mortar 脚本编译为结构化 JSON，如下所示：

```json
{
  "nodes": {
    "Start": {
      "text_blocks": [
        {
          "content": "你好！欢迎来到这个互动故事。",
          "events": [
            {"index": 0, "action": "play_sound", "args": ["greeting.wav"]},
            {"index": 2, "action": "set_animation", "args": ["wave"]},
            {"index": 8, "action": "set_color", "args": ["#FF6B6B"]}
          ]
        }
      ],
      "next_node": "ChoiceDemo"
    }
  },
  "functions": {
    "play_sound": {"params": ["String"], "returns": null},
    "set_animation": {"params": ["String"], "returns": null}
  }
}
```

非常适合游戏引擎集成！