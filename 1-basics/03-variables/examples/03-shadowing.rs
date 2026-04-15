//! # 示例：变量遮蔽（Shadowing）
//!
//! 对应章节：03-高级特性
//! 运行：cargo run --example 03-shadowing

fn main() {
    // ✅ 正确示例：变量遮蔽基础
    let x = 5;
    println!("x = {} (第一次)", x);

    let x = 10; // 遮蔽之前的 x
    println!("x = {} (第二次)", x);

    // ✅ 正确示例：遮蔽时改变类型
    let spaces = "   ";
    let spaces = spaces.len(); // 从 &str 变为 usize
    println!("spaces = {}", spaces);

    // ✅ 正确示例：遮蔽 vs 可变变量
    // 遮蔽：可以改变类型，每次声明是新变量
    let _y = 5;
    let _y = 10; // 可变：类型不能改变

    // 尝试修改：
    // 1. 将 let spaces = spaces.len() 改为 y = spaces.len()，观察编译错误
    // 2. 思考：什么时候用遮蔽，什么时候用 mut？
}
