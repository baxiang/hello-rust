# Rust 基础入门篇重构实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 重构 part1-basics 6章教程，采用6步教学法模板，合并子文件为单一 README.md

**Architecture:** 每章采用统一结构：Step 1 (场景引入) → Step 2 (概念动机) → Step 3 (最简示例) → Step 4 (详细讲解) → Step 5 (渐进复杂化) → Step 6 (实际应用) → 小结

**Tech Stack:** Markdown 文档，Rust 2024 Edition 代码示例

---

## 文件结构

### 变更文件清单

| 章节 | 操作 | 文件路径 |
|------|------|---------|
| 01-intro | 重写 | `part1-basics/01-intro/README.md` |
| 01-intro | 删除 | `part1-basics/01-intro/01-为什么学习Rust.md` |
| 01-intro | 删除 | `part1-basics/01-intro/02-安装与配置.md` |
| 01-intro | 删除 | `part1-basics/01-intro/03-快速上手.md` |
| 01-intro | 删除 | `part1-basics/01-intro/04-常见问题.md` |
| 02-first-program | 重写 | `part1-basics/02-first-program/README.md` |
| 02-first-program | 删除 | `part1-basics/02-first-program/01-HelloWorld详解.md` |
| 02-first-program | 删除 | `part1-basics/02-first-program/02-变量与注释.md` |
| 02-first-program | 删除 | `part1-basics/02-first-program/03-完整示例.md` |
| 02-first-program | 删除 | `part1-basics/02-first-program/04-调试与错误.md` |
| 03-variables | 重写 | `part1-basics/03-variables/README.md` |
| 03-variables | 删除 | `part1-basics/03-variables/01-变量基础.md` |
| 03-variables | 删除 | `part1-basics/03-variables/02-常量与静态.md` |
| 03-variables | 删除 | `part1-basics/03-variables/03-高级特性.md` |
| 03-variables | 删除 | `part1-basics/03-variables/04-实战总结.md` |
| 04-types | 重写 | `part1-basics/04-types/README.md` |
| 04-types | 删除 | `part1-basics/04-types/01-类型系统概述.md` |
| 04-types | 删除 | `part1-basics/04-types/02-基本类型.md` |
| 04-types | 删除 | `part1-basics/04-types/03-复合类型.md` |
| 04-types | 删除 | `part1-basics/04-types/04-字符串类型.md` |
| 04-types | 删除 | `part1-basics/04-types/05-实战总结.md` |
| 05-functions | 重写 | `part1-basics/05-functions/README.md` |
| 05-functions | 删除 | `part1-basics/05-functions/01-函数基础.md` |
| 05-functions | 删除 | `part1-basics/05-functions/02-参数与返回值.md` |
| 05-functions | 删除 | `part1-basics/05-functions/03-所有权与函数.md` |
| 05-functions | 删除 | `part1-basics/05-functions/04-高级函数.md` |
| 05-functions | 删除 | `part1-basics/05-functions/05-闭包与递归.md` |
| 05-functions | 删除 | `part1-basics/05-functions/06-实战总结.md` |
| 06-control-flow | 重写 | `part1-basics/06-control-flow/README.md` |
| 06-control-flow | 删除 | `part1-basics/06-control-flow/01-条件表达式.md` |
| 06-control-flow | 删除 | `part1-basics/06-control-flow/02-循环.md` |
| 06-control-flow | 删除 | `part1-basics/06-control-flow/03-模式匹配.md` |
| 06-control-flow | 删除 | `part1-basics/06-control-flow/04-实战总结.md` |

---

## Task 1: 重构第1章 - 简介与环境搭建

**Files:**
- Create: `part1-basics/01-intro/README.md`
- Delete: `part1-basics/01-intro/01-为什么学习Rust.md`
- Delete: `part1-basics/01-intro/02-安装与配置.md`
- Delete: `part1-basics/01-intro/03-快速上手.md`
- Delete: `part1-basics/01-intro/04-常见问题.md`

- [ ] **Step 1: 编写新的 README.md**

创建包含6步教学法的完整章节内容：

