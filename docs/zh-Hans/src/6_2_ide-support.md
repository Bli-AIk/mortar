# 编辑器支持

Mortar 提供了编辑器支持，让你在写对话时更高效！

## VS Code 扩展

Mortar 有官方的 VS Code 扩展，提供语法高亮、自动补全等功能。

### 安装

有两种方式：

#### 方式1：从源码安装（推荐）

```bash
# 克隆仓库
git clone https://github.com/Bli-AIk/mortar.git
cd mortar/vscode-mortar-extension

# 安装依赖
npm install

# 打包扩展
npm run package

# 在 VS Code 中安装
# 方法1：命令行
code --install-extension mortar-language-0.1.0.vsix

# 方法2：手动安装
# 打开 VS Code -> 扩展面板 -> 点击 "..." -> "从 VSIX 安装"
```

#### 方式2：从市场安装（即将上线）

未来会发布到 VS Code 市场，届时可以直接搜索 "Mortar Language" 安装。

### 功能特性

#### 1. 语法高亮

自动识别 Mortar 语法元素并着色：

- **关键字**：`node`, `text`, `events`, `choice`, `fn` 等
- **字符串**：文本内容高亮
- **注释**：`//` 和 `/* */` 的注释
- **数字**：事件索引等数字
- **函数调用**：特殊颜色标记

**效果预览**：

```mortar
// ← 这是注释（灰色）
node StartScene {  // ← 'node' 是关键字（蓝色），'开始' 是标识符
    text: "你好！"  // ← 'text' 是关键字，字符串是绿色
    events: [  // ← 'events' 是关键字
        0, play_sound("hi.wav")  // ← 函数名特殊高亮
    ]
}
```

#### 2. 代码片段（Snippets）

输入缩写快速生成模板！

| 触发词 | 生成内容 |
|--------|----------|
| `node` | 完整的节点结构 |
| `text` | 文本块 |
| `events` | 事件列表 |
| `choice` | 选择列表 |
| `fn` | 函数声明 |

**示例**：

输入 `node` + `Tab`：

```mortar
node ${1:NodeName} {
    text: "${2:对话内容}"
    $0
}
```

光标会自动停在需要填写的地方！

#### 3. 括号匹配

自动高亮匹配的括号，方便查看嵌套层级：

```mortar
node TestNode {  // ← 点击这里
    choice: [
        "选项" -> [  // ← 自动高亮对应的括号
            "子选项" -> A
        ]
    ]
}  // ← 高亮对应的结束大括号
```

#### 4. 自动缩进

按下 `Enter` 时自动缩进到合适的位置：

```mortar
node TestNode {
    text: "内容"
    events: [
        ← 光标自动缩进到这里
```

#### 5. 文件图标

`.mortar` 文件在文件树中显示专属图标，方便识别。

### 配置选项

在 VS Code 设置中搜索 "Mortar" 可以找到相关配置：

```json
{
  // 设置制表符宽度
  "[mortar]": {
    "editor.tabSize": 4,
    "editor.insertSpaces": true
  },
  
  // 开启自动保存后编译
  "mortar.compileOnSave": true,
  
  // 编译时的参数
  "mortar.compileArgs": ["--pretty"]
}
```

### 快捷键

建议在 VS Code 设置快捷键：

```json
// keybindings.json
[
  {
    "key": "ctrl+shift+b",
    "command": "workbench.action.tasks.build",
    "when": "editorLangId == mortar"
  },
  {
    "key": "f5",
    "command": "mortar.compile",
    "when": "editorLangId == mortar"
  }
]
```

## Language Server Protocol (LSP)

Mortar 提供了 LSP 服务器，支持更高级的 IDE 功能。

### 安装 LSP

```bash
cargo install mortar_lsp
```

### 功能特性

#### 1. 实时错误检查

在你编辑时就能发现错误，不用等到编译：

```mortar
node TestNode {
    text: "你好
    // ↑ 这里会显示红色波浪线：缺少引号
}

node Another {
    choice: [
        "去某处" -> NotExist
        // ↑ 显示黄色波浪线：节点 NotExist 未定义
    ]
}
```

#### 2. 跳转到定义

按住 `Ctrl`/`Cmd` 点击"NodeName"或函数名，跳转到定义处：

```mortar
node Start {
    choice: [
        "下一步" -> NextNode  // ← Ctrl+点击跳转到 NextNode 定义
    ]
}

node NextNode {  // ← 跳到这里
    text: "到了！"
}
```

#### 3. 查找引用

右键点击节点或函数 → "查找所有引用"：

```mortar
fn play_sound(file: String)  // ← 右键这里

// 会列出所有调用 play_sound 的地方：
// - hello.mortar:5:8
// - hello.mortar:12:8
// - world.mortar:3:8
```

#### 4. 自动补全

输入时自动提示：

- 关键字：`node`, `text`, `events` 等
- 已定义的"NodeName"
- 已声明的函数名
- 类型名：`String`, `Bool`, `Number`

```mortar
node Start {
    choice: [
        "去" -> N  // ← 输入 N 时自动提示所有 N 开头的节点
              ↓
           NextNode
           NewPlace
    ]
}
```

#### 5. 悬停提示

鼠标悬停在元素上显示信息：

