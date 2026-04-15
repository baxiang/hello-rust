//! # 示例：条件表达式
//!
//! 对应章节：01-条件表达式
//! 运行：cargo run --example 01-conditionals

fn main() {
    // ✅ 正确示例：if 表达式
    let number = 5;

    if number > 0 {
        println!("{} 是正数", number);
    } else if number < 0 {
        println!("{} 是负数", number);
    } else {
        println!("{} 是零", number);
    }

    // ✅ 正确示例：if 作为表达式
    let is_even = if number % 2 == 0 { "偶数" } else { "奇数" };
    println!("{} 是 {}", number, is_even);

    // ✅ 正确示例：match 表达式
    let grade = 'B';
    let message = match grade {
        'A' => "优秀",
        'B' => "良好",
        'C' => "及格",
        'D' => "需努力",
        _ => "其他",
    };
    println!("等级 {}: {}", grade, message);

    // 尝试修改：
    // 1. 使用 if 判断字符串是否包含特定字符
    // 2. 添加更多 match 分支
    // 3. 思考：if 和 match 的区别是什么？
}
