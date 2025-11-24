# 常见问题解答

在使用 Mortar 的过程中，你可能会遇到一些疑问。这里整理了最常见的问题和解答。

## 基础概念

### Mortar 和其他对话系统有什么区别？

**最大的区别是"内容与逻辑分离"**：

- **传统系统**：`"你好<sound=hi.wav>，欢迎<color=red>光临</color>！"`
- **Mortar**：文本是纯文本，事件单独写，通过位置关联

这样做的好处：
- 写手可以专心写故事，不用管技术标记
- 程序员可以灵活控制事件，不会破坏文本
- 文本内容容易翻译和修改

### 为什么要用字符位置来触发事件？

字符位置让你能**精确控制**事件触发时机：

```mortar
text: "轰隆隆！一道闪电划过天空。"
with events: [
    0, shake_screen()      // 在"轰"字时屏幕震动
    3, flash_effect()      // 在"！"时闪光效果
    4, play_thunder()      // 在"一"字时雷声
]
```

这对于：
- 打字机效果（逐字显示）
- 语音同步
- 音效配合

都特别有用！

### 我可以不用事件，只写对话吗？

**当然可以！** 事件是可选的：

```mortar
node SimpleDialogue {
    text: "你好！"
    text: "欢迎来玩！"
    
    choice: [
        "谢谢" -> Thanks,
        "拜拜" -> return
    ]
}
```

这样写完全合法，适合简单场景。

## 语法相关

### 分号和逗号必须写吗？

**大部分情况下可以省略！** Mortar 语法很宽松：

```mortar
// 这三种写法都可以
text: "你好"
text: "你好",
text: "你好";

with events: [
    0, sound_a()
    1, sound_b()
]

with events: [
    0, sound_a(),
    1, sound_b(),
]

with events: [
    0, sound_a();
    1, sound_b();
]
```

但建议**保持一致**，选一种风格坚持下去。

### 字符串必须用双引号吗？

**单引号和双引号都可以：**

```mortar
text: "双引号字符串"
text: '单引号字符串'
```

### node 和 nd 有什么区别？

**完全一样！** `nd` 只是 `node` 的简写：

```mortar
node OpeningScene { }
nd 开场 { }      // 完全相同
```

类似的简写还有：
- `fn` = `function`
- `Bool` = `Boolean`

### 怎么写注释？

用 `//` 写单行注释，用 `/* */` 写多行注释：

```mortar
// 这是单行注释

/*
这是
多行
注释
*/

node Example {
    text: "对话内容"  // 也可以写在行尾
}
```

## 节点与跳转

### 节点名字有什么要求？

**技术上**可以使用：
- 英文字母、数字、下划线
- 但不能以数字开头

**但我们强烈推荐使用大驼峰命名法（PascalCase）**：

```mortar
// ✅ 推荐的命名（大驼峰）
node OpeningScene { }
node ForestEntrance { }
node BossDialogue { }
node Chapter1Start { }

// ⚠️ 不推荐但能用
node opening_scene { }  // 蛇形是函数的风格
node forest_1 { }       // 可以，但不如 Forest1

// ❌ 不好的命名
node 开场 { }           // 避免使用 非 ASCII 文本
node 1node { }         // 不能以数字开头
node node-1 { }        // 不能用短横线
```

**为什么推荐大驼峰？**
- 与主流编程语言的类型命名一致
- 清晰易读，便于识别
- 避免跨平台编码问题
- 团队协作更规范

### 可以跳转到不存在的节点吗？

**不行！** 编译器会检查所有的跳转：

```mortar
node A {
    choice: [
        "去B" -> B,      // ✅ B存在，可以
        "去C" -> C       // ❌ C不存在，报错
    ]
}

node B { }
```

### 怎么结束对话？

有三种方式：

1. **return** - 结束当前节点（如果有后续节点，会继续）
2. **没有后续跳转** - 对话自然结束
3. **跳转到特殊节点** - 可以做一个专门的"结束"节点

```mortar
// 方式1：使用return
node A {
    choice: [
        "结束" -> return
    ]
}

// 方式2：自然结束
node B {
    text: "再见！"
    // 没有跳转，对话结束
}

// 方式3：结束节点
node C {
    choice: [
        "结束" -> EndingNode
    ]
}

node EndingNode {
    text: "谢谢游玩！"
}
```

