# Mortar Documentation

This directory contains the documentation for the Mortar language, built with mdBook.

**Note**: The English documentation has been rebuilt based on the Chinese documentation (zh-Hans). Some advanced chapters are still being translated and will be completed soon.

## Structure

```
docs/
├── book.toml              # English documentation config
├── src/                   # English markdown sources (rebuilt from zh-Hans)
├── theme/                 # Shared theme files (souprune-hekatonhemeron)
├── zh-Hans/               # Chinese documentation (source of truth)
│   ├── book.toml         # Chinese documentation config  
│   └── src/              # Chinese markdown sources
└── book/                 # Generated documentation
    ├── en/               # Built English docs
    └── zh-Hans/          # Built Chinese docs
```

## Building Documentation

### Prerequisites

```bash
# Install mdBook
cargo install mdbook

# Install optional plugins (for enhanced features)
cargo install mdbook-mermaid
cargo install mdbook-admonish
```

### Building

```bash
# Build English documentation
cd docs
mdbook build

# Build Chinese documentation  
cd docs/zh-Hans
mdbook build
```

### Local Development

```bash
# Serve English docs locally (http://localhost:3000)
cd docs
mdbook serve

# Serve Chinese docs locally (http://localhost:3001)
cd docs/zh-Hans
mdbook serve --port 3001
```

## Theme

The documentation uses the [souprune-hekatonhemeron](https://github.com/Bli-AIk/souprune-hekatonhemeron) theme, which provides:

- Clean, modern appearance
- Catppuccin color scheme
- Good readability for code examples
- Responsive design

## Deployment

Documentation is automatically deployed via GitHub Actions to GitHub Pages:

- **English**: `https://your-domain.com/en/`
- **Chinese**: `https://your-domain.com/zh-Hans/`
- **Root**: `https://your-domain.com/` (auto-redirects based on browser language)

## Translation Status

### Completed ✅
- Introduction
- Quick Start Guide  
- Installation
- Core Concepts (all subsections)
- Basic Examples
- Command Line Interface
- Editor Support

### In Progress 🚧
- Interactive Story Examples (placeholder created)
- Game Integration Guide (placeholder created)
- JSON Output Format (placeholder created)
- FAQ (placeholder created)
- Contributing Guide (placeholder created)

## Contributing

When contributing to documentation:

1. **Primary Language**: Chinese documentation in `docs/zh-Hans/src/` is the source of truth
2. **English Translation**: Edit files in `docs/src/` based on Chinese version
3. Maintain parallel structure between languages
4. Test builds locally before submitting PRs
5. Follow existing formatting and style conventions

## Content Guidelines

- Use clear, concise language
- Include practical code examples
- Cross-reference related sections
- Keep examples up-to-date with the language specification
- Use admonitions (callouts) for important information