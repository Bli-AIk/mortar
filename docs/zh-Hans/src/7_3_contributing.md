# 贡献指南

感谢你对 Mortar 的兴趣！我们欢迎各种形式的贡献。

## 贡献方式

你可以通过以下方式为 Mortar 做贡献：

- 🐛 **报告 Bug** - 发现问题就告诉我们
- ✨ **提议新功能** - 分享你的创意
- 📝 **改进文档** - 让文档更清晰
- 💻 **提交代码** - 修复 Bug 或实现新功能
- 🌍 **翻译** - 帮助翻译文档
- 💬 **回答问题** - 在 Discussions 帮助其他人
- ⭐ **分享项目** - 让更多人知道 Mortar

## 行为准则

参与 Mortar 社区时，请：

- ✅ 保持友善和尊重
- ✅ 欢迎新手
- ✅ 接受建设性批评
- ✅ 专注于对社区最有益的事情

我们致力于提供一个友好、安全和欢迎所有人的环境。

## 报告 Bug

### 在报告前

1. **搜索已有 Issues** - 确认问题是否已被报告
2. **更新到最新版本** - 确认问题在最新版本中仍然存在
3. **准备最小复现示例** - 尽可能简化问题

### 如何报告

前往 [GitHub Issues](https://github.com/Bli-AIk/mortar/issues/new) 创建新 Issue。

**好的 Bug 报告应该包含**：

```markdown
## 描述
简短描述问题

## 复现步骤
1. 创建这样一个文件...
2. 运行这个命令...
3. 看到错误...

## 期望行为
应该发生什么

## 实际行为
实际发生了什么

## 最小复现示例
```mortar
// 能复现问题的最小代码
node TestNode {
    text: "..."
}
```

## 环境信息
- Mortar 版本：0.3.0
- 操作系统：Windows 11 / macOS 14 / Ubuntu 22.04
- Rust 版本（如果从源码构建）：1.75.0
```

**示例**：

```markdown
## 描述
编译带有空选项列表的节点时崩溃

## 复现步骤
1. 创建文件 `test.mortar`
2. 写入以下内容：
   ```mortar
   node TestNode {
       text: "你好"
       choice: []
   }
   ```
3. 运行 `mortar test.mortar`
4. 程序崩溃

## 期望行为
应该给出友好的错误提示："选项列表不能为空"

## 实际行为
程序直接崩溃，显示：
```
thread 'main' panicked at 'index out of bounds'
```

## 环境信息
- Mortar 版本：0.3.0
- 操作系统：Windows 11
```

## 提议新功能

### 在提议前

1. **搜索已有 Issues** - 确认功能是否已被提议
2. **思考必要性** - 这个功能对大多数用户有用吗？
3. **考虑替代方案** - 是否有其他实现方式？

### 如何提议

前往 [GitHub Discussions](https://github.com/Bli-AIk/mortar/discussions) 发起讨论。

**好的功能提议应该包含**：

```markdown
## 问题/需求
描述你遇到的问题或想解决的需求

## 提议的解决方案
详细描述你希望添加的功能

## 示例
展示功能的使用方式

## 替代方案
是否考虑过其他实现方式？

## 影响
这个功能会影响现有用户吗？
```

**示例**：

```markdown
## 问题/需求
写大型对话时，经常需要在多个文件之间共享函数声明，
目前需要在每个文件里重复声明，很麻烦。

## 提议的解决方案
增加 import 语法，可以从其他文件导入函数声明：

```mortar
import functions from "common_functions.mortar"

node MyNode {
    text: "触发共用逻辑"
    with events: [
        0, play_sound("test.wav")  // 这个函数来自 common_functions.mortar
    ]
}
```

## 替代方案
1. 使用预处理器合并文件
2. 在游戏引擎层面解决

## 影响
不会影响现有代码，因为现在不支持 import 关键字
```

## 改进文档

文档在 `docs/` 目录下，使用 Markdown 编写。

### 文档类型

- **教程** - 适合新手的循序渐进指南
- **操作指南** - 解决特定问题的步骤
- **参考** - 详尽的技术说明
- **解释** - 概念和设计思想

### 改进文档的步骤

1. Fork 仓库
2. 创建分支：`git checkout -b improve-docs`
3. 编辑文档文件
4. 本地预览：`mdbook serve docs/zh-Hans` 或 `mdbook serve docs/en`
5. 提交更改：`git commit -m "docs: 改进安装说明"`
6. 推送分支：`git push origin improve-docs`
7. 创建 Pull Request

### 文档风格指南

- **清晰简洁** - 用简单的语言解释复杂概念
- **友好的语气** - 像和朋友聊天一样
- **实际示例** - 提供可运行的代码
- **循序渐进** - 从简单到复杂
- **视觉辅助** - 适当使用图表、emoji
- **代码格式** - 使用语法高亮

**好的文档**：
```markdown
## 创建第一个对话

让我们写一段简单的 NPC 对话：

```mortar
node Villager {
    text: "你好，旅行者！"
}
```

就这么简单！保存文件后编译：

```bash
mortar hello.mortar
```
```

**不好的文档**：
```markdown
## 节点创建

创建节点使用 node 关键字，后跟标识符和块。
块内使用 text 字段定义文本内容。
编译使用 mortar 命令加文件名参数。
```

## 提交代码

### 开发环境设置

1. **安装 Rust**（1.70+）：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **克隆仓库**：
   ```bash
   git clone https://github.com/Bli-AIk/mortar.git
   cd mortar
   ```

3. **构建项目**：
   ```bash
   cargo build
   ```

4. **运行测试**：
   ```bash
   cargo test
   ```

5. **代码检查**：
   ```bash
   cargo clippy
   cargo fmt --check
   ```

### 项目结构

```
mortar/
├── crates/
│   ├── mortar_compiler/  # 编译器核心
│   ├── mortar_cli/       # 命令行工具
│   ├── mortar_lsp/       # 语言服务器
│   └── mortar_language/  # 主库
├── docs/                 # 文档
└── tests/                # 集成测试
```

### 开发流程

1. **创建 Issue** - 描述你要做的改动
2. **Fork 仓库**
3. **创建特性分支**：
   ```bash
   git checkout -b feature/my-feature
   # 或
   git checkout -b fix/bug-description
   ```

4. **进行开发**：
   - 编写代码
   - 添加测试
   - 运行测试确保通过
   - 使用 clippy 检查代码

5. **提交更改**：
   ```bash
   git add .
   git commit -m "feat: 添加新功能"
   ```

6. **推送分支**：
   ```bash
   git push origin feature/my-feature
   ```

7. **创建 Pull Request**

### 提交信息规范

使用约定式提交（Conventional Commits）：

```
<类型>(<范围>): <描述>

[可选的正文]

[可选的脚注]
```

**类型**：
- `feat` - 新功能
- `fix` - Bug 修复
- `docs` - 文档更改
- `style` - 代码格式（不影响代码运行）
- `refactor` - 重构
- `test` - 添加测试
- `chore` - 构建过程或辅助工具的变动

**示例**：

```
feat(compiler): 添加对嵌套选项的支持

增加了解析嵌套选项的能力，现在可以写：
choice: [
    "选项" -> [
        "子选项" -> Node
    ]
]

Closes #42
```

```
fix(cli): 修复 Windows 上的路径问题

在 Windows 上编译时路径分隔符错误导致编译失败。
现在使用 std::path::PathBuf 正确处理路径。

Fixes #38
```

### 代码风格

遵循 Rust 标准风格：

```bash
# 格式化代码
cargo fmt

# 检查代码
cargo clippy -- -D warnings
```

**代码注释**：

```rust
// 好的注释：解释为什么，不是做什么
// 使用哈希表而不是向量，因为需要 O(1) 的查找速度
let mut nodes = HashMap::new();

// 不好的注释：重复代码内容
// 创建一个新的 HashMap
let mut nodes = HashMap::new();
```

**命名规范**：

```rust
// 使用 snake_case
fn parse_node() { }
let node_name = "test";

// 类型使用 PascalCase
struct NodeData { }
enum TokenType { }

// 常量使用 SCREAMING_SNAKE_CASE
const MAX_DEPTH: usize = 10;
```

### 测试

为新功能添加测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_node() {
        let input = r#"
            node TestNode {
                text: "Hello"
            }
        "#;
        
        let result = parse(input);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0].name, "Test");
    }
}
```

### Pull Request

**好的 PR 应该**：

- ✅ 解决单一问题
- ✅ 包含测试
- ✅ 更新相关文档
- ✅ 通过所有 CI 检查
- ✅ 有清晰的描述

**PR 描述模板**：

```markdown
## 改动内容
简短描述这个 PR 做了什么

