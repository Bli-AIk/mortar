# 枚举与结构化状态

枚举是变量系统的重要补充，用来表示一组封闭的状态（章节、好感度、天气等）。它与 [分支插值](./4_6_branch-interpolation.md) 和 `if` 搭配时尤其强大。

## 声明枚举

在顶层使用 `enum`：

```mortar
enum GameState {
    start
    playing
    game_over
}
```

所有枚举值都会出现在 `.mortared` 的 `enums` 数组里，引擎可以直接校验或生成对应的原生枚举。

## 定义枚举变量

```mortar
let current_state: GameState
```

节点内部可随时赋值：

```mortar
node StateMachine {
    if boss_defeated() {
        current_state = game_over
    }
}
```

## 结合分支

利用 `branch<current_state>` 可以把枚举直接映射成文本：

```mortar
let status: branch<current_state> [
    start, "刚刚启程"
    playing, "冒险途中"
    game_over, "剧情完结"
]

node Status {
    text: $"游戏状态是 {status}。"
}
```

## 引擎对接

1. 加载 `enums` 后建立注册表，或生成对应的原生枚举。
2. 将 Mortar 变量（如 `current_state`）与游戏内状态同步。
3. 借助赋值或函数调用，把结果反馈回脚本。

这样既能保持设计意图清晰，又能确保状态流转始终合法。
