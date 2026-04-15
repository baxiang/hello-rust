//! # 示例：函数基础
//!
//! 对应章节：01-函数基础
//! 运行：cargo run --example 01-function-basics

// ✅ 正确示例：基本函数定义
fn greet() {
    println!("Hello from a function!");
}

// ✅ 正确示例：带参数函数
fn greet_name(name: &str) {
    println!("Hello, {}!", name);
}

// ✅ 正确示例：带返回值函数
fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式返回值（无分号）
}

// ❌ 错误示例：语句返回值
fn bad_add(a: i32, b: i32) -> i32 {
    // a + b;  // ❌ 加分号变成语句，返回 ()
    a + b // ✅ 直接返回
}

fn main() {
    // 调用函数
    greet();
    greet_name("Rustacean");

    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    let bad_sum = bad_add(3, 5);
    println!("bad_add: {}", bad_sum);

    // 尝试修改：
    // 1. 创建你自己的函数
    // 2. 尝试表达式 vs 语句返回的区别
    // 3. 思考：return 关键字什么时候必须使用？
}
