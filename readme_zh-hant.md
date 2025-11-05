# Mortar DSL

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/mortar.svg)](https://crates.io/crates/mortar)
[![Documentation](https://docs.rs/mortar/badge.svg)](https://docs.rs/mortar)
[![codecov](https://codecov.io/gh/Bli-AIk/mortar/graph/badge.svg?token=)](https://codecov.io/gh/Bli-AIk/mortar)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **当前状态**：🚧 早期开发中（初始版本正在开发）

**Mortar** 是一个为游戏对话与文字事件系统设计的领域特定语言（DSL），核心理念是实现 **文本内容与事件逻辑的严格分离**。

| 英语                             | 简体中文 |
|--------------------------------|------|
| [English Version](./readme.md) | 简体中文 |

## 简介

Mortar 的灵感来自 [ink](https://github.com/inkle/ink) 与 [Yarn Spinner](https://github.com/YarnSpinnerTool/YarnSpinner)，
但它的核心区别在于：

> **Mortar 旨在实现文本内容与事件逻辑的严格分离**

* **文本部分**：纯叙事内容，完全为人类编写，不混入事件逻辑；
* **事件部分**：系统执行指令，用于控制呈现效果，与文本内容无关；
* **Mortar 语言本身**：提供一种优雅的桥梁，让两者能清晰关联、互不污染。

## 设计目标

Mortar 的设计遵循以下核心原则：**内容分离、语义清晰、程序友好**

1. **内容与逻辑解耦**：事件以字符索引触发，避免富文本标记污染内容；文本中不含控制标记，保持纯净
2. **语义清晰**：采用 Rust 风格的语法设计 ，DSL 语法直观、易读、易维护
3. **程序友好**：以 JSON 结构进行编译，支持使用者进行针对性的解析

## 快速上手

### 安装

```bash
# 从 crates.io 安装（暂未完成）
cargo install mortar

# 或从源码构建
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release
```

### 基本用法

创建一个简单的 Mortar 文件 `hello.mortar`：

```mortar
// 节点定义（Rust 风格结构体）
node start {
    text: "Hello, welcome to this interactive story.",
    events: vec![
        Event::at(0).play_sound("greeting.wav"),
        Event::at(6).set_animation("wave"),
        Event::at(17).set_color("#FF6B6B"),
    ],
}

node choice_point {
    text: "What would you like to do?",
    
    choices: vec![
        Choice::new("Explore the forest") => forest_scene,
        Choice::new("Stay in town")
            .when(has_map == true) => town_scene,
        Choice::new("Check inventory") => inventory,
    ],
}
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
