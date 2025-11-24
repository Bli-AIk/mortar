# Localization Strategy

Mortar v0.4 recommends maintaining multiple language files rather than mixing translations inside a single Mortar script. This section describes a practical workflow for teams shipping multilingual dialogue.

## Separate Mortar Sources per Locale

Organize your repository like this:

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

Each language folder mirrors the same node names so your engine can swap `.mortared` builds based on runtime locale. Mortar does not attempt to translate strings automatically; it simply keeps structure consistent.

## Share Logic via Constants and Functions

Use `pub const` for UI labels and `fn` declarations for reusable hooks so every locale references the same identifiers:

```mortar
pub const continue_label: String = "Continue"
fn play_sound(file: String)
```

Translators can copy these declarations verbatim and only edit the human-readable text nodes. Because `pub const` entries appear in the top-level JSON, tooling can detect missing translations easily.

## Building Docs for Each Locale

This repository already mirrors documentation under `book/en` and `book/zh-Hans`. Follow the same convention for gameplay scripts: add one mdBook per locale or create localized guides under `docs/` so your internal contributors know where to make changes.

## Shipping Workflow

1. Author or update the source language (`locales/en`).
2. Sync node/layout changes across other locales.
3. Run `cargo run -p mortar_cli -- locales/en/story.mortar --pretty` for every locale directory.
4. Bundle the resulting `.mortared` artifacts with your game build.

Because Mortar’s syntax enforces separation between text and logic, you never have to duplicate event wiring or conditionals—only the strings change. This makes localization predictable while still allowing advanced constructs like [branch interpolation](./4_6_branch-interpolation.md) for gendered or region-specific flavor text.
