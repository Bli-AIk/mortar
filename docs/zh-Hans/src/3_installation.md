# 安装工具

要使用 Mortar，你需要安装编译工具。别担心，过程很简单！

## 方法一：用 Cargo 安装（推荐）

如果你已经安装了 [Rust](https://www.rust-lang.org/)，那就太方便了：

```bash
cargo install mortar_cli
```

等待安装完成，然后检查是否成功：

```bash
mortar --version
```

看到版本号就说明安装成功了！

## 方法二：从源码编译

想体验最新的开发版？可以从源码构建：

```bash
# 下载源码
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# 编译
cargo build --release

# 编译好的程序在这里
./target/release/mortar --version
```

**提示**：编译好的可执行文件位于 `target/release/mortar`，你可以把它复制到系统路径中。

## 检查安装

运行这个命令测试一下：

```bash
mortar --help
```

你应该能看到帮助信息，说明各种用法。

## 编辑器支持（可选但推荐）

为了更好的编写体验，可以安装语言服务器：

```bash
cargo install mortar_lsp
```

然后在你喜欢的编辑器里配置它：

### VS Code

1. 安装扩展：搜索 "Mortar" 并安装
2. 重启编辑器，就能享受语法高亮和自动补全了！

### 其他编辑器

查看[编辑器支持](./6_2_ide-support.md)了解如何配置其他编辑器。

## 遇到问题？

### "找不到 cargo 命令"

你需要先安装 Rust。访问 [https://rustup.rs](https://rustup.rs) 按照指引安装。

### "编译失败"

确保你的 Rust 版本足够新：

```bash
rustup update
```

### 其他问题

- 查看 [GitHub Issues](https://github.com/Bli-AIk/mortar/issues)
- 或者在 [Discussions](https://github.com/Bli-AIk/mortar/discussions) 里提问

## 下一步

安装好了？那就去[五分钟上手](./2_quick-start.md)试试看吧！