```mortar
fn get_name() -> String

node TestNode {
    text: $"你好，{get_name()}！"
                     ↑
    // 悬停显示：fn get_name() -> String
}
```

#### 6. 代码诊断

LSP 会分析代码并给出建议：

```mortar
node UnusedNode {  // ⚠️ 警告：此节点未被使用
    text: "内容"
}

fn play_sound(file: String)  // ⚠️ 警告：此函数已声明但未使用
```

### 在不同编辑器中使用 LSP

#### VS Code（自动配置）

安装 Mortar 扩展后自动启用 LSP。

#### Neovim

使用 `nvim-lspconfig`：

```lua
-- 在 init.lua 或 lsp.lua 中
local lspconfig = require('lspconfig')

lspconfig.mortar_lsp.setup{
  cmd = {"mortar-lsp"},
  filetypes = {"mortar"},
  root_dir = lspconfig.util.root_pattern(".git", "*.mortar"),
}

-- 文件类型检测
vim.filetype.add({
  extension = {
    mortar = "mortar",
  },
})
```

#### Emacs

使用 `lsp-mode`：

```elisp
;; 在 init.el 中
(require 'lsp-mode)

(add-to-list 'lsp-language-id-configuration '(mortar-mode . "mortar"))

(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection "mortar-lsp")
  :major-modes '(mortar-mode)
  :server-id 'mortar-lsp))

;; 定义 mortar-mode
(define-derived-mode mortar-mode prog-mode "Mortar"
  "Major mode for Mortar language.")

(add-to-list 'auto-mode-alist '("\\.mortar\\'" . mortar-mode))
```

#### Sublime Text

使用 LSP 插件：

1. 安装 `LSP` 包
2. 创建 `Packages/User/LSP-mortar.sublime-settings`：

```json
{
  "clients": {
    "mortar-lsp": {
      "enabled": true,
      "command": ["mortar-lsp"],
      "selector": "source.mortar"
    }
  }
}
```

3. 创建语法文件 `Packages/User/Mortar.sublime-syntax`（基础版）：

```yaml
%YAML 1.2
---
name: Mortar
file_extensions: [mortar]
scope: source.mortar

contexts:
  main:
    - match: '\b(node|nd|text|events|choice|fn|function|when|return|break)\b'
      scope: keyword.control.mortar
    - match: '"'
      push: string
    - match: '//'
      push: line_comment

  string:
    - meta_scope: string.quoted.double.mortar
    - match: '"'
      pop: true

  line_comment:
    - meta_scope: comment.line.mortar
    - match: $
      pop: true
```

## 手动语法高亮配置

如果你用的编辑器没有专门的支持，可以临时使用类似语言的高亮：

- **类似 Rust**：关键字和结构相似
- **类似 JavaScript**：函数调用和对象结构
- **类似 JSON**：数据格式

## 推荐的编辑器设置

### 通用建议

```
制表符：使用空格
缩进：4 空格
编码：UTF-8
行尾：LF（Unix 风格）
```

### VS Code 设置

```json
{
  "[mortar]": {
    "editor.tabSize": 4,
    "editor.insertSpaces": true,
    "editor.formatOnSave": true,
    "files.encoding": "utf8",
    "files.eol": "\n"
  }
}
```

## 主题推荐

Mortar 语法在这些主题下显示效果较好：

- **One Dark Pro** - 深色主题，对比明显
- **Dracula Official** - 颜色丰富
- **Monokai** - 经典配色
- **Solarized Light** - 浅色主题

## 常见问题

### Q: 为什么语法高亮不生效？

A: 
1. 确保文件后缀是 `.mortar`
2. 重新加载窗口（VS Code: `Ctrl+Shift+P` → "Reload Window"）
3. 检查扩展是否正确安装

### Q: LSP 没有提示？

A:
1. 确保 `mortar-lsp` 已安装：`mortar-lsp --version`
2. 查看 LSP 日志（VS Code: 输出面板 → Mortar LSP）
3. 尝试重启 LSP 服务器

### Q: 可以自定义代码片段吗？

A: 可以！在 VS Code 中：
1. `Ctrl+Shift+P` → "Preferences: Configure User Snippets"
2. 选择 "mortar.json"
3. 添加自定义片段

```json
{
  "My Custom Node": {
    "prefix": "mynode",
    "body": [
      "node ${1:name} {",
      "    text: \"${2:content}\"",
      "    events: [",
      "        0, ${3:function}()",
      "    ]",
      "}$0"
    ]
  }
}
```

## 贡献

想为 Mortar 编辑器支持做贡献？

- VS Code 扩展源码：`vscode-mortar-extension/`
- LSP 服务器源码：`crates/mortar_lsp/`

欢迎提交 PR！

## 小结

Mortar 的编辑器支持：
- ✅ VS Code 官方扩展
- ✅ LSP 服务器支持多种编辑器
- ✅ 语法高亮
- ✅ 代码补全
- ✅ 错误检查
- ✅ 跳转和引用查找

好的工具让写对话更轻松！

## 接下来

- 了解编译工具：[命令行工具](./6_1_cli.md)
- 查看输出格式：[JSON 输出说明](../7_1_json-output.md)
- 回到快速开始：[五分钟上手](../2_quick-start.md)
