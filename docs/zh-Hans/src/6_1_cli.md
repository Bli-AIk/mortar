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

### 批量编译

在 Linux/macOS 上批量编译多个文件：

```bash
# 编译当前目录所有 .mortar 文件
for file in *.mortar; do
    mortar "$file" -o "${file%.mortar}.json"
done
```

在 Windows PowerShell 上：

```powershell
Get-ChildItem *.mortar | ForEach-Object {
    mortar $_.Name -o "$($_.BaseName).json"
}
```

### 自动化构建

在构建脚本中使用：

**Makefile**:
```makefile
.PHONY: compile-dialogues
compile-dialogues:
	mortar dialogues/chapter1.mortar -o assets/chapter1.json
	mortar dialogues/chapter2.mortar -o assets/chapter2.json
	mortar dialogues/chapter3.mortar -o assets/chapter3.json
```

**package.json** (Node.js 项目):
```json
{
  "scripts": {
    "build:dialogues": "mortar dialogues/main.mortar -o public/dialogues.json"
  }
}
```

**build.sh**:
```bash
#!/bin/bash
echo "编译对话文件..."
mortar dialogues/main.mortar -o assets/main_dialogue.json
echo "完成！"
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

## 性能

Mortar 编译器速度很快：

- 小型文件（<100 节点）：几乎瞬间完成
- 中型文件（100-500 节点）：通常 < 1 秒
- 大型文件（500+ 节点）：通常 < 3 秒

**示例**：

```bash
$ time mortar large_story.mortar
编译完成：1247 个节点，389 个函数

real    0m0.856s
user    0m0.721s
sys     0m0.134s
```

## 集成到编辑器

### VS Code

创建任务 `.vscode/tasks.json`：

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "编译 Mortar",
      "type": "shell",
      "command": "mortar",
      "args": [
        "${file}",
        "--pretty"
      ],
      "group": {
        "kind": "build",
        "isDefault": true
      },
      "presentation": {
        "reveal": "always",
        "panel": "new"
      }
    }
  ]
}
```

然后按 `Ctrl+Shift+B` 或 `Cmd+Shift+B` 即可编译当前文件。

### Vim/Neovim

在 `.vimrc` 或 `init.vim` 中添加：

```vim
" 编译当前 Mortar 文件
autocmd FileType mortar nnoremap <buffer> <F5> :!mortar % --pretty<CR>
```

按 `F5` 编译当前文件。

### Sublime Text

创建构建系统 `Tools -> Build System -> New Build System`：

```json
{
  "cmd": ["mortar", "$file", "--pretty"],
  "file_regex": "^(.+):([0-9]+):([0-9]+)",
  "selector": "source.mortar"
}
```

## 常见问题

### Q: 为什么默认是压缩格式？

A: 压缩格式文件更小，加载更快，适合生产环境。开发时用 `--pretty` 查看。

### Q: 可以编译整个目录吗？

A: 目前不支持，但可以用 shell 脚本批量编译（见上文"批量编译"）。

### Q: 输出文件可以不是 JSON 吗？

A: 目前只支持 JSON 输出。JSON 是通用格式，几乎所有语言和引擎都能解析。

### Q: 编译很慢怎么办？

A: 如果文件很大（>1000 节点），考虑拆分成多个文件。通常编译应该很快。

### Q: 怎么检查语法但不生成文件？

A: 目前没有专门的检查模式，但可以输出到临时文件：
```bash
mortar test.mortar -o /tmp/test.json
```

## 高级用法

### 配合 Git Hooks

在 `.git/hooks/pre-commit` 中：

```bash
#!/bin/bash
# 自动编译所有修改的 Mortar 文件

for file in $(git diff --cached --name-only | grep '\.mortar$'); do
    echo "编译 $file..."
    if ! mortar "$file"; then
        echo "❌ $file 编译失败"
        exit 1
    fi
done

echo "✅ 所有对话文件编译成功"
```

### 监视文件变化

使用 `watchexec`（需要单独安装）：

```bash
# 安装 watchexec
cargo install watchexec-cli

# 监视文件变化并自动编译
watchexec -e mortar "mortar dialogues/main.mortar -o output.json --pretty"
```

每次保存 `.mortar` 文件时自动编译。

### CI/CD 集成

**GitHub Actions** 示例：

```yaml
name: 编译对话文件

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: 安装 Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: 安装 Mortar CLI
        run: cargo install mortar_cli
      
      - name: 编译对话文件
        run: |
          mortar dialogues/main.mortar -o assets/main.json
          mortar dialogues/tutorial.mortar -o assets/tutorial.json
      
      - name: 上传编译结果
        uses: actions/upload-artifact@v2
        with:
          name: dialogues
          path: assets/*.json
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