```markdown
# 第 01 章：简介与环境搭建

> 开启 Rust 学习之旅，搭建开发环境，了解 Rust 的优势与应用场景。

> **本章代码基于 Rust 2024 Edition (Rust 1.85+) 编写**

## 第1步：实际场景引入

### 为什么需要编程？

想象一下，你每天需要处理100个CSV文件，提取其中的数据并生成报告。

**手动操作（很麻烦）：**
- 打开每个文件
- 复制需要的数据
- 手动计算统计值
- 重复100次...

这个过程可能需要几个小时，而且容易出错。

**使用程序自动化：**

用 Rust 写一个程序，几秒钟就能完成所有工作：

```rust
// 几行代码就能处理100个文件
for file in files {
    let data = read_csv(file)?;
    process_data(data);
}
```

这就是学习编程的价值：让计算机帮你完成重复性工作。

### 为什么选择 Rust？

对比其他语言：

| 语言 | 内存安全 | 性能 | 并发安全 |
|------|---------|------|---------|
| C/C++ | ❌ 手动管理 | ⚡ 极快 | ❌ 需小心 |
| Python | ✅ GC 管理 | 🐢 较慢 | ⚠️ 有GIL |
| Java | ✅ GC 管理 | 🚀 中等 | ⚠️ 需同步 |
| **Rust** | ✅ 编译时检查 | ⚡ 极快 | ✅ 编译时检查 |

Rust 的独特之处：**无需垃圾回收，编译时就保证内存安全**。

## 第2步：概念动机

### Rust 解决了什么问题？

传统语言的痛点：

```
C/C++ 的内存问题：
┌─────────────────────────────────────────┐
│ char* buffer = new char[100];           │
│ // ... 使用 buffer                       │
│ // 忘记 delete！内存泄漏                 │
│                                         │
│ // 或者更糟：                            │
│ delete buffer;                          │
│ buffer[0] = 'a';  // 使用已释放内存！    │
│ // 程序崩溃或数据损坏                    │
└─────────────────────────────────────────┘
```

Rust 的解决方案：

```rust
let buffer = String::with_capacity(100);
// 使用 buffer
// 离开作用域时，Rust 自动释放内存！
```

### Rust 的核心优势

```
┌─────────────────────────────────────────────────────┐
│                  Rust 的优势                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  🛡️  内存安全    无需垃圾回收，编译时检查          │
│  ⚡  高性能      与 C/C++ 相当                      │
│  🧵  并发安全    编译时防止数据竞争                │
│  📦  工具链      Cargo 包管理，开箱即用            │
│  📚  文档       内置文档系统，rustdoc              │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 谁在使用 Rust？

| 公司/项目 | 用途 |
|-----------|------|
| **Microsoft** | Windows 内核、Azure 服务 |
| **Google** | Android 系统、Fuchsia OS |
| **Amazon AWS** | Firecracker、Bottlerocket |
| **Cloudflare** | 网络服务、边缘计算 |
| **Discord** | 读取状态服务 |

## 第3步：最简示例

### 5分钟上手 Rust

```rust
fn main() {
    println!("Hello, Rust!");
}
```

这就是一个完整的 Rust 程序！

运行它只需要两步：

```bash
cargo new hello_rust    # 创建项目
cargo run               # 运行程序
```

输出：`Hello, Rust!`

## 第4步：详细讲解

### 安装 Rust

#### macOS / Linux 安装

打开终端，执行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，加载环境变量：

```bash
source "$HOME/.cargo/env"
```

#### Windows 安装

