//! # 示例：可变变量
//!
//! 对应章节：01-变量基础
//! 运行：cargo run --example 02-mut-variable

fn main() {
    // ✅ 正确示例：使用 mut 声明可变变量
    let mut x = 5;
    println!("x = {}", x);

    x = 10;
    println!("x = {} (修改后)", x);

    // ✅ 正确示例：对比不可变和可变
    let a = 5; // 不可变
    let mut b = 5; // 可变
    println!("b (初始值) = {}", b);

    // a = 10;      // ❌ E0384: a 是不可变的
    b = 10; // ✅ 正确：b 是可变的
    println!("b (修改后) = {}", b);

    println!("a = {}, b = {}", a, b);

    // 尝试修改：
    // 1. 去掉 a = 10 的注释，观察编译错误
    // 2. 将 let mut b = 5 改为 let b = 5，观察编译错误
}
