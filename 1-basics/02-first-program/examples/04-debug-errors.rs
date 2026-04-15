//! # 示例：调试与错误处理
//!
//! 对应章节：04-调试与错误
//! 运行：cargo run --example 04-debug-errors

fn main() {
    // ✅ 正确示例：使用 dbg! 宏调试
    let x = 5;
    dbg!(x);

    // ✅ 正确示例：使用 println! 调试
    let y = 10;
    println!("调试：y = {}", y);

    // ✅ 正确示例：常见编译错误演示
    // 取消下面的注释观察错误：
    // let z;
    // println!("z = {}", z);  // ❌ E0381: 使用未初始化变量

    // ✅ 正确示例：类型推断
    let inferred = 42; // 推断为 i32
    let explicit: f64 = std::f64::consts::PI;
    println!("inferred: {} (类型: i32)", inferred);
    println!("explicit: {} (类型: f64)", explicit);

    // 尝试修改：
    // 1. 取消注释观察编译错误
    // 2. 使用 dbg! 打印变量值
    // 3. 思考：dbg! 和 println! 的区别？

    println!("\n编译器错误是最好的老师！");
}
