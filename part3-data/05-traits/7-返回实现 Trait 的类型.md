## 16.7 返回实现 Trait 的类型

### impl Trait 返回类型

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

// 返回实现 Summary 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
    }
}

fn main() {
    let item = returns_summarizable();
    println!("{}", item.summarize());
}
```

### 限制：只能返回单一类型

```rust
// ❌ 错误：返回了不同类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Tweet { /* ... */ }
//     } else {
//         NewsArticle { /* ... */ }
//     }
// }

// ✅ 正确：使用 Trait 对象（Box）
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course"),
        })
    } else {
        Box::new(NewsArticle {
            headline: String::from("Breaking"),
            location: String::from("NYC"),
            author: String::from("John"),
            content: String::from("..."),
        })
    }
}
```

### impl Trait vs Box<dyn Trait>

```
┌─────────────────────────────────────────────────────┐
│      impl Trait vs Box<dyn Trait>                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  impl Trait（静态分发）                             │
│  ├── 编译时确定类型                                 │
│  ├── 零运行时开销                                   │
│  ├── 只能返回单一类型                             │
│  └── 更高效的调用                                   │
│                                                     │
│  Box<dyn Trait>（动态分发）                         │
│  ├── 运行时确定类型                                 │
│  ├── 有小开销（虚函数表查找）                      │
│  ├── 可以返回不同类型                             │
│  └── 更灵活                                         │
│                                                     │
│  选择：                                              │
│  • 需要返回不同类型 → Box<dyn Trait>               │
│  • 追求性能 → impl Trait                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

---

## 下一步

[条件实现](../8-条件实现.md)