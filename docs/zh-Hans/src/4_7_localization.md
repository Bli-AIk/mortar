# 本地化策略

Mortar v0.4 建议为每种语言维护独立的 Mortar 脚本，而不是把所有翻译混在同一个文件里。本章节总结了一套易于协作的流程。

## 每种语言独立目录

推荐的仓库结构：

```
mortar/
├─ locales/
│  ├─ en/
│  │  └─ story.mortar
│  ├─ zh-Hans/
│  │  └─ story.mortar
│  └─ ja/
│     └─ story.mortar
```

每个语言文件夹保持相同的节点名称，这样运行时只需根据语言代码选择对应的 `.mortared` 构建即可。

## 共享逻辑与常量

通过 `pub const` 与函数声明共享 UI 文案与逻辑钩子：

```mortar
pub const continue_label: String = "继续"
fn play_sound(file: String)
```

翻译人员只需复制声明并修改具体文本。由于常量会写入顶层 JSON，本地化工具也能快速检测缺失项。

## 文档与脚本的多语言

仓库已经在 `book/en` 与 `book/zh-Hans` 维护对应的 mdBook。编写游戏脚本时同样遵循这个约定，为贡献者提供清晰的落点。

## 发布流程

1. 更新源语言（通常是 `locales/en`）。
2. 将节点及结构变更同步到其他语言。
3. 分别运行 `cargo run -p mortar_cli -- locales/<lang>/story.mortar --pretty`。
4. 将生成的 `.mortared` 文件与游戏一起发布。

Mortar 天生将文本与逻辑分离，所以本地化过程中无需复制事件或条件，只需维护不同语言的文本内容即可。如需在同一语言中处理性别或地区差异，可结合 [分支插值](./4_6_branch-interpolation.md) 提供更细致的体验。
