//! # 示例：为什么学习 Rust
//!
//! 对应章节：01-为什么学习Rust
//! 运行：cargo run --example 01-why-rust

fn main() {
    // Rust 的核心优势：零成本抽象、内存安全、并发安全

    // ✅ 正确示例：类型安全
    let x: i32 = 42;
    let y: f64 = std::f64::consts::PI;
    println!("x = {}, y = {}", x, y);

    // ✅ 正确示例：编译时检查
    // let z = "hello";
    // let w: i32 = z;  // ❌ 编译错误：类型不匹配

    // 尝试修改：
    // 1. 将 let y: f64 = 3.14 改为 let y: i32 = 3.14，观察编译错误
    // 2. 思考：Rust 的编译时检查如何帮助你避免运行时错误？

    println!("Rust 通过编译器检查，将错误消灭在编码阶段！");
}
