# 事件系统与时间线

事件一直是 Mortar 的核心。v0.4 将它们扩展为可复用的命名实体，并通过时间线进行编排。以下内容与 `examples/performance_system.mortar` 保持一致，建议配套阅读。

## 声明事件

顶层 `event` 用于描述可复用动作：

```mortar
event SetColor {
    index: 0
    action: set_color("#228B22")
}

event MoveRight {
    action: set_animation("right")
    duration: 2.0
}

event MoveLeft {
    action: set_animation("left")
    duration: 1.5
}

event PlaySound {
    index: 20
    action: play_sound("dramatic-hit.wav")
}
```

每个事件可以设置默认 `index`、动作以及可选的 `duration`，并会写入 JSON 顶层的 `events`。

## 在节点中运行事件

常见的两种方式：

1. **`run EventName`**：立即执行事件，忽略默认 index（但会尊重 duration）。
2. **`with EventName` 或 `with events: [ ... ]`**：把事件绑定到上一段文本，索引基于文本字符。

```mortar
node WithEventsExample {
    text: "欢迎来到冒险！"
    with SetColor

    text: "森林被声音与色彩唤醒。"
    with events: [
        MoveRight
        PlaySound
    ]
}
```

序列化后，这些绑定都会出现在对应文本的 `events` 数组里。

## 自定义索引

通过 `run EventName with <数值或变量>` 可以临时覆盖事件的触发位置；在 `with` 前加 `run` 则表示“先运行，再把结果附着到上一段文本”：

```mortar
let custom_time: Number = 5

node CustomIndexExample {
    text: "安静……直到爆炸声响起！"
    run PlaySound with custom_time

    custom_time = 28
    with run PlaySound with custom_time
}
```

这些语句在 `.mortared` 中会生成带 `index_override` 的 `ContentItem::RunEvent`。

## 时间线（Timeline）

时间线由 `run`、`wait` 和 `now run` 组成：

```mortar
timeline OpeningCutscene {
    run MoveRight
    wait 1.0
    run MoveLeft
    wait 0.5
    now run PlaySound   // 忽略事件自身的 duration
    wait 10
    run SetColor
}
```

节点中直接 `run OpeningCutscene` 即可播放整条时间线，非常适合开场动画或复杂演出。

## 实用建议

- 把频繁使用的演出封装成事件，减少重复。
- 需要和文本对齐时使用 `with`，需要即时动作时使用 `run`。
- `now run` 可以跳过事件的 duration，便于在时间线里快速切换。
- 通过覆盖 index，让同一个事件在不同上下文拥有不同节奏。

v0.4 JSON 同时输出 `events` 和 `timelines`，方便工具链和游戏引擎进行可视化或复用。