## 选择系统

### 选项可以嵌套吗？

**可以！** 而且可以嵌套任意层：

```mortar
choice: [
    "吃什么？" -> [
        "中餐" -> [
            "米饭" -> End1,
            "面条" -> End2
        ],
        "西餐" -> [
            "牛排" -> End3,
            "意面" -> End4
        ]
    ]
]
```

### when 条件怎么写？

有两种写法：

```mortar
choice: [
    // 链式写法
    ("选项A").when(has_key) -> A,
    
    // 函数式写法
    "选项B" when has_key -> B
]
```

条件函数必须返回 `Bool` 类型：

```mortar
fn has_key() -> Bool
```

### 如果所有选项的条件都不满足怎么办？

这是**游戏逻辑**需要处理的问题。Mortar 只负责编译，不管运行时逻辑。

建议：
- 至少留一个没有条件的"默认选项"
- 在游戏里检查是否有可用选项

## 事件系统

### 事件的数字可以是小数吗？

**可以！** 小数特别适合语音同步：

```mortar
text: "这段话配了语音。"
with events: [
    0.0, start_voice()
    1.5, highlight_word()   // 1.5秒时
    3.2, another_effect()   // 3.2秒时
]
```

### 多个事件可以在同一个位置吗？

**可以！** 而且会按顺序执行：

```mortar
with events: [
    0, effect_a()
    0, effect_b()    // 同样在位置0
    0, effect_c()    // 也在位置0
]
```

游戏运行时会按顺序调用这三个函数。

### 事件函数必须声明吗？

**是的！** 所有用到的函数都要声明：

```mortar
node A {
    with events: [
        0, my_function()   // 使用了函数
    ]
}

// 必须声明
fn my_function()
```

不声明会编译报错。

## 函数相关

### 函数声明只是占位符吗？

**是的！** 函数的实际实现在你的游戏代码里：

```mortar
// Mortar 文件里只需要声明
fn play_sound(file: String)

// 真正的实现在你的游戏代码（比如C#/C++/Rust等）
// 例如在Unity中：
// public void play_sound(string file) {
//     AudioSource.PlayClipAtPoint(file);
// }
```

Mortar 只负责：
- 检查函数名是否正确
- 检查参数类型是否匹配
- 生成JSON让游戏知道该调用什么

### 支持哪些参数类型？

目前支持这些基本类型：

- `String` - 字符串
- `Bool` / `Boolean` - 布尔值（真/假）
- `Number` - 数字（整数或小数）

```mortar
fn example_func(
    name: String,
    age: Number,
    is_active: Bool
) -> String
```

### 函数可以没有参数吗？

**可以！**

```mortar
fn simple_function()
fn another() -> String
```

### 函数可以有多个参数吗？

**可以！** 用逗号分隔：

```mortar
fn complex_function(
    param1: String,
    param2: Number,
    param3: Bool
) -> Bool
```

### 函数名有什么命名规范？

**强烈推荐使用蛇形命名法（snake_case）**：

```mortar
// ✅ 推荐的命名（蛇形）
fn play_sound(file_name: String)
fn get_player_name() -> String
fn check_inventory() -> Bool
fn calculate_damage(base: Number, modifier: Number) -> Number

// ⚠️ 不推荐
fn playSound() { }          // 驼峰是其他语言的风格
fn PlaySound() { }          // 大驼峰是节点的风格
fn 播放声音() { }           // 避免使用 非 ASCII 文本
```

**参数名也要用蛇形命名**：
```mortar
fn load_scene(scene_name: String, fade_time: Number)  // ✅
fn load_scene(SceneName: String, fadeTime: Number)    // ❌
```

## 字符串插值

### 什么是字符串插值？

在字符串里嵌入变量或函数调用：

```mortar
text: $"你好，{get_name()}！你有{get_score()}分。"
```

注意字符串前的 `$` 符号！

### 插值必须是函数吗？

目前 Mortar 的插值主要用于函数调用。插值里的内容会被替换成函数的返回值。

### 不用 $ 会怎样？

没有 `$` 就是普通字符串，`{}` 会被当作普通字符：

