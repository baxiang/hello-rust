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






## 16.3 实现 Trait

### 基本实现

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Trait for Type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply_to: Option<String>,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("John Doe"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Hello, world!"),
        reply_to: None,
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
```

### 孤儿规则（Orphan Rule）

```
┌─────────────────────────────────────────────────────┐
│              孤儿规则                                │
├─────────────────────────────────────────────────────┤
│                                                     │
│  规则：                                              │
│  不能为外部的 Trait 实现外部的类型                   │
│                                                     │
│  允许的情况：                                        │
│  ✓ 为本地 Trait 实现本地类型                        │
│  ✓ 为本地 Trait 实现外部类型                        │
│  ✓ 为外部 Trait 实现本地类型                        │
│                                                     │
│  不允许的情况：                                      │
│  ✗ 为外部 Trait 实现外部类型                        │
│    例：impl Display for Vec<String>  // ❌          │
│    （Display 和 Vec 都是 std 的）                    │
│                                                     │
│  解决方案：                                          │
│  • 使用新类型模式（Newtype Pattern）                │
│  • 包装外部类型                                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 新类型模式

```rust
use std::fmt::Display;

// ❌ 错误：违反孤儿规则
// impl Display for Vec<String> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // ...
//     }
// }

// ✅ 正确：使用新类型模式
struct MyVec(Vec<String>);

impl Display for MyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let my_vec = MyVec(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("{}", my_vec);  // [hello, world]
}
```







