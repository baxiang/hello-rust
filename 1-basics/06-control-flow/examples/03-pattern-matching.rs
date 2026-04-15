//! # 示例：模式匹配
//!
//! 对应章节：03-模式匹配
//! 运行：cargo run --example 03-pattern-matching

fn main() {
    // ✅ 正确示例：基本 match
    let number = 7;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 => println!("质数"),
        4..=8 => println!("4 到 8 之间"),
        _ => println!("其他"),
    }

    // ✅ 正确示例：Option 匹配
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    match some_value {
        Some(x) => println!("some_value 包含：{}", x),
        None => println!("some_value 为空"),
    }

    match none_value {
        Some(x) => println!("none_value 包含：{}", x),
        None => println!("none_value 为空"),
    }

    // ✅ 正确示例：if let 简化匹配
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值：{}", max);
    }

    // ✅ 正确示例：while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("弹出：{}", top);
    }

    // 尝试修改：
    // 1. 创建一个匹配元组的模式
    // 2. 使用 match 处理 Result 类型
    // 3. 思考：match 必须穷尽所有可能，为什么？
}
