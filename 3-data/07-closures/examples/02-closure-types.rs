//! # 示例：类型与捕获
//!
//! 对应章节：02-类型与捕获
//! 运行：cargo run --example 02-closure-types

fn main() {
    // ✅ Fn: 不可变借用
    let s = String::from("hello");
    let print_s = || println!("{}", s);
    print_s();
    print_s(); // 可以多次调用
    println!("s 仍然可用：{}", s);

    // ✅ FnMut: 可变借用
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("increment: {}", increment());
    println!("increment: {}", increment());
    println!("count: {}", count);

    // ✅ FnOnce: 获取所有权
    let greeting = String::from("hello");
    let consume = || {
        println!("消费：{}", greeting);
    };
    consume();
    // consume();  // ❌ 只能调用一次

    // ✅ 闭包作为返回值
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }
    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(3) = {}", add5(3));
    println!("add10(3) = {}", add10(3));

    // 尝试修改：
    // 1. 创建不同类型的闭包
    // 2. 思考：编译器如何选择 Fn/FnMut/FnOnce？
}
