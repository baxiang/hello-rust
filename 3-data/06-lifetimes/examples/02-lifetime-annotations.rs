//! # 示例：生命周期标注
//!
//! 对应章节：02-生命周期标注
//! 运行：cargo run --example 02-lifetime-annotations

// ✅ 正确示例：结构体生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意：{}", announcement);
        self.part
    }
}

// ✅ 正确示例：多个生命周期
fn first_word<'a, 'b>(s: &'a str, _: &'b str) -> &'a str {
    s.split_whitespace().next().unwrap_or(s)
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt { part: &novel[..4] };
    println!("摘录：{}", excerpt.part);
    println!("级别：{}", excerpt.level());

    let result = excerpt.announce_and_return_part("这是摘录");
    println!("返回：{}", result);

    // 尝试修改：
    // 1. 创建带生命周期的结构体
    // 2. 思考：生命周期省略规则？
}
