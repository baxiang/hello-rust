//! # 示例：函数与 Copy
//!
//! 对应章节：03-函数与Copy
//! 运行：cargo run --example 03-function-copy

// ✅ 正确示例：Copy 类型
fn print_number(x: i32) {
    println!("x = {}", x);
}

// ✅ 正确示例：非 Copy 类型
fn print_string(s: String) {
    println!("s = {}", s);
}

// ✅ 正确示例：返回多个值
fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

fn main() {
    // ✅ Copy 类型自动复制
    let num = 5;
    print_number(num);
    println!("num 仍然可用：{}", num);

    // ✅ Copy 类型列表
    // 所有标量类型：i8, i16, i32, i64, u8, u16, u32, u64
    // f32, f64, bool, char, 元组（仅包含 Copy 类型）

    let x: (i32, f64, u8) = (500, std::f64::consts::PI, 1);
    let _y = x;
    println!("x 仍然可用：{:?}", x);

    // ✅ 非 Copy 类型需要显式处理
    let s = String::from("hello");
    print_string(s.clone()); // 克隆传递
    println!("s 仍然可用：{}", s);

    // ✅ 返回元组避免多次移动
    let (sum, product) = calculate(3, 4);
    println!("和：{}, 积：{}", sum, product);

    // 尝试修改：
    // 1. 创建自己的 Copy 类型函数
    // 2. 尝试传递 String 观察所有权转移
    // 3. 思考：什么时候使用 Clone，什么时候使用引用？
}
