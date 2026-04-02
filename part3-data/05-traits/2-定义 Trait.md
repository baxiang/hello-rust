## 16.2 定义 Trait

### 基本语法

```rust
// Trait 定义
pub trait Summary {
    // 抽象方法（必须由实现者提供）
    fn summarize(&self) -> String;
}

// Trait 可以有多个方法
pub trait Formatter {
    // 抽象方法
    fn format(&self) -> String;

    // 抽象方法
    fn format_json(&self) -> String;
}
```

### 方法签名

```rust
trait Example {
    // 使用&self（最常见）
    fn read_only(&self);

    // 使用&mut self（修改自身）
    fn modify(&mut self);

    // 使用 self（获取所有权）
    fn consume(self) -> i32;

    // 关联函数（类似静态方法）
    fn create() -> Self;

    // 带参数的方法
    fn process(&self, data: &str) -> bool;
}
```

### 关联类型

```rust
// Trait 可以有类型占位符
trait Iterator {
    type Item;  // 关联类型

    fn next(&mut self) -> Option<Self::Item>;
}

// 实现时指定具体类型
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;  // 指定关联类型

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}
```

---

---

## 下一步

[实现 Trait](../3-实现 Trait.md)