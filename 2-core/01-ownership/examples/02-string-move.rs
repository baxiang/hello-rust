//! # 示例：String 与移动语义
//!
//! 对应章节：02-String与移动
//! 运行：cargo run --example 02-string-move

fn main() {
    // ✅ 正确示例：String 的内存布局
    // String 在栈上存储：指针、长度、容量
    // 实际数据在堆上
    let s = String::from("hello");
    println!("s: {}", s);
    println!("长度: {}", s.len());
    println!("容量: {}", s.capacity());

    // ✅ 正确示例：移动语义
    let s1 = String::from("hello");
    let s2 = s1; // s1 移动给 s2，s1 无效
    println!("s2: {}", s2);
    // println!("s1: {}", s1);  // ❌ 编译错误

    // ✅ 正确示例：函数中的移动
    let s = String::from("world");
    takes_ownership(s);
    // s 不能再使用

    // ✅ 正确示例：返回所有权
    let s = String::from("hello");
    let s = gives_ownership(s);
    println!("s: {}", s);

    // 尝试修改：
    // 1. 创建自己的函数演示移动
    // 2. 思考：为什么 Rust 要设计移动语义？
    // 3. 尝试使用 &str 避免移动
}

// 获取所有权的函数
fn takes_ownership(s: String) {
    println!("获取所有权：{}", s);
} // s 被 drop

// 返回所有权的函数
fn gives_ownership(s: String) -> String {
    s
}
