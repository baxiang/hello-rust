//! # 示例：引用进阶
//!
//! 对应章节：03-引用进阶
//! 运行：cargo run --example 03-reference-advanced

fn main() {
    // ✅ 正确示例：悬垂引用（编译错误）
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // ❌ s 离开作用域被 drop
    // }
    // 修复：返回 String
    let s = no_dangle();
    println!("no_dangle: {}", s);

    // ✅ 正确示例：引用的作用域
    let mut s = String::from("hello");
    {
        let r1 = &s;
        println!("r1: {}", r1);
    } // r1 作用域结束

    let r2 = &mut s; // ✅ 可以创建可变引用
    r2.push_str(" world");
    println!("r2: {}", r2);

    // ✅ 正确示例：NLL（非词法生命周期）
    let mut s = String::from("hello");
    let r1 = &s;
    println!("r1: {}", r1);
    // r1 最后使用在这里
    let r2 = &mut s; // ✅ NLL 允许
    r2.push('!');
    println!("r2: {}", r2);

    // 尝试修改：
    // 1. 创建一个返回引用的函数
    // 2. 尝试理解 NLL 如何改善代码
    // 3. 思考：悬垂引用为什么危险？
}

fn no_dangle() -> String {
    String::from("hello")
}