```mortar
text: "你好，{name}！"    // 就是显示 "你好，{name}！"
text: $"你好，{name}！"   // name会被替换成实际值
```

## 编译与输出

### 编译后的文件是什么格式？

**JSON 格式**，默认是压缩的（没有空格和换行）：

```bash
mortar hello.mortar           # 生成压缩JSON
mortar hello.mortar --pretty  # 生成格式化JSON
```

### 怎么指定输出文件名？

用 `-o` 参数：

```bash
mortar input.mortar -o output.json
```

不指定的话，默认是 `input.mortared`

### JSON 结构是怎样的？

大致结构：

```json
{
  "nodes": {
    ""NodeName"": {
      "texts": [...],
      "events": [...],
      "choices": [...]
    }
  },
  "functions": [...]
}
```

详细结构看 [JSON 输出说明](./7_1_json-output.md)

### 编译错误怎么看？

Mortar 的错误信息很友好，会指出：
- 错误位置（行号、列号）
- 错误原因
- 相关的代码片段

```
Error: Undefined node 'Unknown'
  ┌─ hello.mortar:5:20
  │
5 │     choice: ["去" -> Unknown]
  │                      ^^^^^^^ 这个节点不存在
  │
```

## 项目实践

### 多人协作怎么办？

建议：
1. 使用 Git 管理 Mortar 文件
2. 按功能模块划分文件，减少冲突
3. 制定命名规范
4. 写清楚注释

### 怎么和游戏引擎配合？

基本流程：
1. 写好 Mortar 文件
2. 编译成 JSON
3. 在游戏里读取 JSON
4. 实现对应的函数
5. 按照 JSON 指示执行

详见 [接入你的游戏](./5_3_game-integration.md)

### 适合什么类型的游戏？

特别适合：
- RPG 对话系统
- 视觉小说
- 文字冒险游戏
- 互动故事

基本上任何需要"结构化对话"的游戏都适合！

### 可以用在非游戏项目吗？

**当然！** 任何需要结构化文本和事件的场景都可以：
- 教育软件
- 聊天机器人
- 交互式演示
- 多媒体展示

## 进阶话题

### 支持变量吗？

目前不支持内置变量系统，但你可以：
- 在游戏代码里维护变量
- 通过函数调用来读写变量

```mortar
// Mortar 文件
fn get_player_hp() -> Number
fn set_player_hp(hp: Number)

// 游戏代码里实现这些函数
```

### 支持表达式吗？

目前不支持复杂表达式，但可以通过函数实现：

```mortar
// 不支持：
choice: [
    "选项" when hp > 50 && has_key -> Next
]

// 可以这样：
choice: [
    "选项" when can_proceed() -> Next  
]

fn can_proceed() -> Bool  // 在游戏里实现逻辑
```

本功能即将支持。

### 怎么做本地化（多语言）？

本功能即将支持。

### 支持模块化吗？

目前每个 `.mortar` 文件是独立的，不能互相引用。

建议：
- 把相关对话写在同一个文件
- 或者在游戏里加载多个 JSON 文件并整合

本功能即将支持。

## 故障排查

### 编译时报"语法错误"怎么办？

1. 仔细看错误信息指出的位置
2. 检查是否漏了括号、引号
3. 检查关键字拼写是否正确
4. 确保"NodeName"、函数名有效

### "未定义的节点"错误？

检查：
- 跳转目标的节点是否存在
- "NodeName"大小写是否一致（区分大小写！）
- 是否有拼写错误

### "类型不匹配"错误？

检查：
- 函数声明的参数类型
- 调用时传入的参数是否匹配
- 返回类型是否正确

### 生成的 JSON 游戏读取不了？

1. 确保JSON格式正确（用 `--pretty` 检查）
2. 检查游戏代码的解析逻辑
3. 查看是否有编码问题（使用UTF-8）

## 还有问题？

- 📖 查看 [示例代码](./5_0_examples)
- 💬 到 [GitHub Discussions](https://github.com/Bli-AIk/mortar/discussions) 提问
- 🐛 在 [GitHub Issues](https://github.com/Bli-AIk/mortar/issues) 报告 bug
- 📚 阅读 [参考资料](./7_3_contributing.md)

我们很乐意帮助你！🎉
