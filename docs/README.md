# Mortar Documentation

This directory contains the documentation for the Mortar language, built with mdBook.

## Structure

```
docs/
├── book.toml              # English documentation config
├── src/                   # English markdown sources
├── theme/                 # Shared theme files (souprune-hekatonhemeron)
├── zh-Hans/               # Chinese documentation
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

## Contributing

When contributing to documentation:

1. **English**: Edit files in `docs/src/`
2. **Chinese**: Edit files in `docs/zh-Hans/src/`
3. Maintain parallel structure between languages
4. Test builds locally before submitting PRs
5. Follow existing formatting and style conventions

## Content Guidelines

- Use clear, concise language
- Include practical code examples
- Cross-reference related sections
- Keep examples up-to-date with the language specification
- Use admonitions (callouts) for important information