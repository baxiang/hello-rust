//! # 示例：Trait 作为参数
//!
//! 对应章节：03-Trait作为参数
//! 运行：cargo run --example 03-trait-as-param

use std::fmt::Display;

// ✅ 正确示例：Trait 作为参数
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// 函数参数使用 Trait
fn notify(item: &impl Summary) {
    println!("通知：{}", item.summarize());
}

// ✅ 正确示例：Trait Bound 语法
fn notify_bound<T: Summary>(item: &T) {
    println!("通知：{}", item.summarize());
}

// ✅ 正确示例：多 Trait Bound
fn display_summary(item: &(impl Summary + Display)) {
    println!("显示：{}", item);
    println!("摘要：{}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Rust"),
        author: String::from("张三"),
    };

    notify(&article);
    notify_bound(&article);

    // 返回 Trait
    fn returns_summarizable() -> impl Summary {
        Article {
            title: String::from("新文章"),
            author: String::from("李四"),
        }
    }

    let item = returns_summarizable();
    println!("返回：{}", item.summarize());

    // 尝试修改：
    // 1. 创建自己的 Trait 参数函数
    // 2. 思考：impl Trait vs 泛型？
}
