//! # 示例：切片基础
//!
//! 对应章节：01-切片基础
//! 运行：cargo run --example 01-slice-basics

fn main() {
    // ✅ 正确示例：字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: '{}', world: '{}'", hello, world);

    // ✅ 正确示例：简化语法
    let hello = &s[..5]; // 从开头到 5
    let world = &s[6..]; // 从 6 到结尾
    let all = &s[..]; // 整个字符串
    println!("简化：'{}' '{}' '{}'", hello, world, all);

    // ✅ 正确示例：数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片：{:?}", slice);

    // ✅ 正确示例：切片类型
    let s: &str = "string slice";
    let a: &[i32] = &[1, 2, 3];
    println!("&str: {}", s);
    println!("&[i32]: {:?}", a);

    // 尝试修改：
    // 1. 创建自己的字符串切片
    // 2. 尝试越界切片（会 panic）
    // 3. 思考：切片和引用的区别？
}
