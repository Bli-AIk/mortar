# 五分钟上手

让我们动手写第一个 Mortar 对话！不用担心，这比你想象的简单。

## 第一步：创建文件

创建一个叫 `hello.mortar` 的文件（用任何文本编辑器都行）。

## 第二步：写一段简单对话

在文件里写下这些：

```mortar
// 这是注释，用来解释代码

node 开场 {
    text: "你好呀，欢迎来到这个互动故事！"
}
```

就这么简单！你已经写好了第一个对话节点。

**解释一下**：
- `node` 表示这是一个对话节点（也可以简写成 `nd`）
- `开场` 是这个节点的名字，你可以随便起名
- `text:` 后面跟着的就是对话内容
- 别忘了大括号 `{}`，它们把节点的内容包起来

## 第三步：加点音效

现在让我们让对话更生动一些：

```mortar
node 开场 {
    text: "你好呀，欢迎来到这个互动故事！"
    events: [
        // 在"你"字出现时播放音效
        0, play_sound("问候.wav")
        // 在"欢"字出现时显示动画
        4, show_animation("挥手")
    ]
}

// 告诉 Mortar 这些函数的"样子"
fn play_sound(文件名: String)
fn show_animation(动画名: String)
```

**解释一下**：
- `events:` 后面跟着事件列表，用方括号 `[]` 包起来
- `0, play_sound("问候.wav")` 表示：在第 0 个字符（也就是"你"）出现时，播放音效
- 数字是字符位置（从 0 开始数）
- 最下面的 `fn` 是函数声明，告诉编译器这些函数需要什么参数

## 第四步：添加多段对话

一个节点可以有好几段文字：

```mortar
node 开场 {
    text: "你好呀，欢迎来到这个互动故事！"
    events: [
        0, play_sound("问候.wav")
    ]
    
    // 第二段文字
    text: "我想你的名字是小明，对吧？"
    
    // 第三段文字
    text: "那我们开始吧！"
}
```

这三段文字会依次显示出来。

## 第五步：让玩家做选择

现在让玩家参与进来：

```mortar
node 开场 {
    text: "你想做什么呢？"
    
    choice: [
        "去森林探险" -> 森林,
        "回城里休息" -> 城镇,
        "我不玩了" -> return
    ]
}

node 森林 {
    text: "你勇敢地走进了森林..."
}

node 城镇 {
    text: "你回到了温暖的城镇。"
}
```

**解释一下**：
- `choice:` 表示这里有选项
- `"去森林探险" -> 森林` 意思是：显示"去森林探险"这个选项，选了就跳到名为"森林"的节点
- `return` 表示退出对话

## 第六步：编译文件

打开命令行（终端/CMD），输入：

```bash
mortar hello.mortar
```

这会生成一个 `hello.mortared` 文件，里面是 JSON 格式的数据，你的游戏可以读取它。

**想看看 JSON 长什么样？** 加上 `--pretty` 参数：

```bash
mortar hello.mortar --pretty
```

**想自定义输出文件名？** 用 `-o` 参数：

```bash
mortar hello.mortar -o 我的对话.json
```

## 完整示例

把刚才学的组合起来：

```mortar
node 开场 {
    text: "你好呀，欢迎来到魔法学院！"
    events: [
        0, play_sound("魔法音效.wav")
        7, sparkle_effect()
    ]
    
    text: $"你的名字是{get_player_name()}，对吧？"
    
    text: "准备好开始冒险了吗？"
    
    choice: [
        "当然！" -> 冒险开始,
        "让我再想想..." -> 犹豫,
        // 带条件的选项（只有有背包才显示）
        "先看看背包" when has_backpack() -> 查看背包
    ]
}

node 冒险开始 {
    text: "太好了！那我们出发吧！"
}

node 犹豫 {
    text: "没关系，慢慢来~"
}

node 查看背包 {
    text: "你的背包里有一些基础道具。"
}

// 函数声明
fn play_sound(file: String)
fn sparkle_effect()
fn get_player_name() -> String
fn has_backpack() -> Bool
```

**新东西解释**：
- `$"你的名字是{get_player_name()}，对吧？"` 这叫字符串插值，`{}` 里的函数会被替换成实际值
- `when has_backpack()` 表示这个选项有条件，只有 `has_backpack()` 返回真才显示
- `-> String` 和 `-> Bool` 表示函数的返回类型

## 接下来学什么？

- 想深入理解？看看[核心概念](./4_core-concepts.md)
- 想看更多例子？去[实战演练](./5_examples.md)
- 想了解所有功能？翻翻[函数](./4_4_functions.md)和[选项](./4_3_choices.md)

恭喜你！你已经学会 Mortar 的基础了 🎉