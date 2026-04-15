//! # 示例：控制流实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-control-flow-review

fn main() {
    // ✅ 正确示例：综合控制流

    // 1. 条件 + 循环
    println!("=== FizzBuzz ===");
    for i in 1..=20 {
        if i % 15 == 0 {
            print!("FizzBuzz ");
        } else if i % 3 == 0 {
            print!("Fizz ");
        } else if i % 5 == 0 {
            print!("Buzz ");
        } else {
            print!("{} ", i);
        }
    }
    println!("\n");

    // 2. match + Option
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }

    println!("=== 除法 ===");
    let results = [(10.0, 2.0), (10.0, 0.0), (10.0, 3.0)];
    for (a, b) in results {
        match divide(a, b) {
            Some(result) => println!("{} / {} = {:.2}", a, b, result),
            None => println!("{} / {} = 除数不能为零", a, b),
        }
    }

    // 3. 模式匹配 + 元组
    println!("\n=== 坐标匹配 ===");
    let points = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for (x, y) in points {
        match (x, y) {
            (0, 0) => println!("原点"),
            (_, 0) => println!("x 轴上：({})", x),
            (0, _) => println!("y 轴上：({})", y),
            _ => println!("象限内：({}, {})", x, y),
        }
    }

    // 尝试修改：
    // 1. 修改 FizzBuzz 规则
    // 2. 添加更多除法测试用例
    // 3. 创建一个更复杂的模式匹配示例

    println!("\n控制流章节完成！");
}
