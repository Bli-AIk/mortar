// ------
// 1.变量 —— 实现键值对解析

// 对接json方，需要维护哈希表/字典来存储变量名和值的映射关系

// 基本变量类型
// 我们没有所有权什么的东西，不必在意 mut
// 我们可以用 let 来定义变量

let player_name: String
let player_score: Number
let is_live: Bool

// 复杂变量类型
// 目前仅支持枚举
enum GameState {
    start
    playing
    game_over
}

// 非对话文本的键值对通过 pub const xxx: String 变量来实现
// pub / public 表示这些变量是公共的，可以被外部访问
// const 表示这些变量是常量，值不会改变
pub const welcome_message: String = "欢迎来到游戏！"
pub const game_over_message: String = "游戏结束！"
pub const continue: String = "继续"
pub const exit: String = "退出"

// ------
// 2.分支插值 —— 参考自 fluent 的 非对称本地化 设计

```mortar
let is_forest: Bool
let is_city: Bool

node ForestScene {
    text: $"你进入了 {place}，你看见了 {object} 。"

    place: branch [
        is_forest, 森林
        is_city, 城市
    ]

    object: branch<ExampleEnum> [
        tree, 树
        wall, 墙壁
    ]
}

enum ExampleEnum {
    tree
    wall
}
```

// 针对索引问题的解决方案：

```mortar
node ForestScene {
    // events 只针对非插值文本。
    // object 作为插值，不会接受任何索引。
    text: $"你看见{object}后，吓哭啦！"
    events: [
            1, set_color("#33CCFF") // 对应文字“看”
            2, set_color("#424242") // 对应文字“见”
            3, set_color("#FF6B6B") // 对应文字“后”，而不是 {object}。
        ]

    // 而插值文本需要单独设计其对应的 events
    object: branch<ExampleEnum> [
        tree, 树, events: [
        0, set_color("#228B22") // 对应文字“树”
        ]
        wall, 墙壁, events: [
        0, set_color("#A9A9A9") // 对应文字“墙”
        1, set_color("#696969") // 对应文字“壁”
        ]
    ]
}

enum ExampleEnum {
    tree
    wall
}

fn set_color(color_code: String)
```


// ------
// 3.本地化
使用多个语言文件来实现。外部库根据语言代码文件夹加载不同的文本资源。

// ------
// 4.控制流
```mortar
let player_score: Number = 123

node ExampleNode {
    if player_score > 100 {
    text: "满分！。"
    } else {
    text: "你还得加把劲。"
    }
}
```

// ------
// 5.演出系统

// 演出系统的主旨就是让 events 变得更加独立和强大。
// 参考 Unity Timeline 的 clip 或者 Ren’Py 的 scene、show、play 等命令：

```mortar
//events的本质是一系列独立的事件（event）

event Basic{
    index: 0,
    action: set_background("bg_forest.png")
}

// 我们可以用 run 关键字调用它。

node ExampleNode {
    run Basic
}

// 当我们单独调用它时，它无视 index 而直接运行。
// 当我们让它依附于 字符串 时，它会根据 index 来排序执行。

node ExampleNodeWithGenerics {
    text: "欢迎来到游戏！"
    // 我们加入了 with 关键字。现在，必须加入with才能让 events 关联上方的 text。
    with events: [
        Basic // 这里会根据 Basic 的 index 插入到合适的位置
    ]

    text: "准备好开始冒险了吗？"
    with Basic // 我们可以 with 单独一个 event

    text: "让我们出发吧！"
    run Basic // 直接运行的话会无视 index

    let test_index: Number = 1
    run Basic with test_index // 也可以指定 index （为变量）

    run Basic with ref test_index // 这样的话，传入的值就不是死的值，而是引用，可以动态变化。

    // 简单来说，run 是直接运行， with 是关联到文本并根据 index 排序执行。
    // 如果在最后 with 了一个 Number，它会被视为 index。
}

// 我们引入了一个与 fn 和 node 类似的 新概念 —— timeline / tl。
// timeline 是一系列 event 的有序集合，类似于函数调用。
// 我们通过它来实现更复杂的演出逻辑。
timeline IntroScene {
    run ShowAlice
    wait 2.0
    run PlayMusic
    run DialogueNode("Start")
}


event ShowAlice {
    action: show_character("alice.png", position: "center")
    duration: 2.0 // 持续时间，相当于 wait 2.0。这也会用于 events
}

event PlayMusic {
    action: play_music("intro_theme.mp3", volume: 0.8)
    duration: 0.0
}

fn show_character(image: String, position: String)
fn play_music(file: String, volume: Number)
```
