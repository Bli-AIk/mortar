# Mortar DSL

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/mortar.svg)](https://crates.io/crates/mortar)
[![Documentation](https://docs.rs/mortar/badge.svg)](https://docs.rs/mortar)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **当前状态**：🚧 早期开发中（初始版本正在开发）

![Mortar](./mortar_logo.svg)

**Mortar** 是一个为游戏对话与文字事件系统设计的领域特定语言（DSL），核心理念是实现 **文本内容与事件逻辑的严格分离**。

| 英语                             | 简体中文 |
|--------------------------------|------|
| [English Version](./readme.md) | 简体中文 |

## 简介

Mortar 的灵感来自 [ink](https://github.com/inkle/ink) 与 [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner)，
但它的核心区别在于：

> **Mortar 旨在实现文本内容与事件逻辑的严格分离。**

* **文本部分**：纯叙事内容，完全为人类编写，不混入事件逻辑；
* **事件部分**：系统执行指令，用于控制呈现效果，与文本内容无关；
* **Mortar 语言本身**：提供一种优雅的桥梁，让两者能清晰关联、互不污染。

## 设计目标

Mortar 的设计遵循以下核心原则：**内容分离、语义清晰、程序友好、静态类型。**

1. **内容与逻辑解耦**：事件以字符索引触发，避免富文本标记污染内容；文本中不含控制标记，保持纯净；
2. **语义清晰**：采用 Rust 风格的语法设计，DSL 语法直观、易读、易维护；
3. **程序友好**：以 JSON 结构进行编译，支持使用者进行针对性的解析；
4. **静态类型**：作为静态类型语言，编译时进行类型检查以提前捕获类型错误，提高运行时可靠性。

## 快速上手

### 安装

```bash
# 从 crates.io 安装
cargo install mortar

# 或从源码构建
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release
```

### 基本用法

创建一个简单的 Mortar 文件 `hello.mortar`：

```mortar
// ‘node’ 就是一个基本的对话节点。
// 也可以缩写为 ‘nd’！
node start {
    // 编写你的文本内容。
    // 双引号（或单引号）是必须的，但分号和逗号可以省略！
    text: "你好呀，欢迎阅读这个互动故事。"
    
    // 这个事件列表写在紧挨着上一个 text，所以它们是关联的。
    events: [
        // 使用 索引 + 事件函数 的方式来编写事件。支持链式写法。
        // 这里的索引表示事件触发的字符位置（从 0 开始计数）。
        // 它会绑定到你的游戏具体实现——打字机播放的位置？音频时间轴？还是别的什么，都可以，看你怎么实现。
        0, play_sound("greeting.wav")
        6, set_animation("wave").play_sound("wave_sound.wav")
        17, set_color("#FF6B6B")
    ]
    // 当我们再次使用 text 字段时，表示这是同一个节点的另一个文本块。
    // 你可以写若干个 text 块，它们会被顺序播放。
    text: "我想你的名字是 {get_name()}，对不？"
    events: [
        // 索引可以是浮点数！一般来说，语音同步会用到小数点。而打字机则是整数。
        // 实际上，mortar 中的数字都是 Number，这和 json 里的数字类型是一样的。
        4.2, set_color("#33CCFF")
        10.8, set_color("#FF6B6B")
    ]
    
    // 这个 text 块没有 events…… 这是完全合法的！
    text: "太好啦，我们走！"
    
// 节点后面的箭头表示跳转到下一个节点。
} -> choice_point

/*
这里还有一个节点，展示了如何编写选项——通过选择字段实现。
*/

node choice_point {
    text: "你想干点啥？"
    
    // 通过选择字段，我们也可以跳转到不同的节点。
    choice: [
        // 这个选项没有任何条件判断。按理来说，你始终可以选择它。
        "探索森林" -> forest_scene,
        
        // 这两个选项带有 when 关键字，这说明它们带有条件判断！
        // when 关键字支持 链式写法 和 函数式写法。
        ("留在城里").when(has_map) -> town_scene,
        "查看背包" when has_backpack  -> inventory,
        
        // 选择字段也可以嵌套一个选择字段。你想嵌套多少层都行！
        "吃点什么" -> [
            "Apple" -> eat_apple,
            "Bread" -> eat_bread
        ]
        
        // 使用 return 关键字 退出当前节点。
        // 顺带一提，如果这个节点有后续节点，那 return 不会终止整个对话流程，只会退出当前节点。
        "别朝我叭叭了！！" -> return,
        
        // 使用 break 关键字 终止选项列表。
        "我不到啊……" -> break,
    ],
    
    // 在这个选择字段中，只有你选了 "我不到啊……"，才会来到这一行。
    text: "我真服了。那咱就先结束对话吧。",
    
    // 然后，由于没有任何后续节点，这个对话还是结束了。
}

// 在 start 中调用的函数不是平白无故就出现的——你需要在 Mortar 文件中定义它们！
// 这有点像是 C / C++ 里的函数声明。它们最终会被编译器识别并关联到你的游戏代码中。
fn play_sound(file_name: String)

fn set_animation(anim_name: String)

fn set_color(value: String)

fn get_name() -> String
```

编译该 Mortar 文件：

```bash
# 基础编译（输出 hello.mortared）
mortarc hello.mortar

# 指定输出文件
mortarc hello.mortar -o hello.json

# 启用详细输出
mortarc hello.mortar --verbose
```

## 适用场景

* 🎮 **游戏对话系统**：RPG 对话、视觉小说
* 📖 **交互小说**：文字冒险、分支叙事
* 📚 **教育内容**：互动式教学、引导式学习场景
* 🤖 **聊天脚本**：结构化对话逻辑
* 🖼️ **多媒体呈现**：文字与媒体事件的同步

## 开发进度

待实现功能：

* 🚧 **命令行工具**：完整 CLI 编译器
* 🚧 **词法分析器**：使用 logos 实现的高性能分词
* 🚧 **解析框架**：支持完整的 token 解析
* 🚧 **AST 结构**：完整的抽象语法树定义
* 🚧 **节点定义**：支持 text、speaker、tags 字段
* 🚧 **事件系统**：事件类型与 AST 构建
* 🚧 **选项系统**：基于构建者模式的选项定义
* 🚧 **错误处理**：ariadne 友好的错误报告
* 🚧 **JSON 输出**：标准化输出格式

计划中功能：

* 🚧 **高级语法解析**：完整事件与选项语法
* 🚧 **条件表达式**：复杂逻辑判断支持
* 🚧 **变量系统**：全局与局部变量管理
* 🚧 **函数调用**：内置与自定义函数
* 🚧 **语言服务器**：IDE 集成与语法高亮

## 参与贡献

欢迎社区贡献！详细信息请参阅 [贡献指南](./CONTRIBUTING.md)。

### 贡献者

以下人员为本项目做出了贡献。

<a href = "https://github.com/Bli-AIk/mortar/Python/graphs/contributors">
<img src = "https://contrib.rocks/image?repo=Bli-AIk/mortar" alt=""/>
</a>

**衷心感谢你们每一个人！🎔**

### 开发环境搭建

```bash
# 克隆仓库
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# 安装依赖并构建
cargo build

# 运行测试
cargo test

# 代码检查
cargo clippy

# 格式化代码
cargo fmt
```

## 许可协议

Mortar 采用双许可证模式：

### MIT License

允许任何人免费使用、复制、修改、分发本软件。

### Apache License 2.0

在 Apache 2.0 许可下分发。

你可以根据需求选择其中任意一种许可证。
详见 [LICENSE-MIT](./LICENSE-MIT) 与 [LICENSE-APACHE](./LICENSE-APACHE)。

## 社区

* **GitHub Issues**：[报告问题或提出功能建议](https://github.com/Bli-AIk/mortar/issues)
* **讨论区**：[社区问答与讨论](https://github.com/Bli-AIk/mortar/discussions)

## 相关项目

* [ink](https://github.com/inkle/ink) —— Inkle 的叙事脚本语言
* [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner) —— 用于构建互动对话的工具

## 致谢

特别感谢 ink 与 Yarn Spinner 的作者们，为互动叙事工具开辟了道路。

同时感谢 Rust 社区提供了优异的解析与编译相关库，使 Mortar 得以诞生。
