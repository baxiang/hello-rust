# AGENTS.md - Rust 教程项目指南

本文档为在此仓库工作的 AI 编码代理提供指导。

## 项目概述

这是一个 **Rust 编程语言中文教程**，包含 28 章 + 9 个实战项目。项目为纯文档项目（无可编译的 Rust 代码），使用 Markdown 格式编写教程内容。

**重要**：此仓库不包含实际的 Rust 项目代码（Cargo.toml、.rs 文件等），仅包含教程 Markdown 文档。

---

## 构建/检查/测试命令

由于本项目为文档项目，以下命令主要用于验证文档中的代码示例：

### Rust 代码验证（在临时项目中）

```bash
# 创建临时项目验证代码示例
cargo new temp_test && cd temp_test
cargo build                  # 编译检查
cargo check                  # 快速检查（不生成二进制）
cargo test                   # 运行测试
cargo test test_name         # 运行单个测试
cargo test -- --nocapture    # 显示测试输出
cargo test -- --test-threads=1  # 单线程运行测试
cargo clippy                 # 代码 lint 检查
cargo fmt -- --check         # 格式检查
cargo doc --no-deps          # 生成文档
```

### 文档格式检查

```bash
# Markdown 格式检查（如使用 markdownlint）
markdownlint '**/*.md'

# 链接检查
markdown-link-check README.md
```

---

## 代码风格指南

### Rust 代码示例风格

文档中的 Rust 代码示例应遵循以下规范：

#### 1. 导入组织

```rust
// 标准库优先
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

// 外部 crate 其次
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

// 当前模块最后
use super::*;
use crate::models::User;
```

#### 2. 格式规范

- 使用 `cargo fmt` 标准格式
- 最大行宽：100 字符
- 缩进：4 空格
- 函数/结构体之间空一行

```rust
// ✅ 正确示例
fn process_data(input: &str) -> Result<String, Error> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(Error::EmptyInput);
    }
    Ok(trimmed.to_uppercase())
}

// ❌ 避免
fn process_data(input:&str)->Result<String,Error>{
    let trimmed=input.trim();
    Ok(trimmed.to_uppercase())
}
```

#### 3. 类型标注

- 公开 API 必须显式标注类型
- 推荐使用类型别名提高可读性

```rust
// ✅ 推荐
pub type Result<T> = std::result::Result<T, MyError>;

pub fn parse_config(path: &str) -> Result<Config> {
    // ...
}

// 内部函数可省略简单类型推断
let count = items.len();  // OK
```

#### 4. 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 结构体 | PascalCase | `UserConfig` |
| 枚举 | PascalCase | `HttpStatus` |
| 函数/方法 | snake_case | `parse_input` |
| 变量 | snake_case | `user_name` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS` |
| 模块 | snake_case | `network_utils` |
| 泛型参数 | 单个大写字母或 PascalCase | `T`, `K`, `V`, `Context` |

```rust
// ✅ 正确命名
const MAX_RETRY_COUNT: u32 = 3;

struct UserSession {
    session_id: String,
    created_at: DateTime,
}

fn create_user_session(user_id: u64) -> UserSession {
    // ...
}

// ❌ 避免
const maxRetryCount = 3;  // 常量应大写
struct userSession { }    // 结构体应 PascalCase
fn CreateUserSession() { } // 函数应 snake_case
```

#### 5. 错误处理

- 应用层使用 `anyhow::Result`
- 库层使用 `thiserror` 自定义错误
- 避免 `unwrap()`，使用 `expect()` 并提供上下文

```rust
// ✅ 推荐 - 库代码
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Missing field: {0}")]
    MissingField(String),
}

pub fn parse(input: &str) -> Result<Data, ParseError> {
    // ...
}

// ✅ 推荐 - 应用代码
fn main() -> anyhow::Result<()> {
    let config = read_config("config.toml")
        .context("Failed to read configuration")?;
    Ok(())
}

// ❌ 避免
let file = File::open("data.txt").unwrap();  // 可能 panic
```

#### 6. 注释风格

- 文档注释使用 `///`
- 普通注释使用 `//`
- 示例代码标注正确/错误

```rust
/// 计算两个数的最大值
///
/// # Arguments
/// * `a` - 第一个数
/// * `b` - 第二个数
///
/// # Examples
/// ```
/// let max = find_max(5, 10);
/// assert_eq!(max, 10);
/// ```
pub fn find_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// ✅ 正确示例
fn good_example() { /* ... */ }

// ❌ 错误示例
fn bad_example() { /* ... */ }
```

---

## Markdown 文档风格

### 标题层级

```markdown
# 章节标题 (一级)
> 章节简介（引用块）

## 主要节 (二级)
### 子节 (三级)
```

### 代码块

- 必须标注语言类型
- 使用 `✅ 正确` 和 `❌ 错误` 标注示例

```markdown
```rust
// ✅ 正确示例
fn valid_code() { }

// ❌ 错误示例（附带错误信息）
fn invalid_code() { }
```
```

### 可视化图表

使用 ASCII 艺术绘制：

```markdown
┌─────────────────────────────────────────────────────┐
│              内存布局图                               │
├─────────────────────────────────────────────────────┤
│  ┌─────┐  ┌─────┐                                   │
│  │ ptr │──│ data│                                   │
│  └─────┘  └─────┘                                   │
└─────────────────────────────────────────────────────┘
```

### 章节结构模板

每个教程章节应包含：

1. 概念介绍（引用块开头）
2. 代码示例（正确/错误对比）
3. 可视化图解（内存布局/流程图）
4. 常见错误解析
5. 练习题
6. 小结

---

## 常用 Crate 推荐

参考 `part4-advanced/02-cargo/03-Cargo命令.md` 的推荐列表：

| 类别 | 推荐 Crate |
|------|-----------|
| 序列化 | `serde`, `serde_json`, `toml` |
| Web 框架 | `axum` (推荐), `actix-web` |
| CLI | `clap` (derive features) |
| 错误处理 | `anyhow` (应用), `thiserror` (库) |
| 异步运行时 | `tokio` |
| 日志 | `tracing`, `tracing-subscriber` |

---

## 文件组织

### 目录结构

```
hello-rust/
├── part1-basics/      # 基础入门 (第 1-6 章)
├── part2-core/        # 核心概念 (第 7-11 章)
├── part3-data/        # 数据结构 (第 12-19 章)
├── part4-advanced/    # 高级主题 (第 20-28 章)
├── part5-projects/    # 项目实战 (9 个项目)
└── README.md          # 教程目录
```

### 添加新内容

1. 在对应目录创建子目录（如 `09-new-topic/`）
2. 创建 `README.md` 或分节 `.md` 文件
3. 更新主 README.md 的导航链接
4. 保持与现有章节风格一致

---

## 中文术语对照

保留英文原文术语对照：

| 中文 | 英文 |
|------|------|
| 所有权 | Ownership |
| 借用 | Borrowing |
| 生命周期 | Lifetime |
| 泛型 | Generics |
| Trait | Trait (保留英文) |
| 模块 | Module |
| Crate | Crate (保留英文) |
| 智能指针 | Smart Pointer |
| 闭包 | Closure |

---

## 版本信息

- 教程最低 Rust 版本：1.75+
- 教程版本：2.2
- Markdown 格式：GitHub Flavored Markdown

---

## 相关文件

- `README.md` - 教程总目录
- `QWEN.md` - 项目详细上下文（已存在）