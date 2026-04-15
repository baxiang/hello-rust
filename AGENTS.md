# AGENTS.md

## Project Type

Chinese Rust tutorial (31 chapters + 9 projects + 3 modern practice chapters). Markdown documentation with per-chapter Cargo projects for runnable examples.

## Directory Structure

```
1-basics/      # Chapters 1-6 (intro → control flow)
2-core/        # Chapters 7-11 (ownership, references, slices, structs, enums)
3-data/        # Chapters 12-19 (collections, hashmap, errors, generics, traits, lifetimes, closures, iterators)
4-advanced/    # Chapters 20-28 (modules, cargo, smart pointers, concurrency, unsafe, macros, cli, web, testing)
5-projects/    # 9 projects: docs only, no Cargo.toml
6-modern/      # Chapters 29-31 (Rust 2024, async, WebAssembly)
exercises/     # Exercise markdown files per chapter
docs/          # style-guide.md, diagrams, templates, FAQ
scripts/       # verify-code.sh, check-links.sh
playground/    # Sample Cargo project
```

## Chapter Cargo Structure

Every chapter (except 5-projects) has its own `Cargo.toml` + `examples/` directory:

```bash
cd 1-basics/03-variables
cargo run --example 01-let-binding   # Run single example
cargo check --examples               # Compile-check all examples
cargo clippy --examples              # Lint check
```

- 32 chapters with Cargo.toml, 96+ example files total
- All use `edition = "2024"`, minimum Rust 1.85+
- 5-projects/ is docs-only (no Cargo projects)

## Verification

```bash
./scripts/verify-code.sh             # Scans all chapters, runs cargo check + clippy
./scripts/check-links.sh             # Requires npm markdown-link-check
```

## Markdown Conventions

- `✅ 正确示例` / `❌ 错误示例` annotations in code blocks
- ASCII art (`┌─┐` box style) for memory diagrams
- All code blocks specify language (`rust`, `bash`, `toml`)
- Chinese terms keep English reference (e.g., "所有权 Ownership")
- Full rules: `docs/style-guide.md`

## Code Style for Examples

- Public APIs: explicit type annotations
- Error handling: `anyhow` for apps, `thiserror` for libraries
- Avoid `unwrap()` in examples; use `expect()` with context
- Prefer idiomatic Rust: Iterator chains, `?` operator, pattern matching

## Adding Content

1. Create subdirectory in appropriate part (e.g., `3-data/09-new-topic/`)
2. Create `Cargo.toml` (edition = "2024") and `examples/` directory
3. Follow `docs/style-guide.md` structure templates
4. Update `README.md` navigation links

## Key Files

- `README.md` - Tutorial index with all chapter links
- `docs/style-guide.md` - Markdown format + Rust learning strategy + example conventions
- `CONTRIBUTING.md` - Contribution workflow

## Version

- Minimum Rust: 1.85+ (2024 Edition)
- Tutorial version: 2.3
