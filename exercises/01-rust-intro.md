# Rust 简介练习题

> 通过实践掌握 Rust 基础概念

---

## 基础题（5题）

### 练习 1：Rust 特点判断

**题目：** 判断以下关于 Rust 的说法是否正确。

1. Rust 是解释型语言
2. Rust 具有零成本抽象特性
3. Rust 必须手动管理内存
4. Rust 可以编译为 WebAssembly
5. Rust 不支持面向对象编程

<details>
<summary>查看答案</summary>

1. ❌ 错误：Rust 是编译型语言
2. ✅ 正确：Rust 的抽象不会带来运行时开销
3. ❌ 错误：Rust 通过所有权系统自动管理内存
4. ✅ 正确：Rust 有优秀的 WebAssembly 支持
5. ❌ 错误：Rust 支持结构体、trait、泛型等面向对象特性

</details>

---

### 练习 2：应用场景选择

**题目：** 为以下场景选择最合适的语言特性。

场景：
- A. 嵌入式系统开发
- B. Web 后端服务
- C. 命令行工具
- D. 浏览器扩展

特性：
1. 零成本抽象 → ___
2. 快速编译 → ___
3. 内存安全 → ___
4. 跨平台支持 → ___

<details>
<summary>查看答案</summary>

1. 零成本抽象 → A（嵌入式系统资源受限）
2. 快速编译 → C（命令行工具需要快速迭代）
3. 内存安全 → A, B, C, D（所有场景都需要）
4. 跨平台支持 → D（浏览器扩展需在不同浏览器运行）

</details>

---

### 练习 3：安装命令

**题目：** 填写安装 Rust 的命令。

```bash
# 安装 rustup
curl --proto '=https' --svvf -sSf https://sh.rustup.rs | sh

# 检查 Rust 版本
rustc --version

# 检查 Cargo 版本
cargo --version

# 更新 Rust
rustup update
```

<details>
<summary>查看答案</summary>

```bash
# 安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 检查 Rust 版本
rustc --version

# 检查 Cargo 版本
cargo --version

# 更新 Rust
rustup update
```

</details>

---

### 练习 4：工具链理解

**题目：** 匹配以下工具的作用。

工具：
- rustc
- cargo
- rustup
- rustfmt

作用：
1. 包管理器和构建工具 → ___
2. Rust 编译器 → ___
3. Rust 版本管理器 → ___
4. 代码格式化工具 → ___

<details>
<summary>查看答案</summary>

1. 包管理器和构建工具 → cargo
2. Rust 编译器 → rustc
3. Rust 版本管理器 → rustup
4. 代码格式化工具 → rustfmt

</details>

---

### 练习 5：Hello World 分析

**题目：** 指出以下代码中的错误。

```rust
fn main() {
    println("Hello, World!");
}
```

<details>
<summary>查看答案</summary>

```rust
fn main() {
    println!("Hello, World!");  // ❌ 缺少感叹号
}
```

**错误说明：**
- `println` 不是函数，而是宏
- 宏调用需要使用 `!` 符号
- 正确写法：`println!("Hello, World!");`

</details>

---

## 进阶题（5题）

### 练习 6：环境配置

**题目：** 配置 Rust 开发环境，设置默认工具链为 stable，并添加 rust-analyzer 组件。

```bash
# 设置默认工具链
rustup __A__ stable

# 安装 rust-analyzer
rustup component __B__ rust-analyzer

# 查看已安装组件
rustup __C__
```

<details>
<summary>查看答案</summary>

```bash
# 设置默认工具链
rustup default stable

# 安装 rust-analyzer
rustup component add rust-analyzer

# 查看已安装组件
rustup show
```

</details>

---

### 练习 7：编译流程

**题目：** 分析 Rust 程序从源码到可执行文件的编译流程。

```
源代码 (.rs)
    ↓
[ 阶段 1 ]
    ↓
[ 阶段 2 ]
    ↓
[ 阶段 3 ]
    ↓
可执行文件
```

填写各阶段名称和作用。

<details>
<summary>查看答案</summary>

```
源代码 (.rs)
    ↓
[ 词法分析与语法分析 ]  # 生成 AST
    ↓
[ 语义分析与类型检查 ]  # 检查类型、所有权等
    ↓
[ 代码生成与链接 ]      # 生成 LLVM IR 并链接
    ↓
可执行文件
```

