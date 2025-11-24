# 分支插值

Mortar 引入了 Fluent 风格的“非对称本地化”，通过 **branch 变量** 来管理状态化的文本片段。完整示例可参考 `examples/branch_interpolation.mortar`。

## 定义 branch 变量

branch 变量和普通 `let` 一样写在顶层，可由布尔值或枚举驱动：

```mortar
let is_forest: Bool = true
let is_city: Bool
let current_location: Location

enum Location {
    forest
    city
    town
}

let place: branch [
    is_forest, "森林"
    is_city, "城市"
]

let location: branch<current_location> [
    forest, "森林深处"
    city, "繁华都市"
    town, "宁静小镇"
]
```

当然，你也可以在节点内部写一次性的 `branch` 块，但集中定义能让翻译与 QA 更好维护。

## 节点中的用法

直接在插值字符串中引用 branch 变量：

```mortar
node LocationDesc {
    text: $"欢迎来到{place}！你现在处于{location}。"
}
```

序列化后，每个 branch 变量都会成为节点 JSON 中的 `branches` 条目，运行时代码按布尔或枚举值挑选对应文本。

## 分支事件

branch case 拥有自己的 `events` 列表。外层文本的 `with events` 只针对实际字符，而 branch 的事件索引会在占位符内部从 0 开始计数，这与 `examples/branch_interpolation.mortar` 的行为一致。

```mortar
text: $"你看向{object}，不禁倒吸一口气！"
with events: [
    1, set_color("#33CCFF")
    6, set_color("#FF6B6B")
]

let object: branch<current_location> [
    forest, "古树", events: [
        0, set_color("#228B22")
    ]
    city, "天际线", events: [
        0, set_color("#A9A9A9")
        1, set_color("#696969")
    ]
]
```

这样每个分支就能携带独立的演出需求，而无需复制整段节点内容。

## 与游戏逻辑协作

branch 适合用于：

- 布尔条件（如 `place`、`greet`）切换称谓。
- 枚举条件（如 `location`、`object`）切换整段描述。
- 在 `if/else` 或赋值后立即反映状态变化。

结合 [变量](./4_5_variables.md) 与 [控制流](./4_8_control-flow.md)，即可在保持脚本整洁的同时呈现丰富的本地化和剧情分支。
