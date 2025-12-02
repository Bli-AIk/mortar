# 参与贡献 Mortar

感谢您有兴趣为 Mortar 做贡献！我们非常感谢您的帮助，让这个项目变得更好。

## Pull Requests (PR)

*   **小问题**：对于小的错误修复、文档拼写错误或细微的改进，请直接提交 Pull Request。
*   **大问题**：对于重大的更改、新功能或大规模的重构，请务必**先开启一个 Issue** 进行讨论。这可以确保您的工作符合项目的目标，避免白费力气。

## 开发流程

1.  在 GitHub 上 **Fork** 本仓库。
2.  将您的 Fork **Clone** 到本地。
3.  为您的更改建立一个新的 **Branch**。
4.  进行更改，并遵守项目的代码风格。
5.  执行 **测试** 以确保一切正常：
    ```bash
    cargo test
    ```
6.  将您的更改 Push 到您的 Fork。
7.  向原始仓库的 `main` 分支提交 **Pull Request**。

## 代码风格

*   我们遵循标准的 Rust 格式惯例。提交前请执行 `cargo fmt`。
*   确保您的代码通过 `cargo clippy` 检查：
    ```bash
    cargo clippy --all-targets --all-features
    ```

## 许可协议

贡献 Mortar 即表示您同意您的贡献将根据项目的双重许可协议（MIT 和 Apache 2.0）进行许可。