//! # 示例：引用实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-reference-review

fn main() {
    println!("=== 引用实战总结 ===\n");

    // 1. 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' 长度：{}", s1, len);

    // 2. 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后：{}", s2);

    // 3. 多个引用
    let s3 = String::from("data");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);

    // 4. 借用规则总结
    println!("\n借用规则：");
    println!("1. 同一时间只能有一个可变引用");
    println!("2. 可以有多个不可变引用");
    println!("3. 引用必须始终有效");

    // 尝试修改：
    // 1. 创建一个使用多种引用的函数
    // 2. 尝试违反借用规则观察错误
    // 3. 思考：引用如何避免数据竞争？

    println!("\n引用章节完成！");
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
