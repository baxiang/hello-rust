//! # 示例：Rust 2024 Edition
//!
//! 对应章节：02-新语言特性
//! 运行：cargo run --example 02-new-features

fn main() {
    println!("=== Rust 2024 Edition ===\n");

    // ✅ 正确示例：2024 Edition 特性
    println!("当前 Edition: 2024");
    println!("Rust 版本：1.85+");

    // 2024 Edition 主要特性：
    println!("\n2024 Edition 主要特性：");
    println!("  - RPITIT 改进");
    println!("  - 安全静态互操作");
    println!("  - 更严格的借用检查");
    println!("  - 新 lint 规则");
    println!("  - 标准库改进");

    // ✅ 正确示例：现代 Rust 写法
    let numbers = [1, 2, 3, 4, 5];
    let sum: i32 = numbers.into_iter().sum();
    println!("\n现代写法：{}", sum);

    // 尝试修改：
    // 1. 探索 2024 Edition 新特性
    // 2. 比较 2021 和 2024 的区别
}