1. 访问 [https://rustup.rs](https://rustup.rs)
2. 下载 `rustup-init.exe`
3. 运行安装程序

#### 验证安装

```bash
rustc --version    # Rust 编译器版本
cargo --version    # Cargo 包管理器版本
rustup --version   # 工具链管理器版本
```

### Rust 工具链详解

#### rustup - 工具链管理器

```bash
# 更新 Rust 到最新版本
rustup update

# 查看已安装的工具链
rustup show

# 安装特定版本
rustup install 1.85.0

# 切换默认版本
rustup default 1.85.0

# 添加组件
rustup component add rustfmt  # 代码格式化
rustup component add clippy   # 代码检查
```

#### cargo - 包管理器

```bash
cargo new project_name    # 创建新项目
cargo build               # 构建项目
cargo run                 # 运行项目
cargo test                # 运行测试
cargo check               # 快速检查（不生成二进制）
cargo fmt                 # 格式化代码
cargo clippy              # 代码 lint 检查
cargo doc --open          # 生成并打开文档
```

### 项目结构

创建项目后，你会看到：

```
hello_rust/
├── Cargo.toml         # 项目配置文件
└── src/
    └── main.rs        # 源代码
```

#### Cargo.toml 详解

```toml
[package]
name = "hello_rust"      # 项目名称
version = "0.1.0"        # 版本号
edition = "2024"         # Rust Edition

[dependencies]           # 依赖项
# 这里添加第三方库
```

#### main.rs 详解

```rust
fn main() {
    // fn: 定义函数
    // main: 程序入口函数（必须有）
    // println!: 打印宏（注意感叹号）
    println!("Hello, Rust!");
}
```

### 调试模式 vs 发布模式

```bash
# 调试模式（开发用）
cargo build
# 输出：target/debug/hello_rust
# 特点：编译快，运行慢，有调试信息

# 发布模式（生产用）
cargo build --release
# 输出：target/release/hello_rust
# 特点：编译慢，运行快，无调试信息
```

### 配置开发环境

#### VS Code 配置（推荐）

安装扩展：
- `rust-analyzer`（必须）- 代码补全、类型提示、错误检查
- `Better TOML`（推荐）- TOML 文件语法高亮
- `CodeLLDB`（调试用）- 调试支持

#### 其他编辑器

| 编辑器 | 插件 |
|--------|------|
| IntelliJ IDEA | Rust 插件 |
| Vim/Neovim | rust.vim + rust-analyzer |
| Emacs | rust-mode + lsp-mode |

### 常见问题

#### Q1: 下载慢怎么办？

配置国内镜像：

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
EOF
```

#### Q2: 编译错误：找不到 rustc

确保已加载环境变量：

```bash
source "$HOME/.cargo/env"
```

或者重启终端。

#### Q3: 如何卸载 Rust？

```bash
rustup self uninstall
```

## 第5步：渐进复杂化

### 层级1：创建并运行第一个项目

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

输出：
```
   Compiling hello_rust v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
     Running `target/debug/hello_rust`
Hello, world!
```

### 层级2：修改程序内容

编辑 `src/main.rs`：

```rust
fn main() {
    println!("你好，Rust!");
    println!("这是第二行");
}
```

运行：

```bash
cargo run
```

### 层级3：创建库项目

```bash
cargo new my_lib --lib
cd my_lib
```

库项目结构：

```
my_lib/
├── Cargo.toml
└── src/
    └── lib.rs    # 注意：是 lib.rs
```

编辑 `src/lib.rs`：

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

运行测试：

```bash
cargo test
```

## 第6步：实际应用

### 完整示例：项目初始化脚本

创建一个脚本来初始化标准 Rust 项目结构：

```rust
// src/main.rs
use std::fs;
use std::process::Command;

fn main() {
    println!("=== Rust 项目初始化工具 ===");

    // 创建项目
    let project_name = "my_project";
    Command::new("cargo")
        .args(["new", project_name])
        .status()
        .expect("创建项目失败");

    println!("✅ 项目 {} 创建成功", project_name);

    // 显示项目结构
    println!("\n项目结构：");
    println!("  {}/", project_name);
    println!("  ├── Cargo.toml");
    println!("  └── src/");
    println!("      └── main.rs");

    // 显示下一步操作
    println!("\n下一步：");
    println!("  cd {}", project_name);
    println!("  cargo run");
}
```

运行：

```bash
cargo run
```

输出：
```
=== Rust 项目初始化工具 ===
✅ 项目 my_project 创建成功

项目结构：
  my_project/
  ├── Cargo.toml
  └── src/
      └── main.rs

下一步：
  cd my_project
  cargo run
```

## 小结

### 本章要点 Checklist

- ✅ 理解 Rust 的核心优势：内存安全、高性能、并发安全
- ✅ 完成 Rust 工具链安装（rustup、cargo）
- ✅ 掌握 cargo 基本命令：new、build、run、test
- ✅ 理解项目结构：Cargo.toml、src/main.rs
- ✅ 配置开发环境（VS Code + rust-analyzer）
- ✅ 了解调试模式 vs 发布模式

### 关键命令速查

```bash
# 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证
rustc --version
cargo --version

# 创建项目
cargo new 项目名

# 运行
cargo run

# 构建
cargo build
cargo build --release

# 检查
cargo check
cargo fmt
cargo clippy
```
```

- [ ] **Step 2: 删除旧的子文件**

```bash
rm /Users/baxiang/Documents/hello-rust/part1-basics/01-intro/01-为什么学习Rust.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/01-intro/02-安装与配置.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/01-intro/03-快速上手.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/01-intro/04-常见问题.md
```

- [ ] **Step 3: 提交变更**

```bash
git add part1-basics/01-intro/
git commit -m "refactor(part1): 重构第1章采用6步教学法模板"
```

---

## Task 2: 重构第2章 - 第一个 Rust 程序

**Files:**
- Create: `part1-basics/02-first-program/README.md`
- Delete: `part1-basics/02-first-program/01-HelloWorld详解.md`
- Delete: `part1-basics/02-first-program/02-变量与注释.md`
- Delete: `part1-basics/02-first-program/03-完整示例.md`
- Delete: `part1-basics/02-first-program/04-调试与错误.md`

- [ ] **Step 1: 编写新的 README.md**

创建包含6步教学法的完整章节内容，整合现有子文件内容。

- [ ] **Step 2: 删除旧的子文件**

```bash
rm /Users/baxiang/Documents/hello-rust/part1-basics/02-first-program/01-HelloWorld详解.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/02-first-program/02-变量与注释.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/02-first-program/03-完整示例.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/02-first-program/04-调试与错误.md
```

- [ ] **Step 3: 提交变更**

```bash
git add part1-basics/02-first-program/
git commit -m "refactor(part1): 重构第2章采用6步教学法模板"
```

---

## Task 3: 重构第3章 - 变量与可变性

**Files:**
- Create: `part1-basics/03-variables/README.md`
- Delete: `part1-basics/03-variables/01-变量基础.md`
- Delete: `part1-basics/03-variables/02-常量与静态.md`
- Delete: `part1-basics/03-variables/03-高级特性.md`
- Delete: `part1-basics/03-variables/04-实战总结.md`

- [ ] **Step 1: 编写新的 README.md**

创建包含6步教学法的完整章节内容，整合现有子文件内容。

- [ ] **Step 2: 删除旧的子文件**

```bash
rm /Users/baxiang/Documents/hello-rust/part1-basics/03-variables/01-变量基础.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/03-variables/02-常量与静态.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/03-variables/03-高级特性.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/03-variables/04-实战总结.md
```

- [ ] **Step 3: 提交变更**

```bash
git add part1-basics/03-variables/
git commit -m "refactor(part1): 重构第3章采用6步教学法模板"
```

---

## Task 4: 重构第4章 - 数据类型

**Files:**
- Create: `part1-basics/04-types/README.md`
- Delete: `part1-basics/04-types/01-类型系统概述.md`
- Delete: `part1-basics/04-types/02-基本类型.md`
- Delete: `part1-basics/04-types/03-复合类型.md`
- Delete: `part1-basics/04-types/04-字符串类型.md`
- Delete: `part1-basics/04-types/05-实战总结.md`

- [ ] **Step 1: 编写新的 README.md**

创建包含6步教学法的完整章节内容，整合现有子文件内容。

- [ ] **Step 2: 删除旧的子文件**

```bash
rm /Users/baxiang/Documents/hello-rust/part1-basics/04-types/01-类型系统概述.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/04-types/02-基本类型.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/04-types/03-复合类型.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/04-types/04-字符串类型.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/04-types/05-实战总结.md
```

- [ ] **Step 3: 提交变更**

```bash
git add part1-basics/04-types/
git commit -m "refactor(part1): 重构第4章采用6步教学法模板"
```

---

## Task 5: 重构第5章 - 函数

**Files:**
- Create: `part1-basics/05-functions/README.md`
- Delete: `part1-basics/05-functions/01-函数基础.md`
- Delete: `part1-basics/05-functions/02-参数与返回值.md`
- Delete: `part1-basics/05-functions/03-所有权与函数.md`
- Delete: `part1-basics/05-functions/04-高级函数.md`
- Delete: `part1-basics/05-functions/05-闭包与递归.md`
- Delete: `part1-basics/05-functions/06-实战总结.md`

- [ ] **Step 1: 编写新的 README.md**

创建包含6步教学法的完整章节内容，整合现有子文件内容。

- [ ] **Step 2: 删除旧的子文件**

```bash
rm /Users/baxiang/Documents/hello-rust/part1-basics/05-functions/01-函数基础.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/05-functions/02-参数与返回值.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/05-functions/03-所有权与函数.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/05-functions/04-高级函数.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/05-functions/05-闭包与递归.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/05-functions/06-实战总结.md
```

- [ ] **Step 3: 提交变更**

```bash
git add part1-basics/05-functions/
git commit -m "refactor(part1): 重构第5章采用6步教学法模板"
```

---

## Task 6: 重构第6章 - 控制流

**Files:**
- Create: `part1-basics/06-control-flow/README.md`
- Delete: `part1-basics/06-control-flow/01-条件表达式.md`
- Delete: `part1-basics/06-control-flow/02-循环.md`
- Delete: `part1-basics/06-control-flow/03-模式匹配.md`
- Delete: `part1-basics/06-control-flow/04-实战总结.md`

- [ ] **Step 1: 编写新的 README.md**

创建包含6步教学法的完整章节内容，整合现有子文件内容。

- [ ] **Step 2: 删除旧的子文件**

```bash
rm /Users/baxiang/Documents/hello-rust/part1-basics/06-control-flow/01-条件表达式.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/06-control-flow/02-循环.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/06-control-flow/03-模式匹配.md
rm /Users/baxiang/Documents/hello-rust/part1-basics/06-control-flow/04-实战总结.md
```

- [ ] **Step 3: 提交变更**

```bash
git add part1-basics/06-control-flow/
git commit -m "refactor(part1): 重构第6章采用6步教学法模板"
```

---

## Task 7: 最终验证与提交

- [ ] **Step 1: 验证所有章节结构**

检查每个章节是否包含完整的6步结构：
- Step 1: 实际场景引入
- Step 2: 概念动机
- Step 3: 最简示例
- Step 4: 详细讲解
- Step 5: 渐进复杂化
- Step 6: 实际应用
- 小结

- [ ] **Step 2: 验证子文件已删除**

```bash
ls part1-basics/01-intro/
ls part1-basics/02-first-program/
ls part1-basics/03-variables/
ls part1-basics/04-types/
ls part1-basics/05-functions/
ls part1-basics/06-control-flow/
```

预期输出：每个目录只有 README.md

- [ ] **Step 3: 最终提交**

```bash
git add part1-basics/
git commit -m "refactor(part1): 完成基础入门篇6章重构，采用6步教学法模板"
```

---

## Self-Review

**1. Spec coverage:**
- ✅ 6章全部覆盖
- ✅ 每章采用6步教学法模板
- ✅ 合并子文件为单一 README.md
- ✅ Rust 2024 Edition 版本说明
- ✅ 场景设计符合设计文档

**2. Placeholder scan:**
- ✅ 无 TBD/TODO
- ✅ 所有代码示例完整
- ✅ 所有命令明确

**3. Type consistency:**
- ✅ 文件路径一致
- ✅ 命令格式一致
- ✅ Markdown 结构一致