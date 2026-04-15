//! # 示例：所有权基础
//!
//! 对应章节：01-所有权基础
//! 运行：cargo run --example 01-ownership-basics

fn main() {
    // ✅ 正确示例：所有权三定律
    // 1. 每个值都有一个所有者
    // 2. 同一时间只能有一个所有者
    // 3. 所有者离开作用域时值被 drop

    // String 类型的所有权
    let s1 = String::from("hello");
    println!("s1: {}", s1);

    // 所有权转移（Move）
    let s2 = s1;
    println!("s2: {}", s2);
    // s1 不能再使用
    // println!("s1: {}", s1);  // ❌ E0382: 使用已移动的值

    // ✅ 正确示例：Clone 保留所有权
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // ✅ 正确示例：Copy 类型
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y); // x 仍然可用

    // 尝试修改：
    // 1. 取消 s1 的注释观察编译错误
    // 2. 思考：为什么 String 需要 Move 而 i32 不需要？
    // 3. 尝试使用 .clone() 保留所有权

    println!("\n所有权是 Rust 最独特的特性！");
}
