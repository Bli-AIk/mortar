# 安装工具

要使用 Mortar，你需要安装编译工具。别担心，过程很简单！

## 方法一：用 Cargo 安装（推荐）

如果你已经安装了 [Rust](https://rust-lang.org/)，那就太方便了：

```bash
cargo install mortar_cli
```

等待安装完成，然后检查是否成功：

```bash
mortar --version
```

看到版本号就说明安装成功了！

## 方法二：从源码编译（不太适合普通用户）

想体验最新的开发版？可以从源码构建（这也需要 rust 开发环境）：

```bash
# 下载源码
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# 编译
cargo build --release

# 编译好的程序在这里
./target/release/mortar --version
```

**提示**：编译好的可执行文件位于 `target/release/mortar`，你可以把它加入环境变量。

## 方法三：从 GitHub Release 下载（不太适合普通用户）

如果你不想使用 Rust 或 Cargo，也可以直接从 [Mortar 的 GitHub Release 页面](https://github.com/Bli-AIk/mortar/releases) 下载预编译的二进制文件。

### Linux / macOS

1. 打开 Release 页面，下载对应版本，例如 `mortar-x.x.x-linux-x64.tar.gz` 或 `mortar-x.x.x-macos-x64.tar.gz`。
2. 解压到任意目录：

```bash
tar -xzf mortar-x.x.x-linux-x64.tar.gz -C ~/mortar
```

3. 将可执行文件路径加入环境变量，例如：

```bash
export PATH="$HOME/mortar:$PATH"
```

4. 检查是否安装成功：

```bash
mortar --version
```

### Windows

1. 下载对应版本的 `mortar-x.x.x-windows-x64.zip`。
2. 解压到任意目录，例如 `D:\mortar`。
3. 将目录添加到系统环境变量 PATH：
    * 右键「此电脑」→「属性」→「高级系统设置」→「环境变量」
    * 在「系统变量」或「用户变量」中找到 `Path` → 编辑 → 添加 `D:\mortar`
4. 打开新的命令提示符，检查安装：

```cmd
mortar --version
```

⚠️ **注意**：

* 需要手动设置环境变量
* 每次开新终端或修改系统配置时可能会出现问题
* 对普通用户来说不太友好

因此推荐使用 **方法一（Cargo）**，安装体验更顺畅。

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

然后在你喜欢的编辑器里配置它即可。

查看 [编辑器支持](./6_2_ide-support.md) 了解如何配置你的编辑器。

## 遇到问题？

### "找不到 cargo 命令"

你需要先安装 Rust。访问 [https://rust-lang.org/](https://rust-lang.org/) 按照指引安装。

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
