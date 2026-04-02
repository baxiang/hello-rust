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

---

---

## 下一步

[默认实现](../4-默认实现.md)