# Mortar Language

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Crates.io](https://img.shields.io/crates/v/mortar_language.svg)](https://crates.io/crates/mortar_language)
[![Documentation](https://docs.rs/mortar_language/badge.svg)](https://docs.rs/mortar_language)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar_language/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar_language)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

![mortar_logo](https://raw.githubusercontent.com/Bli-AIk/mortar/refs/heads/main/crates/mortar_logo.svg)

**Mortar** 是一个为游戏对话与文字事件系统设计的领域特定语言（DSL），核心理念是实现 **文本内容与事件逻辑的严格分离**。

阅读 [官方指南](https://bli-aik.github.io/mortar/zh-hans/) 来学习如何使用 mortar 吧！

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

> `Mortar Language` 是 SoupRune 项目的衍生工具，是其对话系统的首选语言。
>
> SoupRune 是专门针对 Deltarune / Undertale Fangame 的游戏框架。[了解更多](https://github.com/Bli-AIk/souprune)。

## 设计目标

Mortar 的设计遵循以下核心原则：**内容分离、语义清晰、程序友好、静态类型。**

1. **内容与逻辑解耦**：事件以字符索引触发，避免富文本标记污染内容；文本中不含控制标记，保持纯净；
2. **语义清晰**：采用 Rust 风格的语法设计，DSL 语法直观、易读、易维护；
3. **程序友好**：以 JSON 结构进行编译，支持使用者进行针对性的解析；
4. **静态类型**：作为静态类型语言，编译时进行类型检查以提前捕获类型错误，提高运行时可靠性。

## 快速上手

### 安装

```bash
# 从 crates.io 安装命令行工具
cargo install mortar_cli

# 或从源码构建
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release
```

### 简单示例

创建一个名为 `hello.mortar` 的文件：

```mortar
node Start {
    text: "你好，欢迎使用 Mortar！"
    text: "这是一个极简示例。"
}
```

### 进阶特性

Mortar 支持复杂的事件、选项和逻辑：

```mortar
node Start {
    text: "你好呀，欢迎阅读这个互动故事。"

    // 与文本关联的事件列表
    events: [
        0, play_sound("greeting.wav")
        6, set_animation("wave")
    ]

    text: $"我想你的名字是 {get_name()}，对不？"
    events: [
        4.2, set_color("#33CCFF")
    ]

} -> ChoicePoint

node ChoicePoint {
    text: "你想干点啥？"

    choice: [
        "探索森林" -> ForestScene,
        ("留在城里").when(has_map) -> TownScene,
        "吃点什么" -> [
            "Apple" -> EatApple,
            "Bread" -> EatBread
        ]
        "退出" -> return,
    ]
}

// 函数声明
fn play_sound(file_name: String)
fn set_animation(anim_name: String)
fn set_color(value: String)
fn get_name() -> String
function has_map() -> Bool
``````

编译该 Mortar 文件：

```bash
# 基础编译（默认输出压缩格式的 JSON）
mortar hello.mortar

# 生成带缩进的格式化 JSON
mortar hello.mortar --pretty

# 指定输出文件
mortar hello.mortar -o output_file

# 组合选项
mortar hello.mortar -o custom.json --pretty
```

编译器现在默认生成压缩格式的 JSON 以获得最优的文件大小和性能表现。当需要人类可读的格式化输出用于调试或查看时，请使用 `--pretty` 标志。

## 适用场景

* 🎮 **游戏对话系统**：RPG 对话、视觉小说
* 📖 **交互小说**：文字冒险、分支叙事
* 📚 **教育内容**：互动式教学、引导式学习场景
* 🤖 **聊天脚本**：结构化对话逻辑
* 🖼️ **多媒体呈现**：文字与媒体事件的同步

## 开发进度

实现功能：

* ✅ **命令行工具**：完整 CLI 编译器
* ✅ **词法分析器**：使用 logos 实现的高性能分词
* ✅ **解析框架**：支持完整的 token 解析
* ✅ **AST 结构**：完整的抽象语法树定义
* ✅ **错误处理**：ariadne 友好的错误报告
* ✅ **JSON 输出**：标准化输出格式
* ✅ **语言服务器**：IDE 集成与语法高亮
* ✅ **变量系统**：变量声明、常量定义、枚举类型
* ✅ **分支插值**：支持非对称文本（参考 [Fluent](https://github.com/projectfluent/fluent) 设计）
* ✅ **条件表达式**：与、或、非，比较
* ✅ **判断语句**：if，else
* ✅ **演出系统**：将 events 提取成 独立节点

## 参与贡献

欢迎社区贡献！详细信息请参阅 [贡献指南](./CONTRIBUTING_zh-hans.md)。

### 贡献者

以下人员为本项目做出了贡献。

<a href = "https://github.com/Bli-AIk/mortar/Python/graphs/contributors">
<img src = "https://contrib.rocks/image?repo=Bli-AIk/mortar" alt=""/>
</a>

**衷心感谢你们每一个人！🎔**

## 项目结构

```mermaid
graph TD
    subgraph "Mortar 生态系统"
        Compiler[mortar_compiler<br>(核心逻辑)]
        CLI[mortar_cli<br>(命令行工具)]
        LSP[mortar_lsp<br>(语言服务器)]
        Lib[mortar_language<br>(主库)]
    end

    CLI --> Compiler
    LSP --> Compiler
    Lib --> Compiler
    Lib --> LSP
```

本项目采用 Rust workspace 组织，包含四个主要的 crate：

* **`mortar_language`** - 主要的库 crate，重新导出所有其他 crate 的功能
* **`mortar_compiler`** - 核心编译库，包含词法分析、语法解析和代码生成
* **`mortar_cli`** - 命令行界面，提供 `mortar` 命令
* **`mortar_lsp`** - 语言服务器协议实现，用于 IDE 集成

### 构建项目

```bash
# 克隆仓库
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# 构建 workspace 中的所有 crate
cargo build

# 构建优化的 release 版本
cargo build --release

# 构建特定的 crate
cargo build -p mortar_cli
cargo build -p mortar_compiler
cargo build -p mortar_language
cargo build -p mortar_lsp

# 运行所有 crate 的测试
cargo test

# 运行特定 crate 的测试
cargo test -p mortar_compiler

# 代码检查
cargo clippy

# 格式化代码
cargo fmt
```

### 安装单个组件

```bash
# 仅安装 CLI 工具
cargo install mortar_cli

# 仅安装 LSP 服务器
cargo install mortar_lsp

# 在 Cargo.toml 中作为库依赖使用
[dependencies]
mortar_language = "0.4"
# 或使用单个组件
mortar_compiler = "0.4"
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
