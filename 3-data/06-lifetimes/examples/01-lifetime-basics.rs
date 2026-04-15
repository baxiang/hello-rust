//! # 示例：生命周期基础
//!
//! 对应章节：01-生命周期基础
//! 运行：cargo run --example 01-lifetime-basics

// ✅ 正确示例：生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("最长的字符串是：{}", result);

    // ✅ 正确示例：生命周期与作用域
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("result: {}", result);
    }

    // 尝试修改：
    // 1. 创建自己的生命周期函数
    // 2. 思考：为什么需要生命周期标注？
    // 3. 尝试省略生命周期（观察编译器推断）
}
