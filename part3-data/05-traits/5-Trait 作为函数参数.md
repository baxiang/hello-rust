## 16.5 Trait 作为函数参数

### impl Trait 语法（推荐）

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

// 使用 impl Trait 语法
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet { /* ... */ };
    let article = NewsArticle { /* ... */ };

    notify(&tweet);
    notify(&article);
}
```

### 等同于泛型约束

```rust
// 以下两种写法在大多数情况下等价

// impl Trait 语法（更简洁）
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 泛型约束语法（更明确）
pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

### 多个 impl Trait 参数

```rust
// 每个 impl Trait 是独立的类型
pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// 等价于
pub fn notify_two<T: Summary>(item1: &T, item2: &T) {
    // item1 和 item2 必须是同一类型
}

pub fn notify_two<T: Summary, U: Summary>(item1: &T, item2: &U) {
    // item1 和 item2 可以是不同类型
}
```

---

---

## 下一步

[Trait 约束](../6-Trait 约束.md)