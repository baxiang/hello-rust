//! # 示例：第一个 Cargo 项目
//!
//! 对应章节：03-快速上手
//! 运行：cargo run --example 03-first-cargo-project

fn main() {
    // ✅ 正确示例：Cargo 项目结构
    println!("Cargo 项目结构：");
    println!("Cargo.toml    ← 项目配置和依赖");
    println!("src/");
    println!("  main.rs     ← 入口文件");

    // ✅ 正确示例：运行 Cargo 命令
    println!("\n常用 Cargo 命令：");
    println!("cargo new my_project    ← 创建新项目");
    println!("cargo run               ← 编译并运行");
    println!("cargo build             ← 编译项目");
    println!("cargo check             ← 快速检查编译");
    println!("cargo clippy            ← lint 检查");

    // ✅ 正确示例：变量与输出
    let project_name = "my_first_rust_project";
    println!("\n项目名称：{}", project_name);

    // 尝试修改：
    // 1. 在终端运行 cargo new test_project 创建新项目
    // 2. 思考：Cargo 和 rustc 的区别是什么？
}
