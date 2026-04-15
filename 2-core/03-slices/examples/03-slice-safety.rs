//! # 示例：切片方法与安全
//!
//! 对应章节：03-切片方法与安全
//! 运行：cargo run --example 03-slice-safety

fn main() {
    // ✅ 正确示例：字符串切片方法
    let s = String::from("hello, world!");

    println!("长度：{}", s.len());
    println!("是否为空：{}", s.is_empty());
    println!("包含 'world'：{}", s.contains("world"));
    println!("以 'hello' 开头：{}", s.starts_with("hello"));

    // ✅ 正确示例：切片迭代
    let text = "hello";
    for c in text.chars() {
        print!("{} ", c);
    }
    println!();

    // ✅ 正确示例：数组切片方法
    let a = [1, 2, 3, 4, 5];
    println!("第一个：{:?}", a.first());
    println!("最后一个：{:?}", a.last());
    println!("中间：{:?}", a.get(1..4));

    // ✅ 正确示例：切片安全
    // 安全的索引访问
    if let Some(val) = a.get(10) {
        println!("a[10] = {}", val);
    } else {
        println!("索引 10 越界");
    }

    // 尝试修改：
    // 1. 使用 chars() 遍历中文字符串
    // 2. 尝试在字节边界切片中文字符串（会 panic）
    // 3. 思考：为什么 &str 是 UTF-8 安全的？

    // 中文示例
    let chinese = "你好世界";
    println!("\n中文字符：");
    for c in chinese.chars() {
        println!("{}", c);
    }
}
