//! # 示例：let 绑定
//!
//! 对应章节：01-变量基础
//! 运行：cargo run --example 01-let-binding

fn main() {
    // ✅ 正确示例：基本 let 绑定
    let x = 5;
    println!("x = {}", x);

    // ✅ 正确示例：显式类型标注
    let y: i32 = 10;
    let z: f64 = std::f64::consts::PI;
    println!("y = {}, z = {}", y, z);

    // ✅ 正确示例：使用下划线开头表示暂未使用
    let _unused = 5;

    // 尝试修改：
    // 取消下面的注释观察编译错误：
    // x = 10;  // ❌ E0384: 不能给不可变变量重新赋值
}
