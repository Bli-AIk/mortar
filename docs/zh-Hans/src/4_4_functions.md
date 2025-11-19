# 函数：连接游戏世界

函数是 Mortar 和你的游戏代码之间的桥梁。通过函数声明，你告诉 Mortar："这些功能我的游戏会实现"。

## 为什么需要函数声明？

在 Mortar 脚本中，你会调用各种函数：

```mortar
events: [
    0, play_sound("boom.wav")
    2, shake_screen()
]
```

但这些函数在哪里？它们在你的游戏代码里！

**函数声明**就是一个"约定"：
- 你告诉 Mortar：我的游戏有这些函数，它们需要什么参数，返回什么
- Mortar 编译时检查类型，确保你用对了
- 编译成 JSON 后，你的游戏再实现这些函数

## 函数命名规范

> **⚠️ 重要：推荐使用蛇形命名法（snake_case）**

**✅ 推荐的命名方式**：
```mortar
fn play_sound(file_name: String)         // 蛇形：全小写，单词用下划线分隔
fn get_player_name() -> String           // 清晰易读
fn check_inventory_space() -> Bool       // 见名知意
fn calculate_damage(base: Number, modifier: Number) -> Number
```

**⚠️ 不推荐的命名方式**：
```mortar
fn playSound() { }              // 避免小驼峰命名（这是其他语言的风格）
fn PlaySound() { }              // 不要用大驼峰（这是节点的风格）
fn play-sound() { }             // 不建议使用串型明明
fn 播放声音() { }               // 不建议使用非 ASCII 文本
fn playsound() { }              // 全小写不易阅读
```

**参数命名规范**：
```mortar
// ✅ 好的参数命名
fn move_to(x: Number, y: Number)
fn load_scene(scene_name: String, fade_time: Number)

// ❌ 不好的参数命名
fn move_to(a: Number, b: Number)        // 没有语义
fn load_scene(s: String, t: Number)        // 缩写不清晰
```

**命名建议**：
- 使用英文单词，全小写
- 多个单词用下划线 `_` 分隔
- 动词开头，描述函数功能：`get_`, `set_`, `check_`, `play_`, `show_`
- 参数名要有描述性
- 保持项目内命名风格一致

## 基本语法

```mortar
fn function_name(param: Type) -> ReturnType
```

### 无参数无返回值

```mortar
fn shake_screen()
fn clear_text()
fn show_menu()
```

### 有参数无返回值

```mortar
fn play_sound(file: String)
fn set_color(color: String)
fn move_character(x: Number, y: Number)
```

### 有返回值

```mortar
fn get_player_name() -> String
fn get_gold() -> Number
fn has_key() -> Bool
```

### 有参数有返回值

```mortar
fn calculate(a: Number, b: Number) -> Number
fn find_item(name: String) -> Bool
```

## 支持的类型

Mortar 支持 json 中的类型：

| 类型 | 别名 | 说明 | 示例 |
|------|------|------|------|
| `String` | - | 字符串 | `"你好"`, `"file.wav"` |
| `Number` | - | 数字（整数或小数） | `42`, `3.14` |
| `Bool` | `Boolean` | 布尔值 | `true`, `false` |

**注意**：`Bool` 和 `Boolean` 是一样的，随便用哪个。

## 完整示例

```mortar
// 一个完整的 Mortar 文件

node StartScene {
    text: $"欢迎，{get_player_name()}！"
    events: [
        0, play_bgm("theme.mp3")
    ]
    
    text: $"你有 {get_gold()} 金币。"
    
    choice: [
        "去商店" when can_shop() -> 商店,
        "去冒险" -> 冒险
    ]
}

node 商店 {
    text: "欢迎来到商店！"
}

node 冒险 {
    text: "冒险开始！"
    events: [
        0, start_battle("哥布林")
    ]
}

// ===== 函数声明区 =====

// 播放背景音乐
fn play_bgm(music: String)

// 获取玩家名字
fn get_player_name() -> String

// 获取金币数量
fn get_gold() -> Number

// 检查是否能购物
fn can_shop() -> Bool

// 开始战斗
fn start_battle(enemy: String)
```

## 在事件中使用

### 调用无参数函数

```mortar
events: [
    0, shake_screen()
    2, flash_white()
]

fn shake_screen()
fn flash_white()
```

