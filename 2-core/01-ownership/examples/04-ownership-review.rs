//! # 示例：所有权实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-ownership-review

fn main() {
    // ✅ 正确示例：综合所有权

    // 1. 所有权转移
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
    // s1 不可用

    // 2. Clone 保留
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // 3. Copy 类型
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // 4. 函数与所有权
    let s = String::from("test");
    let len = calculate_length(&s);
    println!("'{}' 的长度是 {}", s, len);

    // 5. 返回值所有权
    let s1 = gives_ownership();
    println!("返回值：{}", s1);

    // 尝试修改：
    // 1. 创建一个使用所有权的函数
    // 2. 尝试混合使用 Clone 和引用
    // 3. 思考：所有权系统如何保证内存安全？

    println!("\n所有权章节完成！");
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn gives_ownership() -> String {
    String::from("hello")
}
