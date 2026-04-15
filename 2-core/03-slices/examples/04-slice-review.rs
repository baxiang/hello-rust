//! # 示例：切片实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-slice-review

fn main() {
    println!("=== 切片实战总结 ===\n");

    // 1. 字符串切片
    let s = String::from("hello world");
    let hello = &s[..5];
    println!("字符串切片：{}", hello);

    // 2. 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片：{:?}", slice);

    // 3. 切片作为参数
    fn process(s: &str) {
        println!("处理：{}", s);
    }
    process(&s);
    process("直接传递");

    // 4. 切片方法
    println!("\n切片方法：");
    println!("长度：{}", s.len());
    println!("包含：{}", s.contains("world"));
    println!("分割：{:?}", s.split(' ').collect::<Vec<_>>());

    // 5. 综合示例：处理文本
    let text = "Rust is a systems programming language";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("\n单词数：{}", words.len());
    println!("第一个单词：{}", words[0]);
    println!("最后一个单词：{}", words.last().unwrap());

    // 尝试修改：
    // 1. 创建一个使用切片的函数
    // 2. 尝试处理中文字符串
    // 3. 思考：切片为什么比索引更安全？

    println!("\n切片章节完成！");
}
