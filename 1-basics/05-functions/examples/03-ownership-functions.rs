//! # 示例：所有权与函数
//!
//! 对应章节：03-所有权与函数
//! 运行：cargo run --example 03-ownership-functions

// ✅ 正确示例：所有权转移
fn take_ownership(s: String) {
    println!("获取所有权：{}", s);
} // s 离开作用域，被 drop

// ✅ 正确示例：借用（不获取所有权）
fn borrow_string(s: &String) {
    println!("借用：{}", s);
} // s 只是借用，不 drop

// ✅ 正确示例：返回所有权
fn give_back(s: String) -> String {
    s // 返回所有权
}

fn main() {
    let my_string = String::from("Hello");

    // 借用（推荐）
    borrow_string(&my_string);
    println!("my_string 仍然可用：{}", my_string);

    // 转移所有权
    let another = String::from("World");
    take_ownership(another);
    // another 不能再使用

    // 返回所有权
    let s = String::from("Rust");
    let s = give_back(s);
    println!("返回后：{}", s);

    // 尝试修改：
    // 1. 调用 take_ownership(my_string) 后尝试使用 my_string
    // 2. 思考：什么时候应该借用，什么时候应该转移所有权？
}
