//! # 示例：闭包与递归
//!
//! 对应章节：05-闭包与递归
//! 运行：cargo run --example 05-closures-recursion

// ✅ 正确示例：闭包
fn main() {
    // 基本闭包
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 带类型标注的闭包
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("add(3, 4) = {}", add(3, 4));

    // 闭包捕获环境
    let x = 5;
    let print_x = || println!("x = {}", x);
    print_x();

    // ✅ 正确示例：递归函数
    fn factorial(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    println!("\n阶乘：");
    for i in 1..=6 {
        println!("{}! = {}", i, factorial(i));
    }

    // ✅ 正确示例：斐波那契数列
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    println!("\n斐波那契：");
    for i in 0..=10 {
        print!("{} ", fibonacci(i));
    }
    println!();

    // 尝试修改：
    // 1. 创建一个闭包捕获可变变量
    // 2. 优化 fibonacci 使用记忆化
    // 3. 思考：闭包和函数的区别？
}
