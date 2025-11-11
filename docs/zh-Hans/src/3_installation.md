# 安装

通过这些安装选项让 Mortar 在您的系统上运行。

## 系统要求

- **Rust**: 1.70 或更高版本（如果从源代码构建）
- **Git**: 用于克隆仓库（如果从源代码构建）

## 选项 1: 从 Crates.io 安装（推荐）

安装 Mortar 最简单的方法是从官方 Rust 包注册表：

```bash
# 安装完整的 Mortar CLI
cargo install mortar_cli

# 验证安装
mortar --version
```

您也可以安装单独的组件：

```bash
# 用于 IDE 支持的语言服务器
cargo install mortar_lsp

# 仅核心库（用于 Rust 项目）
# 添加到您的 Cargo.toml：
[dependencies]
mortar_language = "0.3"
```

## 选项 2: 从源代码构建

获取最新的开发功能或贡献代码：

```bash
# 克隆仓库
git clone https://github.com/Bli-AIk/mortar.git
cd mortar

# 构建所有组件
cargo build --release

# 全局安装 CLI
cargo install --path crates/mortar_cli

# 安装 LSP 服务器
cargo install --path crates/mortar_lsp
```

## 选项 3: 下载预构建二进制文件

预构建的二进制文件可在 [GitHub Releases 页面](https://github.com/Bli-AIk/mortar/releases) 上获得：

1. 下载适合您平台的二进制文件
2. 解压归档文件
3. 将二进制文件添加到您的系统 PATH

## 验证安装

测试 Mortar 是否正确安装：

```bash
# 检查 CLI 版本
mortar --version

# 尝试编译一个简单的脚本
echo 'node Test { text: "你好世界!" }' > test.mortar
mortar test.mortar --pretty
```

## IDE 支持设置

### Visual Studio Code

1. 从 VS Code 市场安装 Mortar 扩展
2. 扩展会自动使用您安装的 `mortar_lsp` 服务器

### 其他 IDE

对于支持语言服务器协议的 IDE：

1. 安装 `mortar_lsp`：
   ```bash
   cargo install mortar_lsp
   ```

2. 配置您的 IDE 为 `.mortar` 文件使用 `mortar_lsp`
3. 将语言 ID 设置为 `mortar`

## 开发依赖（可选）

用于贡献 Mortar 开发：

```bash
# 安装开发工具
cargo install mdbook          # 用于文档
cargo install cargo-tarpaulin # 用于测试覆盖率
cargo install cargo-audit     # 用于安全审计

# 运行开发命令
cargo test                    # 运行测试
cargo clippy                  # 代码检查
cargo fmt                     # 格式化代码
```

## 故障排除

### 常见问题

**找不到 `mortar` 命令**
- 确保 `~/.cargo/bin` 在您的 PATH 中
- 运行 `source ~/.bashrc` 或重启终端

**在旧版 Rust 上构建失败**
- 更新 Rust：`rustup update stable`
- 最低要求版本是 Rust 1.70

**LSP 在 IDE 中不工作**
- 验证 `mortar_lsp` 已安装：`which mortar_lsp`
- 检查 IDE LSP 配置
- 安装后重启您的 IDE

**Unix 系统上权限被拒绝**
- 确保二进制文件可执行：`chmod +x mortar`

### 获取帮助

- **GitHub Issues**: [报告错误或提问](https://github.com/Bli-AIk/mortar/issues)
- **讨论**: [社区支持](https://github.com/Bli-AIk/mortar/discussions)
- **文档**: 您正在阅读！

## 下一步

现在 Mortar 已安装：

1. 尝试[快速开始](./quick-start.md)指南
2. 探索[基本概念](./basic-concepts.md)
3. 设置您的 [IDE 支持](./ide-support.md)
4. 阅读[最佳实践](./best-practices.md)