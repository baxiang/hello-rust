//! # 示例：Trait 基础
//!
//! 对应章节：01-Trait基础
//! 运行：cargo run --example 01-trait-basics

// ✅ 正确示例：定义 Trait
trait Summary {
    fn summarize(&self) -> String;
}

// ✅ 正确示例：实现 Trait
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust 入门"),
        author: String::from("张三"),
        content: String::from("本文介绍 Rust 基础..."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("今天学了 Trait！"),
    };

    println!("文章摘要：{}", article.summarize());
    println!("推文摘要：{}", tweet.summarize());

    // 尝试修改：
    // 1. 创建自己的 Trait 和实现
    // 2. 思考：Trait 和接口的区别？
    // 3. 尝试使用 Trait 作为参数
}
