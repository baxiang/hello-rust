//! # 示例：基本类型
//!
//! 对应章节：02-基本类型
//! 运行：cargo run --example 02-primitive-types

fn main() {
    // ✅ 正确示例：整数类型
    let i32_val: i32 = -42;
    let u32_val: u32 = 42;
    let i64_val: i64 = 100_000_000;

    println!("整数类型：");
    println!("i32: {}", i32_val);
    println!("u32: {}", u32_val);
    println!("i64: {}", i64_val);

    // ✅ 正确示例：浮点类型
    let f32_val: f32 = std::f32::consts::PI;
    let f64_val: f64 = std::f64::consts::PI;

    println!("\n浮点类型：");
    println!("f32: {}", f32_val);
    println!("f64: {}", f64_val);

    // ✅ 正确示例：布尔类型
    let is_true: bool = true;
    let is_false: bool = false;
    println!("\n布尔类型：{} {}", is_true, is_false);

    // ✅ 正确示例：字符类型
    let letter: char = 'A';
    let emoji: char = '😊';
    println!("\n字符类型：{} {}", letter, emoji);

    // 尝试修改：
    // 1. 添加更多整数类型 (i8, u8, i16, u16)
    // 2. 尝试整数运算溢出
    // 3. 思考：为什么默认整数是 i32 而不是 i64？
}
