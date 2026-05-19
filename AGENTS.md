# AGENTS.md

## Project Type

Chinese Rust tutorial (31 chapters + 9 projects + 3 modern practice chapters). Markdown documentation with per-chapter Cargo projects for runnable code examples.

## Directory Layout

```
1-basics/      # Ch 1-6   (intro → control flow)
2-core/        # Ch 7-11  (ownership, references, slices, structs, enums)
3-data/        # Ch 12-19 (collections → iterators)
4-advanced/    # Ch 20-28 (modules → testing)
5-projects/    # 9 projects — docs only, NO Cargo.toml
6-modern/      # Ch 29-31 (Rust 2024, async, WebAssembly)
exercises/     # Exercise markdown files per chapter
docs/          # style-guide.md, templates, FAQ
scripts/       # verify-code.sh, check-links.sh
playground/    # 01-hello-world Cargo project
```

## Working with Chapter Code

Every chapter (except `5-projects/`) is an independent Cargo project with an `examples/` directory. 31 chapters + 1 playground = 32 Cargo.toml files, 101 example `.rs` files total.

```bash
cd 1-basics/03-variables
cargo run --example 01-let-binding   # Run single example
cargo check --examples               # Compile-check all examples
cargo clippy --examples              # Lint (CI uses -- -D warnings)
```

**Cargo.toml naming**: `chapter-<two-digit-number>-<topic>` (e.g., `chapter-03-variables`). All use `edition = "2024"`, minimum Rust 1.85+. `Cargo.lock` is gitignored.

**5-projects/** has no Cargo projects — each project is split into multiple markdown files (e.g., `01-项目概述.md`, `02-实现步骤.md`, `03-运行与扩展.md`, `04-测试策略.md`).

## Verification Commands

```bash
./scripts/verify-code.sh             # Iterates all chapters: cargo check --examples + cargo clippy --examples -- -D warnings
./scripts/check-links.sh             # Requires npm markdown-link-check; uses .markdown-link-check.json config
```

**Note**: `verify-code.sh` treats clippy warnings as errors (`-D warnings`). The CI workflow (`.github/workflows/ci.yml`) is currently a skeleton — most steps are placeholder `echo` statements.

## Markdown & Code Conventions

- `✅ 正确示例` / `❌ 错误示例` annotations in code blocks
- ASCII art (`┌─┐` box style) for memory diagrams and syntax structure diagrams
- All code blocks specify language (`rust`, `bash`, `toml`)
- Chinese terms keep English reference (e.g., "所有权 Ownership"); "Trait" and "Crate" always kept in English
- Full rules: `docs/style-guide.md`

## Code Style for Examples

- Public APIs: explicit type annotations
- Error handling: `anyhow` for apps, `thiserror` for libraries
- Avoid `unwrap()` in examples; use `expect()` with context
- Prefer idiomatic Rust: Iterator chains, `?` operator, pattern matching
- Example files: top-level `//!` module doc comment, `// ✅` for correct, commented-out `// ❌` for error examples
- Run `cargo fmt` and `cargo clippy` before committing

## Adding a New Chapter

1. Create subdirectory in appropriate part (e.g., `3-data/09-new-topic/`)
2. Create `Cargo.toml` with `name = "chapter-<NN>-<topic>"`, `edition = "2024"`, and `examples/` directory
3. Follow `docs/style-guide.md` structure templates (syntax diagram → minimal example → detailed example → comprehensive example)
4. Update `README.md` navigation links
