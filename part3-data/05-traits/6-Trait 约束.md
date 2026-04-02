## 16.6 Trait 约束

### 单个约束

```rust
fn print_item<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}
```

### 多个约束（使用 +）

```rust
// T 必须同时实现 Summary 和 Display
pub fn notify(item: &(impl Summary + std::fmt::Display)) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}

// 泛型语法
pub fn notify<T: Summary + std::fmt::Display>(item: &T) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}
```

### where 子句

```rust
// 复杂约束使用 where 更清晰
fn some_function<T, U>(t: T, u: U) -> String
where
    T: Summary + std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone + PartialEq,
{
    format!("t: {}, u: {:?}", t, u)
}
```

### 约束关联类型

```rust
use std::fmt::Display;

// 约束 Iterator 的 Item 类型
fn print_all<T>(iter: T)
where
    T: IntoIterator,
    T::Item: Display,
{
    for item in iter {
        println!("{}", item);
    }
}

fn main() {
    print_all(vec![1, 2, 3]);
    print_all(vec!["a", "b", "c"]);
}
```

---

---

## 下一步

[返回实现 Trait 的类型](../7-返回实现 Trait 的类型.md)