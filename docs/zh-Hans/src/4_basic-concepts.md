# 基本概念

理解这些核心概念将帮助您编写有效的 Mortar 脚本。

## 节点

节点是 Mortar 对话的基本构建块。将它们视为场景或对话片段。

```mortar
node WelcomeScene {
    text: "欢迎来到我们的酒馆！"
    
    // 节点可以连接到其他节点
} -> MainMenu
```

**主要特征：**
- 每个节点都有唯一标识符
- 节点包含文本、事件和/或选择
- 节点可以跳转到其他节点或返回/中断

## 文本和事件

Mortar 的核心理念是将干净的文本与交互事件分离。

### 文本块
文本块包含您的对话内容：

```mortar
node Example {
    text: "你好，旅行者！"
    text: "什么带你来到这片土地？"
}
```

### 事件系统
事件在特定字符位置触发：

```mortar
node Example {
    text: "龙发出响亮的咆哮！"
    events: [
        0, play_sound("dragon_roar.wav")  // 在"龙"字符处
        2, screen_shake(intensity: 3)     // 在"出"字符处
    ]
}
```

**事件索引从 0 开始**并计算 Unicode 字符，而不是字节。

## 字符串插值

使用 `$` 前缀创建带有函数调用的动态文本：

```mortar
node Greeting {
    text: $"欢迎回来，{get_player_name()}！"
    text: $"你有 {get_gold_count()} 个金币。"
}
```

## 选择

使用 `choice` 字段创建分支对话：

```mortar
node DecisionPoint {
    text: "你选择哪条路？"
    
    choice: [
        "走森林小径" -> ForestPath,
        "沿着河流" -> RiverPath,
        "返回" -> return
    ]
}
```

### 条件选择
选择可以使用 `when` 关键字添加条件：

```mortar
choice: [
    "使用魔法咒语" when has_magic -> CastSpell,
    "用剑攻击" when has_sword -> SwordAttack,
    "尝试谈判" -> Negotiate
]
```

## 函数声明

声明您的游戏将实现的函数：

```mortar
// 音频函数
fn play_sound(filename: String)
fn stop_music()

// 视觉效果
fn set_color(hex_color: String)
fn screen_shake(intensity: Number)

// 游戏状态查询
fn get_player_name() -> String
fn has_magic() -> Bool
fn get_gold_count() -> Number
```

**支持的类型：**
- `String` - 文本数据
- `Number` - 数值（整数和浮点数）
- `Bool` / `Boolean` - 真/假值

## 导航

使用导航关键字控制对话流程：

### 跳转到节点
```mortar
node A {
    text: "移动到场景 B"
} -> SceneB
```

### 返回
退出当前节点或选择块：
```mortar
choice: [
    "离开对话" -> return
]
```

### 中断
停止处理当前选择列表：
```mortar
choice: [
    "我不知道" -> break
]
// 中断后继续执行这里
text: "让我想想..."
```

## 注释

使用 `//` 进行单行注释，使用 `/* */` 进行多行注释：

```mortar
// 这是单行注释
node Example {
    /* 
     * 多行注释
     * 用于详细说明
     */
    text: "你好！"  // 行内注释
}
```

## 数据流

以下是 Mortar 处理您脚本的方式：

1. **解析**: 您的 `.mortar` 文件被标记化和解析
2. **验证**: 类型检查和引用验证  
3. **编译**: 生成结构化 JSON 输出
4. **执行**: 您的游戏加载并解释 JSON

## 最佳实践

- **保持节点专注**: 每个节点应代表单个对话时刻
- **使用描述性名称**: 节点和函数名称应该清晰
- **逻辑组织事件**: 将相关事件分组在它们影响的文本附近
- **注释复杂逻辑**: 解释条件选择和事件时序

## 下一步

现在您了解了基础知识：
- 深入了解[语法参考](./syntax-reference.md)
- 学习[高级特性](./advanced-features.md)
- 查看实用[示例](./examples/basic-dialogue.md)