</details>

---

### 练习 8：Cargo.toml 配置

**题目：** 创建一个 Cargo.toml 配置文件，包含以下信息：
- 项目名称：my_app
- 版本：0.1.0
- 作者：Your Name
- 依赖：serde 1.0 版本

<details>
<summary>查看答案</summary>

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]

[dependencies]
serde = "1.0"
```

</details>

---

### 练习 9：错误处理

**题目：** 分析以下编译错误，找出问题并修复。

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;  // 错误：cannot assign twice to immutable variable
    println!("The value of x is: {}", x);
}
```

<details>
<summary>查看答案</summary>

```rust
fn main() {
    let mut x = 5;  // ✅ 添加 mut 关键字
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

**错误原因：**
- Rust 中变量默认不可变
- 需要使用 `mut` 关键字声明可变变量

</details>

---

### 练习 10：跨平台编译

**题目：** 如何为 Linux x86_64 目标编译 Rust 程序？

```bash
# 添加目标平台
rustup __A__ add x86_64-unknown-linux-gnu

# 编译到目标平台
cargo __B__ --target x86_64-unknown-linux-gnu
```

<details>
<summary>查看答案</summary>

```bash
# 添加目标平台
rustup target add x86_64-unknown-linux-gnu

# 编译到目标平台
cargo build --target x86_64-unknown-linux-gnu
```

</details>

---

## 挑战题（3题）

### 练习 11：项目结构设计

**题目：** 创建一个命令行计算器项目的目录结构。

要求：
1. 使用 Cargo 初始化项目
2. 创建 lib.rs 和 main.rs
3. 添加 README.md
4. 配置 gitignore

写出所有命令和文件内容。

<details>
<summary>查看答案</summary>

```bash
# 创建项目
cargo new calculator --name calc

# 进入项目目录
cd calculator

# 创建 lib.rs
touch src/lib.rs
```

项目结构：
```
calculator/
├── Cargo.toml
├── .gitignore
├── README.md
└── src/
    ├── lib.rs
    └── main.rs
```

.gitignore:
```
/target
Cargo.lock
```

README.md:
```markdown
# Calculator

A simple command-line calculator written in Rust.

## Usage

```bash
cargo run -- 1 + 2
```
```

</details>

---

### 练习 12：工具链切换

**题目：** 在项目中同时使用 stable 和 nightly 工具链。

要求：
1. 安装 nightly 工具链
2. 为项目设置 nightly 为默认
3. 在 nightly 中使用不稳定特性

<details>
<summary>查看答案</summary>

```bash
# 安装 nightly
rustup install nightly

# 设置项目使用 nightly
rustup override set nightly

# 或在项目根目录创建 rust-toolchain 文件
echo "nightly" > rust-toolchain
```

使用不稳定特性示例：
```rust
// 允许使用不稳定特性
#![feature(try_blocks)]

fn main() {
    let result: Option<i32> = try {
        let x = Some(5)?;
        let y = Some(10)?;
        x + y
    };
    println!("{:?}", result);  // Some(15)
}
```

</details>

---

### 练习 13：自定义开发环境

**题目：** 配置完整的 Rust 开发环境，包括：
1. 代码格式化（rustfmt）
2. 代码检查（clippy）
3. 自动补全（rust-analyzer）
4. 预提交钩子（cargo-husky）

<details>
<summary>查看答案</summary>

```bash
# 安装工具
rustup component add rustfmt clippy rust-analyzer
cargo install cargo-husky
```

Cargo.toml 添加：
```toml
[dev-dependencies]
cargo-husky = { version = "1", features = ["precommit-hooks", "run-cargo-test", "run-cargo-clippy", "run-cargo-fmt"] }
```

.husky/pre-commit:
```bash
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

</details>

---

## 小结

本章练习涵盖了：
- Rust 语言特点与应用场景
- 开发环境配置
- Cargo 项目管理
- 编译流程理解

完成所有练习后，你应该能够：
- 独立配置 Rust 开发环境
- 创建和管理 Rust 项目
- 理解 Rust 的基本概念
- 解决常见编译错误

下一章：[第一个程序练习题](./02-first-program.md)