### 调用有参数函数

```mortar
events: [
    0, play_sound("boom.wav")
    2, set_color("#FF0000")
    4, move_to(100, 200)
]

fn play_sound(file: String)
fn set_color(hex: String)
fn move_to(x: Number, y: Number)
```

### 链式调用

```mortar
events: [
    0, play_sound("boom.wav").shake_screen().flash_white()
]

fn play_sound(file: String)
fn shake_screen()
fn flash_white()
```

## 在文本插值中使用

只有返回值的函数才能用在 `${}` 中：

```mortar
text: $"你好，{get_name()}！"
text: $"等级：{get_level()}"
text: $"状态：{get_status()}"

fn get_name() -> String
fn get_level() -> Number
fn get_status() -> String
```

**注意**：插值中的函数必须返回 String！

```mortar
// ❌ 错误：函数无返回值
text: $"结果：{do_something()}"
fn do_something()  // 没有返回值


// ❌ 错误：返回类型不是 String
text: $"结果：{get_hp()}"
fn get_hp() -> Number  // 返回类型错误

// ✅ 正确
text: $"结果：{get_result()}"
fn get_result() -> String
```

## 在条件中使用

`when` 后面的函数必须返回 `Bool` / `Boolean`：

```mortar
choice: [
    "特殊选项" when is_unlocked() -> 特殊节点
]

fn is_unlocked() -> Bool
```

## 函数声明的位置

习惯上，把所有函数声明放在文件末尾：

```mortar
// 节点定义
node A { ... }
node B { ... }
node C { ... }

// ===== 函数声明 =====
fn func1()
fn func2()
fn func3()
```

但其实位置不重要，你可以放在任何地方。

## 静态类型检查

Mortar 会在编译时检查类型是否正确：

```mortar
// ✅ 正确
events: [
    0, play_sound("file.wav")
]
fn play_sound(file: String)

// ❌ 错误：参数类型不对
events: [
    0, play_sound(123)  // 传了数字，但需要字符串
]
fn play_sound(file: String)
```

这能帮你提前发现错误！

## 实现函数（游戏端）

Mortar 只负责声明，真正的实现在你的游戏代码里。

编译后的 JSON 会包含函数信息：

```json
{
  "functions": [
    {
      "name": "play_sound",
      "params": [
        {"name": "file", "type": "String"}
      ]
    },
    {
      "name": "get_player_name",
      "return": "String"
    }
  ]
}
```

你的游戏读取 JSON，然后实现这些函数。

详见[接入游戏](./5_3_game-integration.md)。

## 最佳实践

### ✅ 好的做法

```mortar
// 清晰的命名
fn play_background_music(file: String)
fn get_player_health() -> Number
fn is_quest_completed(quest_id: Number) -> Bool
```

```mortar
// 合理的参数
fn spawn_enemy(name: String, x: Number, y: Number)
fn set_weather(type: String, intensity: Number)
```

### ❌ 不好的做法

```mortar
// 命名不清晰
fn psm(f: String)  // 什么意思？
fn x() -> Number   // x 是什么？
```

```mortar
// 参数太多
fn do_complex_thing(a: Number, b: Number, c: String, d: Bool, e: Number, f: String)
```

### 建议

1. **见名知意**：函数名应该说明它做什么
2. **参数适度**：一般不超过 7 个参数
3. **类型明确**：所有参数和返回值都要注明类型
4. **分类整理**：相关的函数放在一起，加注释说明

## 常见问题

### Q: 必须声明所有用到的函数吗？
是的！没声明就用会报错。

### Q: `fn` 可以写成 `function` 吗？
可以！两者完全一样：

```mortar
fn play_sound(file: String)
function play_sound(file: String)  // 一样的
```

### Q: 能声明但不使用吗？
可以。声明了但没用到的函数，编译器会警告，但不会报错。

### Q: 函数可以重载吗？
不可以。每个函数名只能声明一次。

```mortar
// ❌ 错误：重复声明
fn test(a: String)
fn test(a: Number, b: Number)
```

### Q: 参数可以有默认值吗？
目前不支持。所有参数都是必需的。

## 下一步

- 看[完整示例](./5_1_basic-dialogue.md)
- 学习如何[接入游戏](./5_3_game-integration.md)
- 查看 [JSON 输出格式](./7_1_json-output.md)
