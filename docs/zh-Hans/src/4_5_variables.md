# 变量、常量与初始状态

Mortar v0.4 引入了脚本全局状态，让对话可以感知游戏进度。所有声明必须写在脚本的顶层（不在 `node` 或 `fn` 中），这样编译器才能在 `.mortared` 的 `variables` 区域里完整记录它们。

## 定义变量

使用 `let`，依次写名称、类型以及可选的初始值。目前支持 `String`、`Number`、`Bool` 三种基础类型：

```mortar
let player_name: String
let player_score: Number = 0
let is_live: Bool = true
```

规则提示：

- 没有 `null`，不赋值的话会提供默认值（空字符串、0、`false`）。
- 重新赋值发生在节点内部，例如 `player_score = player_score + 10`。
- 顶层声明有助于游戏端直接用哈希表/字典来同步所有变量。

## 公共常量（Key-Value 文本）

通过 `pub const` 定义 UI 文案、配置或跨语言共享的键值对：

```mortar
pub const welcome_message: String = "欢迎来到冒险！"
pub const continue_label: String = "继续"
pub const exit_label: String = "退出"
```

这些常量在 JSON 中会被标记为 `public`，方便本地化流水线或脚本系统读取。

## 在节点中使用

可以在节点里更新变量并引用它们：

```mortar
node AwardPoints {
    player_score = player_score + 5
    text: $"当前分数：{player_score}"
}
```

序列化后，这些赋值会记录在 `pre_statements` 或文本内容中，确保执行顺序与脚本一致。结合 [枚举](./4_10_enums.md) 与 [分支插值](./4_6_branch-interpolation.md) 可以实现更复杂的状态展示，同时保持 Mortar 的声明式特性。
