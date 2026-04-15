//! # 示例：Cargo 基础
//!
//! 对应章节：01-Cargo基础
//! 运行：cargo run --example 01-cargo-basics

fn main() {
    println!("=== Cargo 基础 ===\n");
    println!("常用命令：");
    println!("  cargo new <name>    - 创建项目");
    println!("  cargo build         - 编译");
    println!("  cargo run           - 运行");
    println!("  cargo check         - 快速检查");
    println!("  cargo test          - 测试");
    println!("  cargo clippy        - lint");
    println!("  cargo fmt           - 格式化");
    println!("  cargo doc --open    - 文档");
    println!("\n当前项目：{}", env!("CARGO_PKG_NAME"));
}
