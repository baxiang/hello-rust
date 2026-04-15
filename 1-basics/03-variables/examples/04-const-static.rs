//! # 示例：常量与静态变量
//!
//! 对应章节：02-常量与静态
//! 运行：cargo run --example 04-const-static

// ✅ 正确示例：const 常量
const MAX_POINTS: u32 = 100_000;

// ✅ 正确示例：static 静态变量
static GREETING: &str = "Hello, Rust!";

fn main() {
    // ✅ 使用 const 常量
    println!("MAX_POINTS = {}", MAX_POINTS);

    // ✅ 使用 static 静态变量
    println!("{}", GREETING);

    // const vs let 对比
    let _max_points: u32 = 100_000; // 运行时赋值
                                    // const 在编译时求值，let 在运行时求值

    // 尝试修改：
    // 1. MAX_POINTS = 200_000;  // ❌ 常量不能修改
    // 2. 思考：const 和 let 什么时候用哪个？

    // 最佳实践：
    // - 编译时常数用 const
    // - 运行时的值用 let
    // - 跨模块共享的常量用 const
}
