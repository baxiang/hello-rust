//! # 示例：变量与注释
//!
//! 对应章节：02-变量与注释
//! 运行：cargo run --example 02-variables-comments

fn main() {
    // ✅ 正确示例：变量声明
    let message = "Hello from Rust!";
    let count = 42;

    // ✅ 正确示例：单行注释
    println!("message: {}", message);
    println!("count: {}", count);

    // ✅ 正确示例：多行注释
    // 这是多行注释
    // 可以写很多行
    let year = 2026;
    println!("year: {}", year);

    // ✅ 正确示例：文档注释（///）
    /// 这是一个文档注释
    /// 用于生成 API 文档
    /// 这是 author 变量
    let author = "Rust Community";
    println!("author: {}", author);

    // 尝试修改：
    // 1. 添加更多变量
    // 2. 使用 /// 为你的变量添加文档注释
    // 3. 思考：注释和文档注释的区别是什么？
}
