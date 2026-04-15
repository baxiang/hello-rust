//! # 示例：错误诊断实战
//!
//! 对应章节：05-错误诊断实战
//! 运行：cargo run --example 05-error-diagnosis

fn main() {
    println!("=== 所有权常见错误诊断 ===\n");

    // ✅ 错误 1：使用已移动的值
    // 取消注释观察 E0382 错误：
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1: {}", s1);  // ❌ E0382
    println!("错误 1：E0382 - 使用已移动的值");
    println!("修复：使用 .clone() 或引用\n");

    // ✅ 错误 2：借用冲突
    // 取消注释观察 E0502 错误：
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);  // ❌ E0502
    println!("错误 2：E0502 - 不可变和可变借用冲突");
    println!("修复：分离借用生命周期\n");

    // ✅ 错误 3：悬垂引用
    // 取消注释观察 E0515 错误：
    // let r = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // ❌ E0515
    // }
    println!("错误 3：E0515 - 返回悬垂引用");
    println!("修复：返回 String 而非引用\n");

    println!("编译器错误是最好的老师！");
}
