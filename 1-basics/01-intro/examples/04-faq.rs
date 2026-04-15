//! # 示例：常见问题排查
//!
//! 对应章节：04-常见问题
//! 运行：cargo run --example 04-faq

fn main() {
    // ✅ 正确示例：常见编译错误演示
    println!("常见问题与解决方案：\n");

    // 问题 1：未安装 rustc
    println!("问题 1：command not found: rustc");
    println!("解决：curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n");

    // 问题 2：Cargo.toml 语法错误
    println!("问题 2：Cargo.toml 解析错误");
    println!("解决：检查 toml 语法，使用 even-better-toml 插件\n");

    // 问题 3：依赖版本冲突
    println!("问题 3：依赖版本冲突");
    println!("解决：使用 cargo update 更新依赖树\n");

    // 问题 4：编译速度慢
    println!("问题 4：编译速度慢");
    println!("解决：使用 cargo check 代替 cargo build 快速验证\n");

    // 尝试修改：
    // 1. 运行 cargo --version 确认工具链安装
    // 2. 思考：你遇到过哪些 Rust 编译错误？如何解决的？

    println!("记住：编译器是你的朋友，错误信息是学习线索！");
}
