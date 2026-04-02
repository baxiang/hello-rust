## 16.1 为什么需要 Trait？

### 代码组织问题

```rust
// 问题：不同类型的相似操作

struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    fn get_summary(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
}

impl Tweet {
    fn get_summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 问题：无法编写通用函数处理两者
// fn print_summary(item: ???) {
//     println!("{}", item.get_summary());
// }
```

### Trait 的解决方案

```rust
// 使用 Trait 定义共享行为
trait Summary {
    fn get_summary(&self) -> String;
}

impl Summary for NewsArticle {
    fn get_summary(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn get_summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 现在可以编写通用函数
fn print_summary(item: &impl Summary) {
    println!("{}", item.get_summary());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        author: String::from("John"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Hello world!"),
    };

    print_summary(&article);  // Breaking News, by John
    print_summary(&tweet);    // @rustlang: Hello world!
}
```

### Trait 的核心作用

```
┌─────────────────────────────────────────────────────┐
│              Trait 的核心作用                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. 抽象行为（接口）                                 │
│     • 定义一组方法的签名                            │
│     • 不关心具体实现                                │
│     • 类似其他语言的 interface                      │
│                                                     │
│  2. 泛型约束                                        │
│     • 限制泛型参数的可用方法                        │
│     • 编译时类型检查                                │
│     • 零运行时开销                                  │
│                                                     │
│  3. 代码复用                                        │
│     • 默认实现减少重复代码                         │
│     • 为现有类型添加行为（扩展）                   │
│     • 组合优于继承                                  │
│                                                     │
│  4. 多态支持                                        │
│     • Trait 对象（动态分发）                        │
│     • 运行时可以选择不同实现                        │
│     • 灵活但有小开销                                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

---

## 下一步

[定义 Trait](../2-定义 Trait.md)