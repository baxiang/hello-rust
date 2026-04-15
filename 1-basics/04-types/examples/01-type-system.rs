//! # 示例：类型系统概述
//!
//! 对应章节：01-类型系统概述
//! 运行：cargo run --example 01-type-system

fn main() {
    // ✅ 正确示例：Rust 是静态类型语言
    // 类型在编译时确定

    // 类型推断：编译器自动推断类型
    let _inferred_i32 = 42; // i32
    let _inferred_f64 = std::f64::consts::PI; // f64
    let _inferred_str = "hello"; // &str
    let _inferred_bool = true; // bool

    println!("类型推断示例：");
    println!("42     → i32");
    println!("3.14   → f64");
    println!("\"hello\" → &str");
    println!("true   → bool");

    // 尝试修改：
    // 1. 显式标注每个变量的类型
    // 2. 思考：静态类型 vs 动态类型的区别？
    // 3. 思考：类型推断的优势是什么？

    println!("\nRust 的类型系统在编译时保证内存安全！");
}
