//! # 示例：引用基础
//!
//! 对应章节：01-引用基础
//! 运行：cargo run --example 01-reference-basics

fn main() {
    // ✅ 正确示例：引用（借用）
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 创建引用
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然可用

    // ✅ 正确示例：引用不获取所有权
    let s2 = String::from("world");
    print_string(&s2);
    println!("s2 仍然可用：{}", s2);

    // ✅ 正确示例：引用的引用
    let x = 5;
    let r = &x;
    let rr = &r;
    println!("x={}, r={}, rr={}", x, r, rr);

    // 尝试修改：
    // 1. 创建一个使用引用的函数
    // 2. 思考：引用和所有权的区别？
    // 3. 尝试修改引用指向的值（会失败）
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn print_string(s: &String) {
    println!("借用：{}", s);
}
