# 文本与事件：分离的艺术

这是 Mortar 最与众不同的地方：**文本和事件分开写，但精确关联**。

## 为什么要分离？

想象你在写游戏对话，传统方式可能是：

```
"你好<sound=hi.wav>，欢迎<anim=wave>来<color=red>到<color=normal>这里！"
```

问题来了：
- 😰 写手看到一堆代码，难以专注于文字本身
- 😰 程序员要解析复杂的标记，容易出错
- 😰 改一个字就要调整所有位置

Mortar 的方式：

```mortar
text: "你好，欢迎来到这里！"
events: [
    0, play_sound("hi.wav")
    3, show_animation("wave")
    5, set_color("red")
    6, set_color("normal")
]
```

干净！清晰！好维护！

## 文本块基础

### 最简单的文本

```mortar
node 示例 {
    text: "这是一段文本。"
}
```

### 多段文本

```mortar
node 对话 {
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

如果文本里有引号，用另一种包起来：

```mortar
text: "他说：'你好！'"
text: '她说："再见！"'
```

## 事件系统

### 基本语法

```mortar
events: [
    位置, 函数调用
    位置, 函数调用
]
```

**位置**就是字符的索引，从 0 开始数。

### 简单示例

```mortar
text: "你好世界！"
events: [
    0, sound_a()  // 在"你"字
    2, sound_b()  // 在"世"字  
    4, sound_c()  // 在"！"
]
```

**字符计数**：
- "你" = 位置 0
- "好" = 位置 1
- "世" = 位置 2
- "界" = 位置 3
- "！" = 位置 4

### 链式调用

可以在同一个位置调用多个函数：

```mortar
events: [
    0, play_sound("boom.wav").shake_screen().flash_white()
]
```

或者分开写：

```mortar
events: [
    0, play_sound("boom.wav")
    0, shake_screen()
    0, flash_white()
]
```

两种方式效果一样。

## 小数位置（高级）

位置可以是小数，这在语音同步时特别有用：

```mortar
text: "你好，世界！"
events: [
    0.0, start_voice("hello.wav")   // 开始播放语音
    1.5, blink_eyes()               // 1.5秒时眨眼
    3.2, show_smile()               // 3.2秒时微笑
    5.0, stop_voice()               // 5秒时结束
]
```

**什么时候用小数？**
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
- 字符串前面要加 `$`
- 变量/函数放在 `{}` 里
- 函数要提前声明（见后面）

### 混合使用

插值文本也可以配事件：

```mortar
text: $"欢迎，{get_name()}！"
events: [
    0, play_fanfare()
    3, show_confetti()
]
```

## 实战示例

### 打字机效果配音效

```mortar
node 打字机 {
    text: "叮！叮！叮！"
    events: [
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
    events: [
        0, fade_in_bgm("story_theme.mp3")
        0, dim_lights()
    ]
    
    text: "住着一位勇敢的骑士。"
}
```

### 语音同步动画

```mortar
node 语音对话 {
    text: "我要告诉你一个秘密..."
    events: [
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
events: [
    0, greeting_sound()
    2, sparkle_effect()
]
```

### ❌ 不好的做法

```mortar
// 不要把事件和文本离得太远
text: "你好"
text: "世界"
events: [
    0, some_sound()  // 这是哪个 text 的？？
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
当然可以！不是每段文本都需要事件。

```mortar
text: "这是纯文本。"
// 没有 events，完全没问题
```

### Q: 多个事件在同一位置的执行顺序？
按写的顺序执行：

```mortar
events: [
    0, first()   // 先执行
    0, second()  // 再执行
    0, third()   // 最后执行
]
```

### Q: 中文字符怎么计数？
每个中文字符算 1 个位置，和英文字母一样。

## 下一步

- 了解[选项系统](./4_3_choices.md)
- 学习[函数声明](./4_4_functions.md)
- 看[完整示例](./5_1_basic-dialogue.md)
