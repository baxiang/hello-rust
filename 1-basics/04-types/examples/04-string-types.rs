//! # 示例：字符串类型
//!
//! 对应章节：04-字符串类型
//! 运行：cargo run --example 04-string-types

fn main() {
    // ✅ 正确示例：&str (字符串切片)
    let greeting: &str = "Hello, Rust!";
    println!("&str: {}", greeting);

    // ✅ 正确示例：String (堆分配字符串)
    let mut name = String::from("Rustacean");
    println!("String: {}", name);

    // 修改 String
    name.push_str(" is great!");
    println!("修改后: {}", name);

    // ✅ 正确示例：String 操作
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let combined = format!("{}{}", s1, s2);
    println!("合并: {}", combined);

    // ✅ 正确示例：字符串方法
    let text = "  Hello, World!  ";
    println!("原始: '{}'", text);
    println!("trim: '{}'", text.trim());
    println!("to_uppercase: '{}'", text.trim().to_uppercase());
    println!("长度: {}", text.len());

    // 尝试修改：
    // 1. 创建一个 String 并添加内容
    // 2. 尝试修改 &str (会失败，思考为什么)
    // 3. 思考：&str 和 String 的区别？
}
