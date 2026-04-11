# AGENTS.md

## Project Type

**Pure documentation project** - no Cargo.toml, no .rs files. This is a Rust tutorial written in Markdown (31 chapters + 9 projects + 3 modern practice chapters).

## Directory Structure

```
1-basics/      # Chapters 1-6: intro, variables, types, functions, control flow
2-core/        # Chapters 7-11: ownership, references, slices, structs, enums
3-data/        # Chapters 12-19: collections, hashmap, errors, generics, traits, lifetimes, closures, iterators
4-advanced/    # Chapters 20-28: modules, cargo, smart pointers, concurrency, unsafe, macros, cli, web, testing
5-projects/    # 9 projects: todo-cli, file-search, rest-api, log-analyzer, chat-room, kv-store, web-crawler, image-processor, interpreter
6-modern/      # Chapters 29-31: Rust 2024, async, WebAssembly
exercises/     # Exercise files per chapter
docs/          # Style guide, diagrams, FAQ
scripts/       # verify-code.sh, check-links.sh
```

## Verification Commands

```bash
./scripts/verify-code.sh   # Creates temp Cargo project to validate code examples
./scripts/check-links.sh   # Check markdown links
```

## Markdown Conventions

- Use `✅ 正确示例` and `❌ 错误示例` annotations in code blocks
- Use ASCII art (`┌─┐` box style) for memory diagrams
- All code blocks must specify language (`rust`, `bash`, `toml`)
- Reference: `docs/style-guide.md` for full formatting rules

## Code Style for Examples

Follow standard Rust conventions. Key specifics:
- Public APIs: explicit type annotations
- Error handling: `anyhow` for apps, `thiserror` for libraries
- Avoid `unwrap()` in examples; use `expect()` with context

## Crate Recommendations

| Category | Recommended |
|----------|-------------|
| Serialization | serde, serde_json, toml |
| Web | axum, actix-web |
| CLI | clap (derive) |
| Errors | anyhow (app), thiserror (lib) |
| Async | tokio |
| Logging | tracing |

## Adding Content

1. Create subdirectory in appropriate part (e.g., `3-data/09-new-topic/`)
2. Create `.md` files following `docs/style-guide.md` structure
3. Update `README.md` navigation links
4. Ensure Chinese terms keep English reference (e.g., "所有权 Ownership")

## Version

- Minimum Rust: 1.75+ (modern chapters need 1.85+)
- Tutorial version: 2.3

## Related Files

- `README.md` - Tutorial index
- `docs/style-guide.md` - Full Markdown style guide
- `CONTRIBUTING.md` - Contribution workflow