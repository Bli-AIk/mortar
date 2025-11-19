# 命令行工具

Mortar 提供了一个简单易用的命令行工具，用来编译 `.mortar` 文件。

## 安装

### 从 crates.io 安装（推荐）

```bash
cargo install mortar_cli
```

### 从源码构建

```bash
git clone https://github.com/Bli-AIk/mortar.git
cd mortar
cargo build --release

# 编译后的可执行文件在：
# target/release/mortar (Linux/macOS)
# target/release/mortar.exe (Windows)
```

### 验证安装

```bash
mortar --version
```

应该显示版本号，例如：`mortar 0.3.0`

## 基本用法

### 最简单的编译

```bash
mortar 你的文件.mortar
```

这会生成一个同名的 `.mortared` 文件（默认是压缩的 JSON）。

**例如**：
```bash
mortar hello.mortar
# 生成 hello.mortared
```

### 格式化输出

如果想要人类可读的格式化 JSON（带缩进和换行）：

```bash
mortar hello.mortar --pretty
```

**对比**：

```bash
# 压缩格式（默认）
{"nodes":{"Start":{"texts":[{"content":"Hello"}]}}}

# 格式化输出（--pretty）
{
  "nodes": {
    "Start": {
      "texts": [
        {
          "content": "Hello"
        }
      ]
    }
  }
}
```

### 指定输出文件

使用 `-o` 或 `--output` 参数：

```bash
mortar input.mortar -o output.json

# 也可以写全：
mortar input.mortar --output output.json
```

### 组合使用

```bash
# 格式化输出到指定文件
mortar hello.mortar -o dialogue.json --pretty

# 或者这样写：
mortar hello.mortar --output dialogue.json --pretty
```

## 完整参数列表

```bash
mortar [OPTIONS] <INPUT_FILE>
```

### 必需参数

- `<INPUT_FILE>` - 要编译的 `.mortar` 文件路径

### 可选参数

| 参数 | 简写 | 说明 |
|------|------|------|
| `--output <FILE>` | `-o` | 指定输出文件路径 |
| `--pretty` | - | 生成格式化的 JSON（带缩进） |
| `--version` | `-v` | 显示版本信息 |
| `--help` | `-h` | 显示帮助信息 |

## 使用场景

### 开发阶段

开发时使用 `--pretty` 方便查看和调试：

```bash
mortar story.mortar --pretty
```

可以直接打开生成的 JSON 查看结构。

### 生产环境

发布游戏时使用压缩格式，减小文件体积：

```bash
mortar story.mortar -o assets/dialogues/story.json
```

## 错误处理

### 语法错误

如果 Mortar 文件有语法错误，编译器会清楚地指出：

```
Error: Unexpected token
  ┌─ hello.mortar:5:10
  │
5 │     text: Hello"
  │          ^ 缺少引号
  │
```

错误信息包含：
- 错误类型
- 文件名和位置（行号、列号）
- 相关代码片段
- 错误提示

### 未定义的节点

```
Error: Undefined node 'Unknown'
  ┌─ hello.mortar:10:20
  │
10 │     choice: ["去" -> Unknown]
   │                      ^^^^^^^ 这个节点不存在
   │
```

### 类型错误

```
Error: Type mismatch
  ┌─ hello.mortar:8:15
  │
8 │     0, play_sound(123)
  │                   ^^^ 期望 String，得到 Number
  │
```

### 文件不存在

```bash
$ mortar notfound.mortar
Error: 文件不存在: notfound.mortar
```

## 退出码

Mortar CLI 遵循标准的退出码约定：

- `0` - 编译成功
- `1` - 编译失败（语法错误、类型错误等）
- `2` - 文件读取失败
- `3` - 文件写入失败

这在 CI/CD 脚本中特别有用：

```bash
#!/bin/bash
if mortar dialogue.mortar; then
    echo "✅ 编译成功"
else
    echo "❌ 编译失败"
    exit 1
fi
```

## 常见问题

### Q: 为什么默认是压缩格式？

A: 压缩格式文件更小，加载更快，适合生产环境。开发时用 `--pretty` 查看。

### Q: 可以编译整个目录吗？

A: 目前不支持，但可以用 shell 脚本批量编译。

### Q: 输出文件可以不是 JSON 吗？

A: 目前只支持 JSON 输出。JSON 是通用格式，几乎所有语言和引擎都能解析。

### Q: 怎么检查语法但不生成文件？

A: 目前没有专门的检查模式，但可以输出到临时文件：
```bash
mortar test.mortar -o /tmp/test.json
```

## 小结

Mortar CLI 的关键点：
- ✅ 简单易用，一个命令搞定
- ✅ 清晰的错误提示
- ✅ 支持格式化输出方便调试
- ✅ 易于集成到开发流程
- ✅ 快速可靠

## 接下来

- 了解编辑器支持：[编辑器支持](./6_2_ide-support.md)
- 查看 JSON 输出格式：[JSON 输出说明](../7_1_json-output.md)
- 回到快速开始：[五分钟上手](../2_quick-start.md)
