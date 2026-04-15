//! # 示例：默认实现
//!
//! 对应章节：02-默认实现
//! 运行：cargo run --example 02-default-impl

// ✅ 正确示例：Trait 默认实现
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(作者: {})", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        self.username.clone()
    }

    // 覆盖默认实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("新闻标题"),
        author: String::from("记者"),
    };

    let tweet = Tweet {
        username: String::from("用户"),
        content: String::from("内容"),
    };

    println!("文章（默认）：{}", article.summarize());
    println!("推文（覆盖）：{}", tweet.summarize());

    // 尝试修改：
    // 1. 添加更多默认方法
    // 2. 思考：什么时候使用默认实现？
}
