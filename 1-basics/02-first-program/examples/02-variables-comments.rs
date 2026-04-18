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

    // ✅ 正确示例：文档注释（///）只能用于顶层条目（函数、结构体等），不能用于局部变量
    // // 普通注释：可以写在任何地方
    // /// 文档注释：只对 fn / struct / enum 等顶层定义有效
    let author = "Rust Community";
    println!("author: {}", author);

    // 尝试修改：
    // 1. 添加更多变量和注释
    // 2. 在 fn main() 上方（函数定义前）添加 /// 文档注释，观察效果
    // 3. 思考：// 注释和 /// 文档注释的适用场景有何不同？
}
