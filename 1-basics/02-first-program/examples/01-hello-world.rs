//! # 示例：Hello World 详解
//!
//! 对应章节：01-HelloWorld详解
//! 运行：cargo run --example 01-hello-world

fn main() {
    // ✅ 正确示例：最简单的 Hello World
    println!("Hello, World!");

    // ✅ 正确示例：带格式化输出
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // ✅ 正确示例：多参数输出
    let rust_version = "1.85";
    let edition = "2024";
    println!("Rust {} with {} Edition", rust_version, edition);

    // ✅ 正确示例：不同格式化宏
    println!("println! - 带换行");
    print!("print! - 不带换行");
    println!(); // 手动换行

    // 尝试修改：
    // 1. 添加更多 println! 输出你的信息
    // 2. 尝试使用 print! 和 println! 观察区别
    // 3. 思考：println! 和 print! 的底层实现是什么？
}
