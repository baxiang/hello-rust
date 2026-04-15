# AGENTS.md

## Project Type

Chinese Rust tutorial (31 chapters + 9 projects + 3 modern practice chapters). Primarily Markdown documentation. Small `playground/` dir contains a sample Cargo project for testing.

## Directory Structure

```
1-basics/      # Chapters 1-6
2-core/        # Chapters 7-11 (ownership, references, slices, structs, enums)
3-data/        # Chapters 12-19 (collections, hashmap, errors, generics, traits, lifetimes, closures, iterators)
4-advanced/    # Chapters 20-28 (modules, cargo, smart pointers, concurrency, unsafe, macros, cli, web, testing)
5-projects/    # 9 projects: todo-cli, file-search, rest-api, log-analyzer, chat-room, kv-store, web-crawler, image-processor, interpreter
6-modern/      # Chapters 29-31 (Rust 2024, async, WebAssembly)
exercises/     # Exercise files per chapter
docs/          # Style guide, diagrams, templates, FAQ
scripts/       # verify-code.sh, check-links.sh
playground/    # Sample Cargo project for testing
```

## Verification Commands

```bash
./scripts/verify-code.sh   # Creates temp Cargo project, validates code examples
./scripts/check-links.sh   # Requires npm markdown-link-check installed
```

## Markdown Conventions

- `✅ 正确示例` and `❌ 错误示例` annotations in code blocks
- ASCII art (`┌─┐` box style) for memory diagrams
- All code blocks must specify language (`rust`, `bash`, `toml`)
- Chinese terms keep English reference (e.g., "所有权 Ownership")
- Full formatting rules in `docs/style-guide.md`

## Code Style for Examples

- Public APIs: explicit type annotations
- Error handling: `anyhow` for apps, `thiserror` for libraries
- Avoid `unwrap()` in examples; use `expect()` with context

## Adding Content

1. Create subdirectory in appropriate part (e.g., `3-data/09-new-topic/`)
2. Follow `docs/style-guide.md` structure templates
3. Update `README.md` navigation links

## Key Files

- `README.md` - Tutorial index with all chapter links
- `docs/style-guide.md` - Markdown format + Rust learning strategy
- `CONTRIBUTING.md` - Contribution workflow

## Version

- Minimum Rust: 1.85+ (2024 Edition)
- Tutorial version: 2.3