//! # 示例：WASM 基础
//!
//! 对应章节：01-WASM基础
//! 运行：cargo run --example 01-wasm-basics

fn main() {
    println!("=== WebAssembly 基础 ===\n");

    println!("WebAssembly 特点：");
    println!("  - 接近原生性能");
    println!("  - 跨平台运行");
    println!("  - 与 JavaScript 互操作");
    println!("  - 安全沙箱环境");

    // ✅ 正确示例：简单计算
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    let result = fibonacci(10);
    println!("\nfibonacci(10) = {}", result);

    println!("\nWASM 编译命令：");
    println!("  cargo build --target wasm32-unknown-unknown");

    // 尝试修改：
    // 1. 探索 wasm-pack 工具
    // 2. 思考：WASM 适合什么场景？
}
