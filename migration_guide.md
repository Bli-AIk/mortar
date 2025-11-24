### Mortar 编译器 JSON 输出重构迁移指南

#### 1. 简介与目的

为了提升 `mortar` 引擎输出格式的清晰度和易用性，我们对 `*.mortared` JSON 文件的核心结构进行了重构。在旧版本中，一个节点（`Node`）的内容被分散在 `texts`、`runs` 和 `choice` 等多个平级字段中，需要客户端进行复杂的合并与排序。

**新版本将这些字段统一到了一个单一的、有序的 `content` 数组中。**

这一改动使得客户端（如游戏引擎）的解析逻辑**大大简化**。您不再需要手动处理事件和选项的插入位置，只需按顺序遍历 `content` 数组即可。

#### 2. 核心变更：`content` 数组

这是本次重构最核心的变化。

**旧结构** 示例：
```json
{
  "name": "Start",
  "texts": [ { "text": "欢迎！" } ],
  "runs": [ { "event_name": "PlayMusic", "position": 0 } ],
  "choice": [ { "text": "开始", "next": "NextNode" } ],
  "choice_position": 1
}
```

**新结构** 将上述逻辑表示为：
```json
{
  "name": "Start",
  "content": [
    {
      "type": "run_event",
      "name": "PlayMusic"
    },
    {
      "type": "text",
      "value": "欢迎！"
    },
    {
      "type": "choice",
      "options": [
        { "text": "开始", "next": "NextNode" }
      ]
    }
  ]
}
```
所有流程都按执行顺序被整合进了 `content` 数组，并通过 `type` 字段加以区分。

#### 3. `content` 内容项详解

`content` 数组中的每一项都是一个对象，包含一个 `type` 字段。主要有以下几种类型：

*   **`type: "text"`**:
    *   **描述**: 代表一个对话文本行。
    *   **主要字段**:
        *   `value` (string): 文本内容 (替换了旧的 `text` 字段)。
        *   `events` (array, optional): 与旧结构一致的行内事件。
        *   `condition` (object, optional): 当文本位于 `if/else` 分支中时出现。
        *   `pre_statements` (array, optional): 在显示文本前执行的赋值语句。

*   **`type: "run_event"`**:
    *   **描述**: 执行一个在根级 `events` 中定义的事件。
    *   **主要字段**:
        *   `name` (string): 事件名称 (替换了 `event_name`)。
        *   `index_override` (object, optional): 覆盖事件的默认 `index`。

*   **`type: "run_timeline"`**:
    *   **描述**: 执行一个在根级 `timelines` 中定义的时间线。
    *   **主要字段**:
        *   `name` (string): 时间线名称。

*   **`type: "choice"`**:
    *   **描述**: 向玩家呈现一个或多个选项。
    *   **主要字段**:
        *   `options` (array): 包含所有选项对象的数组。每个选项的内部结构（`text`, `next`, `condition` 等）保持不变。

#### 4. 客户端代码迁移步骤

您需要更新项目中解析 `*.mortared` 文件的代码。

##### 步骤 1: 更新数据结构

首先，修改您项目中用于表示“节点”的结构体。移除 `texts`、`runs`、`choice` 和 `choice_position` 字段，并添加一个新的 `content` 字段。

由于 `content` 是一个多态数组（包含不同类型的对象），您可以使用 `serde_json::Value` 来反序列化它，然后在运行时根据 `type` 字段进行匹配。

**示例 (Rust)**

```rust
// --- 旧的数据结构 (简化) ---
// #[derive(Deserialize, Debug)]
// pub struct Node {
//     pub name: String,
//     pub texts: Vec<Text>,
//     pub runs: Vec<RunStmt>,
//     #[serde(default)]
//     pub choice: Option<Vec<Choice>>,
//     #[serde(default)]
//     pub choice_position: Option<usize>,
// }

// --- 新的数据结构 ---
use serde::{Deserialize, Serialize};
use serde_json::Value; // 用于反序列化多态 content

#[derive(Deserialize, Serialize, Debug, Clone)] // 根据您的需求添加更多 derive 宏
pub struct Node {
    pub name: String,
    #[serde(default)]
    pub content: Vec<Value>, // 将 content 反序列化为 serde_json::Value 的列表
    // ... 其他可能仍然存在的字段，如 branches, variables, next
}

// 您也可以直接反序列化为定义好的枚举（如果 ContentItem 是公开的）
// use mortar_compiler::{ContentItem};
// #[derive(Deserialize, Debug>)
// pub struct Node {
//     pub name: String,
//     pub content: Vec<ContentItem>, // 如果 ContentItem 是公开可用的
//     // ...
// }
```

##### 步骤 2: 更新解析逻辑

这是最关键的一步。您需要重写处理节点内容的逻辑。之前您可能需要复杂的代码来交错处理 `texts` 和 `runs`，现在只需一个简单的循环和模式匹配。

**伪代码示例 (Rust)：**

```rust
fn process_node(node: &Node) {
    println!("处理节点: {}", node.name);

    for item_value in &node.content {
        // 尝试获取 type 字段，并根据其值进行匹配
        if let Some(item_type) = item_value["type"].as_str() {
            match item_type {
                "text" => {
                    if let Some(value) = item_value["value"].as_str() {
                        println!("  文本: {}", value);
                    }
                    // (可选) 处理 item_value["events"]
                    // 例如: if let Some(events) = item_value["events"].as_array() { ... }
                },
                "run_event" => {
                    if let Some(name) = item_value["name"].as_str() {
                        println!("  运行事件: {}", name);
                    }
                    // (可选) 处理 item_value["args"], item_value["index_override"]
                },
                "run_timeline" => {
                    if let Some(name) = item_value["name"].as_str() {
                        println!("  运行时间线: {}", name);
                    }
                },
                "choice" => {
                    println!("  选项:");
                    if let Some(options) = item_value["options"].as_array() {
                        for (i, choice) in options.iter().enumerate() {
                            if let Some(text) = choice["text"].as_str() {
                                println!("    {}. {}", i + 1, text);
                            }
                        }
                    }
                    // 在实际游戏中，这里通常会等待玩家输入，并根据选择跳转
                    // return; // 示例中此处结束流程
                },
                _ => {
                    println!("  未知内容类型: {}", item_type);
                }
            }
        }
    }

    // 如果流程未被 choice 中断，处理默认跳转
    if let Some(next_node_name) = &node.next {
        println!("  默认跳转到: {}", next_node_name);
        // goToNode(next_node_name);
    }
}
```

#### 5. 总结

这次重构旨在使您的开发工作更加轻松和直观。通过将所有节点内容线性化，解析和执行对话流的逻辑变得前所未有的简单。我们建议您尽快更新您的项目以利用这一改进。

如果您在迁移过程中遇到任何问题，请随时提出。
