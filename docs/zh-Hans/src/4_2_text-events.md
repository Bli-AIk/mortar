# 文本与事件：分离的艺术

这是 Mortar 最与众不同的地方：**文本和事件分开写，但精确关联**。

## 为什么要分离？

想象你在写带事件的游戏对话，传统方式可能是：

```
"你好<sound=hi.wav>，欢迎<anim=wave>来到<color=red>这里</color>！"
```

问题来了：

- 😰 写手看到一堆“标记”，难以专注于文字本身
- 😰 程序员要解析复杂的标记，容易出错
- 😰 事件增减参数相当麻烦

Mortar 的方式：

```mortar
text: "你好，欢迎来到这里！"
with events: [
    0, play_sound("hi.wav")
    3, show_animation("wave")
    8, set_color("red")
    9, set_color("normal")
]
```

干净！清晰！好维护！

## 文本块基础

### 最简单的文本

```mortar
node Example {
    text: "这是一段文本。"
}
```

### 多段文本

```mortar
node Dialogue {
    text: "第一句话。"
    text: "第二句话。"
    text: "第三句话。"
}
```

它们会按顺序显示。

### 引号的使用

单引号和双引号都可以：

```mortar
text: "双引号"
text: '单引号'
```

### 转义序列

Mortar 支持标准的转义序列：

| 转义序列 | 字符  |
|------|-----|
| `\n` | 换行符 |
| `\t` | 制表符 |
| `\r` | 回车符 |
| `\\` | 反斜杠 |
| `\"` | 双引号 |
| `\'` | 单引号 |
| `\0` | 空字符 |

**示例：**

```mortar
node Dialogue {
    text: "第一行\n第二行"           // 两行文本
    text: "名字:\t小明"              // 带制表符
    text: "她说\"你好！\""           // 带引号
    text: "路径: C:\\Users\\XiaoMing"   // 带反斜杠
}
```

## 事件系统

### 基本语法

```mortar
with events: [
    索引, 函数调用
    索引, 函数调用
]
```

索引，从 0 开始计数，类型为 Number，也就是支持整数和小数（浮点数）。

索引具体的使用方式取决于程序方的实现。

### 简单示例

以字符索引为例：

```mortar
text: "你好世界！"
with events: [
    0, sound_a()  // 在"你"字
    2, sound_b()  // 在"世"字  
    4, sound_c()  // 在"！"
]
```

**字符索引**：

- "你" = 位置 0
- "好" = 位置 1
- "世" = 位置 2
- "界" = 位置 3
- "！" = 位置 4

### 链式调用

可以在同一个位置调用多个函数：

```mortar
with events: [
    0, play_sound("boom.wav").shake_screen().flash_white()
]
```

或者分开写：

```mortar
with events: [
    0, play_sound("boom.wav")
    0, shake_screen()
    0, flash_white()
]
```

两种方式效果一样。

## 小数索引

索引可以是小数，这在语音同步时特别有用：

```mortar
text: "你好，世界！"
with events: [
    0.0, start_voice("hello.wav")   // 开始播放语音
    1.5, blink_eyes()               // 1.5秒时眨眼
    3.2, show_smile()               // 3.2秒时微笑
    5.0, stop_voice()               // 5秒时结束
]
```

**什么时候用小数？**

我们的建议是：

- 打字机效果：用整数（一个字一个触发）
- 语音同步：用小数（按时间轴触发）
- 视频同步：用小数（精确到帧）

## 字符串插值

想在文本中插入变量或函数返回值？用 `$` 和 `{}`：

```mortar
text: $"你好，{get_player_name()}！"
text: $"你有 {get_gold()} 金币。"
text: $"今天是 {get_date()}。"
```

**注意**：

- 字符串前面要加 `$`，来声明“可插值字符串”
- 变量/函数放在 `{}` 里
- 函数要提前声明

## 实战示例

### 打字机效果配音效

```mortar
node 打字机 {
    text: "叮！叮！叮！"
    with events: [
        0, play_sound("ding.wav")  // 第一个"叮"
        2, play_sound("ding.wav")  // 第二个"叮"
        4, play_sound("ding.wav")  // 第三个"叮"
    ]
}
```

### 旁白配背景音乐

```mortar
node 旁白 {
    text: "在一个遥远的王国..."
    with events: [
        0, fade_in_bgm("story_theme.mp3")
        0, dim_lights()
    ]
    
    text: "住着一位勇敢的骑士。"
}
```

### 语音同步动画

```mortar
node Dialogue {
    text: "我要告诉你一个秘密..."
    with events: [
        0.0, play_voice("secret.wav")
        0.0, set_expression("serious")
        2.5, lean_closer()
        4.0, whisper_effect()
        6.0, set_expression("normal")
    ]
}
```

## 事件函数声明

用到的所有函数都要先声明：

```mortar
// 在文件末尾声明
fn play_sound(file: String)
fn shake_screen()
fn flash_white()
fn set_expression(expr: String)
fn get_player_name() -> String
fn get_gold() -> Number
```

详见[函数：连接游戏世界](./4_4_functions.md)。

## 最佳实践

### ✅ 好的做法

```mortar
// 清晰的结构
text: "你好，世界！"
with events: [
    0, greeting_sound()
    2, sparkle_effect()
]
```

### ❌ 错误的做法

```mortar
text: "你好"
text: "世界"
with events: [
    0, say_hello()  // 关联的文本不对！
]
```

### 建议

1. **紧跟原则**：events 紧跟在对应的 text 后面
2. **适度使用**：不是每句话都需要事件
3. **有序排列**：事件按位置从小到大写（虽然不强制）
4. **合理命名**：函数名要见名知意

## 常见问题

### Q: 位置超出文本长度会怎样？

编译器会警告，但不会报错。运行时行为由你的游戏决定。

### Q: 可以没有 events 吗？

当然可以！不是每段文本都需要事件。但是事件都需要依附于文本。

```mortar
text: "这是纯文本。"
// 没有 events，完全没问题
```

### Q: 多个事件在同一位置的执行顺序？

按写的顺序执行：

```mortar
with events: [
    0, first()   // 先执行
    0, second()  // 再执行
    0, third()   // 最后执行
]
```

## 下一步

- 了解[选项系统](./4_3_choices.md)
- 学习[函数声明](./4_4_functions.md)
- 看[完整示例](./5_1_basic-dialogue.md)
