//! # 示例：Unsafe 概述
//!
//! 对应章节：01-Unsafe概述
//! 运行：cargo run --example 01-unsafe-overview

fn main() {
    println!("=== Unsafe Rust 概述 ===\n");

    // ✅ 正确示例：解引用裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        *r2 = 10;
        println!("r2: {}", *r1);
    }

    // ✅ 正确示例：调用 unsafe 函数
    unsafe fn dangerous() {
        println!("这是 unsafe 函数");
    }

    unsafe {
        dangerous();
    }

    println!("\nUnsafe 用于：");
    println!("  - 解引用裸指针");
    println!("  - 调用 unsafe 函数");
    println!("  - 访问或修改可变静态变量");
    println!("  - 实现 unsafe trait");

    // 尝试修改：
    // 1. 思考：什么时候需要 unsafe？
    // 2. 为什么 Rust 要有 unsafe？
}
