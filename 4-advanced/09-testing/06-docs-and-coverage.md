## 生成文档

### Cargo doc

```bash
# 生成文档
cargo doc

# 生成并打开文档
cargo doc --open

# 包含私有项
cargo doc --private

# 不包含依赖文档
cargo doc --no-deps
```

### 文档属性

```rust
//! # My Crate
//!
//! 这是一个示例 crate
//!
//! ## 功能
//!
//! - 功能 1
//! - 功能 2
//!
//! ## 示例
//!
//! ```
//! use my_crate::add;
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```

/// 函数文档
#[doc(hidden)]  // 不显示在文档中
pub fn hidden_func() {}

/// 结构体文档
#[derive(Debug)]
pub struct MyStruct {
    /// 字段文档
    pub field: i32,
}

/// Trait 文档
pub trait MyTrait {
    /// 方法文档
    fn method(&self);
}
```






## 测试覆盖率

### 使用 cargo-llvm-cov

```bash
# 安装
cargo install cargo-llvm-cov

# 运行覆盖率测试
cargo llvm-cov

# 生成 HTML 报告
cargo llvm-cov --open

# 生成 LCOV 报告
cargo llvm-cov --lcov --output-path lcov.info
```

### 使用 cargo-tarpaulin

```bash
# 安装
cargo install cargo-tarpaulin

# 运行覆盖率测试
cargo tarpaulin

# 生成 HTML 报告
cargo tarpaulin --out Html

# 生成 XML 报告
cargo tarpaulin --out Xml
```

### 覆盖率报告示例

```
Filename             | Lines | Covered | Missed | Percent
---------------------|-------|---------|--------|--------
src/lib.rs           |   100 |      85 |     15 |   85%
src/utils.rs         |    50 |      40 |     10 |   80%
---------------------|-------|---------|--------|--------|
Total                |   150 |     125 |     25 |   83%
```







