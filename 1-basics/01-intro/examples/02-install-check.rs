//! # 示例：安装与配置验证
//!
//! 对应章节：02-安装与配置
//! 运行：cargo run --example 02-install-check

fn main() {
    // 验证 Rust 环境安装

    // ✅ 正确示例：打印版本信息
    println!("Rust 环境验证：");
    println!("✓ rustc - Rust 编译器");
    println!("✓ cargo - Rust 包管理器");
    println!("✓ rustup - Rust 工具链管理器");

    // ✅ 正确示例：基本类型验证
    let version: &str = "1.85+";
    let edition: &str = "2024";
    println!("✓ 最低版本：{}", version);
    println!("✓ Edition：{}", edition);

    // 尝试修改：
    // 1. 运行 cargo --version 和 rustc --version 查看实际版本
    // 2. 思考：为什么使用 rustup 而不是直接安装 rustc？

    println!("\n环境配置完成！开始 Rust 之旅。");
}
