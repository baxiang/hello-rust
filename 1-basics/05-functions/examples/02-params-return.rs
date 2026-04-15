//! # 示例：参数与返回值
//!
//! 对应章节：02-参数与返回值
//! 运行：cargo run --example 02-params-return

// ✅ 正确示例：多参数函数
fn print_info(name: &str, age: u32, city: &str) {
    println!("姓名: {}, 年龄: {}, 城市: {}", name, age, city);
}

// ✅ 正确示例：多返回值（使用元组）
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &num in numbers {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }
    (min, max)
}

// ✅ 正确示例：无返回值（隐式返回 ()）
fn say_hello() {
    println!("Hello!");
    // 隐式返回 ()
}

fn main() {
    // 调用多参数函数
    print_info("张三", 25, "北京");

    // 调用多返回值函数
    let nums = [3, 7, 1, 9, 4];
    let (min, max) = min_max(&nums);
    println!("\n最小值: {}, 最大值: {}", min, max);

    // 无返回值函数
    say_hello();

    // 尝试修改：
    // 1. 创建一个返回三个值的函数
    // 2. 尝试修改参数类型
    // 3. 思考：元组返回值的解构方式有哪些？
}
