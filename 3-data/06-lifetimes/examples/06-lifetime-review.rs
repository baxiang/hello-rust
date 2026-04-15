//! # 示例：生命周期实战总结
//!
//! 对应章节：06-实战总结
//! 运行：cargo run --example 06-lifetime-review

fn main() {
    println!("=== 生命周期实战总结 ===\n");

    // 1. 基本生命周期
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or(s)
    }
    let s = String::from("hello world");
    println!("第一个单词：{}", first_word(&s));

    // 2. 结构体生命周期
    struct Parser<'a> {
        data: &'a str,
    }
    let text = "parse this";
    let parser = Parser { data: text };
    println!("解析数据：{}", parser.data);

    // 3. 省略规则总结
    println!("\n省略规则：");
    println!("  1. 每个引用参数获得独立生命周期");
    println!("  2. 单输入引用 → 返回该生命周期");
    println!("  3. &self 方法 → 返回 self 生命周期");

    // 4. 'static
    const CONST_STR: &str = "静态字符串";
    println!("\n'static: {}", CONST_STR);

    // 5. 常见场景
    println!("\n常见场景：");
    println!("  - 结构体持有引用");
    println!("  - 函数返回引用");
    println!("  - Trait 对象");
    println!("  - 闭包捕获引用");

    println!("\n生命周期章节完成！");
}
