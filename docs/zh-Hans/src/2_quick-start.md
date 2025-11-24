# 五分钟上手

让我们动手写第一个 Mortar 对话！不用担心，这比你想象的简单。

## 第一步：创建文件

创建一个叫 `hello.mortar` 的文件（用任何文本编辑器都行，你也可以先阅读 [编辑器支持](./6_2_ide-support.md) 学习如何配置编辑器）。

## 第二步：写一段简单对话

在文件里写下这些：

```mortar
// 这是注释，这一行内容会被编译器无视，不必担心！
// 我会在注释里为你解释 mortar 代码。

node StartScene {
    text: "你好呀，欢迎来到这个互动故事！"
}
```

就这么简单！你已经写好了第一个对话节点。

**解释一下**：
- `node` 声明一个对话节点（也可以简写成 `nd`）
- `StartScene` 是这个节点的名字（使用大驼峰命名法）
- `text:` 后面跟着的就是对话内容
- 别忘了大括号 `{}`，它们把节点的内容包起来

> **💡 命名规范提示**：
> - "NodeName"使用**大驼峰命名**（PascalCase），如 `StartScene`、`ForestPath`
> - 函数名使用**蛇形命名**（snake_case），如 `play_sound`、`get_player_name`
> - 我们建议仅使用 英文、数字、下划线 的组合作为标识符

## 第三步：加点音效

现在让我们的对话更生动一些。假设程序方和你已经沟通好了——我们要让每句话像打字机一样慢慢打印出来：

```mortar
node StartScene {
    text: "你好呀，欢迎来到这个互动故事！"
    with events: [
        // 在"你"字出现时播放音效
        0, play_sound("greeting.wav")
        // 在"欢"字出现时显示动画
        4, show_animation("wave")
    ]
}

// 告诉 Mortar 一共有哪些函数可以用
// 程序方需要在项目中绑定下面的函数
fn play_sound(file_name: String)
fn show_animation(anim_name: String)
```

**解释一下**：

* `with events:` 绑定到它上面的那条文本，事件列表用方括号 `[]` 包起来
* `0, play_sound("greeting.wav")` 表示：在第 0 个索引（我们用打字机关联索引，也就是"你"字）出现时，播放音效
* 数字就是“索引”，也就是字符的位置，从 0 开始计数。索引可以是小数（浮点数）
* 这里的索引不是死板的，有的游戏可能用打字机效果，有的游戏可能和语音对齐，所以具体怎么数完全看项目需求
* 最下面的 `fn` 是函数声明，告诉编译器这些函数需要什么参数
* 函数参数名建议用蛇形命名：`file_name`、`anim_name`

## 第四步：添加多段对话

一个节点可以有好几段文字：

```mortar
node StartScene {
    text: "你好呀，欢迎来到这个互动故事！"
    // ↕ 这个 事件列表 与其上方的 文本 绑定
    with events: [
        0, play_sound("greeting.wav")
    ]
    
    // 第二段文字
    text: "我想你的名字是 Ferris，对吧？"
    
    // 第三段文字
    text: "那我们开始吧！"
}
```

这三段文字会依次显示出来。其中，第一个文本带有事件，而后两个文本没有事件。

## 第五步：让玩家做选择

现在让玩家参与进来：

```mortar
node StartScene {
    text: "你想做什么呢？"
    
    choice: [
        "去森林探险" -> ForestScene,
        "回城里休息" -> TownScene,
        "我不玩了" -> return
    ]
}

node ForestScene {
    text: "你勇敢地走进了森林..."
}

node TownScene {
    text: "你回到了温暖的城镇。"
}
```

**解释一下**：
- `choice:` 表示这里有选项
- `"去森林探险" -> ForestScene` 意思是：显示"去森林探险"这个选项，选了就跳到名为 `ForestScene` 的节点
- "node"都使用大驼峰命名法的好处在这里就体现出来了――便于识别和维护！
- `return` 表示结束当前对话

## 第六步：编译文件

打开命令行（终端/CMD），输入：

```bash
mortar hello.mortar
```

这会生成一个 `hello.mortared` 文件，里面是 JSON 格式的数据，你的游戏可以读取它。

**显示 “命令未找到”？** 那说明你还没有安装 mortar 的编译器！请阅读 [安装工具](./3_installation.md) 来安装它。

**JSON 缩成一行了？** 加上 `--pretty` 参数：

```bash
mortar hello.mortar --pretty
```

**想自定义输出文件名和文件后缀？** 用 `-o` 参数：

```bash
mortar hello.mortar -o 我的对话.json
```

## 完整示例

把刚才学的组合起来，再“加点细节”：

```mortar
node WelcomeScene {
    text: "你好呀，欢迎来到魔法学院！"
    with events: [
        0, play_sound("magic_sound.wav")
        7, sparkle_effect()
    ]
    
    text: $"你的名字是{get_player_name()}，对吧？"
    
    text: "准备好开始冒险了吗？"
    
    choice: [
        "当然！" -> AdventureStart,
        "让我再想想..." -> Hesitate,
        // 带条件的选项（只有有背包才显示）
        "先看看背包" when has_backpack() -> CheckInventory
    ]
}

node AdventureStart {
    text: "太好了！那我们出发吧！"
}

node Hesitate {
    text: "没关系，慢慢来~"
}

node CheckInventory {
    text: "你的背包里有一些基础道具。"
}

// 函数声明
fn play_sound(file: String)
fn sparkle_effect()
fn get_player_name() -> String
fn has_backpack() -> Bool
```

**新东西解释**：
- `$"你的名字是{get_player_name()}，对吧？"` 这叫字符串插值，`{}` 里的内容会被替换成函数的返回值
- `when` 表示这个选项有条件，只有 `has_backpack()` 返回 true，才“显示”
- `-> String` 和 `-> Bool` 表示函数的返回类型。mortar 会进行静态类型检测，以防止类型混用！

## 接下来学什么？

- 想深入理解？看看[核心概念](./4_0_core-concepts)
- 想看更多例子？去[实战演练](./5_0_examples)
- 想了解所有功能？翻翻[函数](./4_4_functions.md)和[选项](./4_3_choices.md)

恭喜你！你已经学会 Mortar 的基础了 🎉