## 动机
为什么需要这个改动？解决了什么问题？

## 改动类型
- [ ] Bug 修复
- [ ] 新功能
- [ ] 文档更新
- [ ] 代码重构
- [ ] 其他：___

## 测试
如何测试这个改动？

## 相关 Issue
Closes #issue_number

## 截图（如适用）
```

### Code Review

提交 PR 后：

1. **CI 检查** - 确保所有自动化测试通过
2. **等待审核** - 维护者会审查你的代码
3. **响应反馈** - 根据建议进行修改
4. **合并** - 审核通过后会合并到主分支

## 翻译文档

想帮助翻译文档到其他语言？太好了！

### 当前支持的语言

- 🇨🇳 简体中文（zh-Hans）
- 🇬🇧 English（en）

### 添加新语言

1. 在 `docs/` 下创建新目录：`docs/your-language/`
2. 复制 `book.toml` 并修改语言设置
3. 翻译 `src/` 目录下的所有 `.md` 文件
4. 测试构建：`mdbook build docs/your-language`
5. 提交 PR

### 翻译指南

- **保持结构一致** - 不要改变文档结构
- **本地化示例** - 根据文化背景调整示例
- **专业术语** - 保持术语一致性
- **代码不翻译** - 代码示例保持英文
- **链接更新** - 确保内部链接指向对应语言的页面

## 社区

### 获取帮助

- 💬 [GitHub Discussions](https://github.com/Bli-AIk/mortar/discussions) - 提问和讨论
- 🐛 [GitHub Issues](https://github.com/Bli-AIk/mortar/issues) - 报告 Bug
- 📧 Email - 见项目 README

### 保持联系

- ⭐ Star 项目关注更新
- 👀 Watch 仓库接收通知
- 🔔 订阅 Release 通知

## 许可证

贡献的代码将采用与项目相同的许可证：

- MIT License
- Apache License 2.0

提交 PR 即表示你同意在这些许可证下分发你的贡献。

## 致谢

感谢所有贡献者！你们的帮助让 Mortar 变得更好 ❤️

贡献者列表见项目 README。

---

再次感谢你的贡献！如有任何问题，随时在 Discussions 提问 🎉
