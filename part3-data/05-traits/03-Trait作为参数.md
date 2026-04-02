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







