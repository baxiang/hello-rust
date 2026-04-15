//! # 示例：函数实战总结
//!
//! 对应章节：06-实战总结
//! 运行：cargo run --example 06-function-review

// ✅ 正确示例：综合函数示例
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn print_result(operation: &str, result: f64) {
    println!("{}: {:.2}", operation, result);
}

fn main() {
    // 1. 基础函数调用
    println!("=== 函数实战总结 ===\n");

    let width = 5.0;
    let height = 3.0;
    let area = calculate_area(width, height);
    print_result("矩形面积", area);

    // 2. 闭包使用
    let multiply = |a: f64, b: f64| a * b;
    let volume = multiply(width, height) * 2.0;
    print_result("体积", volume);

    // 3. 函数组合
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    let average = sum as f64 / count as f64;
    print_result("平均值", average);

    // 尝试修改：
    // 1. 创建一个计算圆面积的函数
    // 2. 使用闭包创建过滤器
    // 3. 创建一个递归函数计算幂

    println!("\n函数章节完成！");
}
