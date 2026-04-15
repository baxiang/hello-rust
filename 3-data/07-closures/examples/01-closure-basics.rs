//! # 示例：闭包基础
//!
//! 对应章节：01-闭包基础
//! 运行：cargo run --example 01-closure-basics

fn main() {
    // ✅ 正确示例：基本闭包
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // ✅ 正确示例：闭包捕获环境
    let x = 4;
    let add_x = |y| x + y;
    println!("add_x(3) = {}", add_x(3));

    // ✅ 正确示例：闭包类型
    let square = |x: i32| -> i32 { x * x };
    println!("square(4) = {}", square(4));

    // ✅ 正确示例：闭包作为参数
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let result = apply(|n| n * 2, 5);
    println!("apply(|n| n * 2, 5) = {}", result);

    // ✅ 正确示例：move 闭包
    let data = vec![1, 2, 3];
    let consume = move || {
        println!("data: {:?}", data);
    };
    consume();
    // data 不能再使用
    // println!("{:?}", data);  // ❌

    // 尝试修改：
    // 1. 创建自己的闭包
    // 2. 尝试捕获可变变量
    // 3. 思考：闭包和函数的区别？
}
