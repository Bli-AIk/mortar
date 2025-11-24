// ------
// 1.变量 —— 实现键值对解析

// 对接json方，需要维护哈希表/字典来存储变量名和值的映射关系

// 基本变量类型
// 我们没有所有权什么的东西，不必在意 mut
// 我们可以用 let 来定义变量
// let 只能定义在节点/方法外部
// 变量类型有三种：String、Number、Bool
// 格式是 let 变量名: 变量类型 = 值（值可选）
// 我们没有null的概念，变量必须赋值后才能使用
// 节点内可以给变量赋值，格式是 变量名 = 值

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
// 2.分支插值 —— 参考 examples/branch_interpolation.mortar

```mortar
let is_forest: Bool = true
let is_city: Bool

enum ExampleEnum {
    tree
    wall
}

// 建议在节点外部声明 branch 变量，便于集中管理
let place: branch [
    is_forest, "森林"
    is_city, "城市"
]

let object: branch<ExampleEnum> [
    tree, "古树"
    wall, "石墙"
]

node ForestScene {
    text: $"你进入了{place}，看见{object}。"
}
```

// 针对索引问题的解决方案：

```mortar
text: $"你看见{object}后，吓哭啦！"
with events: [
    1, set_color("#33CCFF")
    2, set_color("#424242")
    3, set_color("#FF6B6B")
]

// branch case 自带独立的 events，索引从占位符内部重新计数
let object: branch<ExampleEnum> [
    tree, "古树", events: [
        0, set_color("#228B22")
    ]
    wall, "石墙", events: [
        0, set_color("#A9A9A9")
        1, set_color("#696969")
    ]
]
```

fn set_color(color_code: String)

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
    // 如果希望 run 的效果跟随上一段文本，也可以写：
    text: "准备触发音效！"
    with run Basic with test_index

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
    now run PlaySound // now 关键字表示忽略 duration，立即执行下一个语句
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

// ------
// 6.枚举 —— 对 1 的补充

// 我们可以定义枚举。

enum GameState {
start
playing
game_over
}

// 但是，要使用枚举，我们需要定义一个枚举变量。

let current_state: GameState

// 以及一个branch类型，来根据枚举值选择文本。

let status: branch<current_state> [
    forest, "森林深处"
    city, "繁华都市"
    town, "宁静小镇"
]

// 然后，在实际使用中：
node Status {
    text: $"游戏状态是 {status}。"
}

// 请注意，所有变量应该定义在节点/方法外部……我们没有作用域的概念。
