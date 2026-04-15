//! # 示例：借用检查器
//!
//! 对应章节：05-借用检查器
//! 运行：cargo run --example 05-borrow-checker

fn main() {
    println!("=== 借用检查器规则 ===\n");

    // ✅ 规则 1：每个值有且只有一个所有者
    let _s = String::from("owner");
    println!("s 的所有者：main 函数");

    // ✅ 规则 2：同一时间只能有一个可变借用
    let mut data = vec![1, 2, 3];
    let r1 = &mut data;
    r1.push(4);
    // r1 使用结束
    let r2 = &mut data;
    r2.push(5);
    println!("data: {:?}", data);

    // ✅ 规则 3：不可变借用和可变借用不能共存
    let text = String::from("hello");
    let immutable = &text;
    println!("不可变借用：{}", immutable);
    // immutable 使用结束
    let mutable = &mut text.clone();
    mutable.push_str(" world");
    println!("可变借用：{}", mutable);

    // ✅ 规则 4：借用必须始终有效
    let s = String::from("outer");
    let result = &s;
    println!("result: {}", result);

    println!("\n借用检查器保证内存安全！");
}
