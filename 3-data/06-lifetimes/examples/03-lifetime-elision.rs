//! # 示例：生命周期省略规则
//!
//! 对应章节：03-省略规则
//! 运行：cargo run --example 03-lifetime-elision

// ✅ 正确示例：省略规则 1 - 每个引用参数获得独立生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

// ✅ 正确示例：省略规则 2 - 如果只有一个输入引用，返回该生命周期
fn longest_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// ✅ 正确示例：省略规则 3 - &self 的方法
struct Text {
    content: String,
}

impl Text {
    fn first_line(&self) -> &str {
        self.content.lines().next().unwrap_or("")
    }
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词：{}", word);

    let longer = longest_str("short", "longer string");
    println!("更长：{}", longer);

    let text = Text {
        content: String::from("第一行\n第二行\n第三行"),
    };
    println!("第一行：{}", text.first_line());

    // 尝试修改：
    // 1. 思考：什么时候必须标注生命周期？
    // 2. 尝试违反省略规则
}
