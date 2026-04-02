# Edition 机制

> 理解 Rust Edition 的工作原理和演进策略。

## 什么是 Edition？

Rust Edition 是 Rust 语言的版本标识系统，允许语言在不破坏现有代码的情况下引入破坏性变更。

```rust
// 在 Cargo.toml 中指定 Edition
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"  // 使用 Rust 2024 Edition
```

### Edition 历史

| Edition | 发布年份 | 主要特性 |
|---------|---------|---------|
| 2015 | 2015 | 初始版本 |
| 2018 | 2018 | 模块系统改进、错误处理增强 |
| 2021 | 2021 | 闭包改进、数组 IntoIterator |
| 2024 | 2024 | 异步闭包、impl Trait 增强 |

## 为什么需要 Edition？

### 问题场景

```rust
// Rust 2015: 关键字可以用于变量名
let async = 1;  // ✅ 合法

// Rust 2018+: async 成为关键字
// let async = 1;  // ❌ 编译错误
```

如果没有 Edition 机制，引入 `async` 关键字会破坏现有代码。

### Edition 解决方案

```rust
// 项目 A: 使用 Rust 2015
// edition = "2015"
let async = 1;  // ✅ 仍然合法

// 项目 B: 使用 Rust 2024
// edition = "2024"
let async_value = 1;  // ✅ 必须避免使用关键字
```

## 兼容性保证

### 编译器版本 vs Edition

```
┌─────────────────────────────────────────────┐
│        Rust 编译器（rustc）                  │
│  ┌────────┬────────┬────────┬────────┐    │
│  │ 2015   │ 2018   │ 2021   │ 2024   │    │
│  │ Edition│ Edition│ Edition│ Edition│    │
│  └────────┴────────┴────────┴────────┘    │
│                                             │
│  编译器支持所有 Edition，自动检测版本         │
└─────────────────────────────────────────────┘
```

**关键原则：**
- 新编译器支持所有旧 Edition
- 不同 Edition 的代码可以互操作
- Edition 选择是项目级的，不是文件级的

### 跨 Edition 互操作

```rust
// lib.rs (edition = "2024")
pub fn process() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}

// main.rs (edition = "2018")
use my_lib::process;

fn main() {
    for item in process() {  // ✅ 跨 Edition 调用
        println!("{}", item);
    }
}
```

## Edition 与 Rust 版本

### 版本号系统

Rust 使用语义化版本：
- **Stable**: 1.75.0, 1.76.0, ... (每 6 周发布)
- **Beta**: 下一版本的预览
- **Nightly**: 最新特性（可能不稳定）

### Edition 发布周期

```
Edition 2015 → Edition 2018 → Edition 2021 → Edition 2024
    (3 年)         (3 年)         (3 年)
```

**重要：**
- Edition 不是编译器版本
- Edition 是语言特性的集合
- 可以在使用旧 Edition 的编译器版本上使用新 Edition 特性（通常需要最低版本）

## 迁移策略

### 1. 自动迁移

```bash
# Cargo 提供自动迁移工具
cargo fix --edition

# 自动修复大部分兼容性问题
```

### 2. 手动检查

```bash
# 编译并查看警告
cargo build

# 运行测试
cargo test
```

### 3. 增量迁移

```rust
// 可以在模块级别指定 Edition（实验性）
#![feature(rust_2024_prelude_collisions)]
```

## 常见问题

### 何时应该升级 Edition？

```
✅ 推荐升级：
- 新项目直接使用最新 Edition
- 想使用新特性
- 维护成本低的项目

⏸️ 暂缓升级：
- 依赖兼容性问题
- 大型遗留代码库
- 临近发布周期的项目
```

### 不同 Edition 代码可以混合吗？

```rust
// ❌ 不可以在同一个 crate 内混合 Edition
// 每个 crate 只能有一个 Edition

// ✅ 但不同 crate 可以使用不同 Edition
// 依赖树中可以有不同 Edition 的 crate
```

### Edition 会影响性能吗？

不会。Edition 只影响语法解析和语义检查，不影响编译后的机器码性能。

## 最佳实践

### 1. 新项目使用最新 Edition

```toml
[package]
name = "my_new_project"
version = "0.1.0"
edition = "2024"  // ✅ 使用最新稳定 Edition
```

### 2. 渐进式迁移

```bash
# 步骤 1: 更新 Cargo.toml
# edition = "2024"

# 步骤 2: 运行自动修复
cargo fix --edition --allow-dirty

# 步骤 3: 手动处理警告
cargo build

# 步骤 4: 运行测试
cargo test
```

### 3. 使用 rust-analyzer

```json
// .vscode/settings.json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

rust-analyzer 会提示 Edition 相关的迁移建议。

## 小结

**Edition 机制的核心价值：**
- 允许语言演进而不破坏现有代码
- 提供渐进式升级路径
- 保持生态兼容性

**关键要点：**
- Edition ≠ 编译器版本
- 新编译器支持所有旧 Edition
- 不同 Edition 的代码可以互操作
- 升级 Edition 通常是安全的

**下一步：**
下一节我们将学习 Rust 2024 Edition 的具体新特性。

## 练习

### 练习 1：检查当前 Edition

创建一个新项目并查看默认 Edition：

```bash
cargo new edition_test
cd edition_test
cat Cargo.toml | grep edition
```

### 练习 2：模拟迁移

1. 创建一个 Rust 2021 项目
2. 故意使用一些 2021 语法
3. 尝试迁移到 2024 Edition
4. 观察自动修复的行为

### 练习 3：阅读迁移指南

查看官方迁移文档：
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)