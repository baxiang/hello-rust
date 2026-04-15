//! # 示例：可变引用与借用规则
//!
//! 对应章节：02-可变引用与借用规则
//! 运行：cargo run --example 02-mutable-references

fn main() {
    // ✅ 正确示例：可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后：{}", s);

    // ✅ 正确示例：借用规则
    // 规则 1：同一时间只能有一个可变引用
    // 规则 2：可变引用和不可变引用不能同时存在

    let mut data = String::from("data");

    let r1 = &data; // 不可变引用
    println!("r1: {}", r1);
    // r1 作用域结束

    let r2 = &mut data; // 可变引用
    r2.push_str(" modified");
    println!("r2: {}", r2);

    // ✅ 正确示例：多个不可变引用
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // 尝试修改：
    // 1. 尝试创建两个可变引用（会失败）
    // 2. 思考：为什么 Rust 有这些借用规则？
    // 3. 尝试在不可变引用存在时创建可变引用
}

fn change(s: &mut String) {
    s.push_str(", world");
}
