# 节点：对话的积木块

节点（Node）是 Mortar 中最基本的单位，把它想象成对话中的一个"场景"或"片段"。

## 最简单的节点

```mortar
node OpeningScene {
    text: "你好，世界！"
}
```

就这么简单！一个节点需要：
- `node` 关键字（也可以简写成 `nd`）
- 一个名字（这里是 `OpeningScene`）
- 大括号 `{}` 里面的内容

## 节点命名规范

> **⚠️ 重要：推荐使用大驼峰命名法（PascalCase）**

**✅ 推荐的命名方式**：
```mortar
node OpeningScene { }       // 大驼峰：每个单词首字母大写
node ForestEntrance { }     // 清晰易读
node BossDialogue { }       // 见名知意
node Chapter1Start { }      // 可以包含数字
```

**⚠️ 不推荐的命名方式**：
```mortar
node 开场 { }              // 避免使用中文
node opening_scene { }    // 不要用蛇形命名（这是函数的风格）
node openingscene { }     // 全小写不易阅读
node opening-scene { }    // 不能使用短横线
node 1stScene { }         // 不能以数字开头
```

**命名建议**：
- 使用英文单词组合
- 每个单词首字母大写
- 名字要有意义，能够描述节点的用途
- 避免使用特殊字符和非ASCII字符
- 保持项目内命名风格一致

**技术原因**：
- 标准命名便于代码维护和团队协作
- 避免不同操作系统和编辑器的编码问题
- 与主流编程语言的命名习惯一致
- 便于在代码中引用和跳转

## 节点里能放什么？

一个节点可以包含：

### 1. 文本块

```mortar
node Dialogue {
    text: "这是第一句话。"
    text: "这是第二句话。"
    text: "还可以有第三句。"
}
```

多段文本会按顺序显示。

### 2. 事件列表

```mortar
node Dialogue {
    text: "你好呀！"
    events: [
        0, play_sound("hi.wav")
        3, show_smile()
    ]
}
```

### 3. 选项

```mortar
node 选择 {
    text: "你想去哪？"
    
    choice: [
        "森林" -> ForestScene场景,
        "城镇" -> TownScene场景
    ]
}
```

### 4. 混合使用

```mortar
node 完整示例 {
    // 第一段文字 + 事件
    text: "欢迎来到魔法学院！"
    events: [
        0, play_bgm("magic.mp3")
        7, sparkle()
    ]
    
    // 第二段文字
    text: "你准备好了吗？"
    
    // 让玩家做选择
    choice: [
        "准备好了！" -> 开始冒险,
        "再等等..." -> 等待
    ]
}
```

## 节点跳转

### 方式一：箭头跳转

在节点结束后用 `->` 指定下一个节点：

```mortar
node A {
    text: "这是节点 A"
} -> B  // 执行完 A 就跳到 B

node B {
    text: "这是节点 B"
}
```

### 方式二：通过选项跳转

```mortar
node 主菜单 {
    text: "选择一个选项："
    
    choice: [
        "选项 1" -> 节点1,
        "选项 2" -> 节点2
    ]
}
```

### 方式三：Return 退出

```mortar
node 结束 {
    text: "再见！"
    
    choice: [
        "退出" -> return  // 直接退出对话
    ]
}
```

## 节点的执行流程

让我们看一个例子：

```mortar
node 场景1 {
    text: "第一句"    // 1. 显示这个
    text: "第二句"    // 2. 显示这个
    
    choice: [        // 3. 显示选项
        "A" -> 场景2,
        "B" -> 场景3
    ]
    
    text: "选择后的话" // 4. 只有选了会中断的选项才到这里
} -> 场景4            // 5. 如果没有中断，最后跳这里
```

**重点**：
- 文本块按顺序执行
- 遇到 `choice` 时，玩家需要做选择
- 如果选项有 `return` 或 `break`，会影响后续流程
- 节点末尾的箭头是"默认出口"

## 实用模式

### 纯文本节点（没有选项）

```mortar
node 叙述 {
    text: "故事开始于一个黑暗的夜晚..."
    text: "突然，一声巨响！"
    text: "你决定去看看。"
} -> 下一场景
```

### 纯选项节点（没有文本）

```mortar
node 快速选择 {
    choice: [
        "向左" -> 左边,
        "向右" -> 右边
    ]
}
```

### 分段式对话

```mortar
node Dialogue {
    text: "嗨，很高兴见到你。"
    
    // 第一个选择点
    choice: [
        "你好" -> [
            "继续" -> 继续对话
        ],
        "再见" -> return
    ]
    
    text: "那么..."  // 只有选了"你好"才会看到
    text: "我们聊聊吧。"
}
```

## 常见问题

### Q: "NodeName"可以重复吗？
不行！每个"NodeName"必须唯一。

### Q: 节点顺序重要吗？
不重要。你可以先定义节点 B，后定义节点 A，只要跳转关系对就行。

### Q: 节点可以为空吗？
技术上可以，但没意义：
```mortar
node 空节点 {
}  // 编译器会警告你
```

### Q: 能从节点 A 跳回节点 A 吗？
可以！循环是允许的：
```mortar
node 循环 {
    text: "要再来一次吗？"
    
    choice: [
        "再来！" -> 循环,  // 跳回自己
        "不了" -> return
    ]
}
```

## 下一步

- 学习如何在节点中使用[文本与事件](./4_2_text-events.md)
- 了解更多[选项系统](./4_3_choices.md)的用法
- 看看[完整示例](./5_1_basic-dialogue.md)
