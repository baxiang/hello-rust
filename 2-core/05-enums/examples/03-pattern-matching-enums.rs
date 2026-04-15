//! # 示例：模式匹配
//!
//! 对应章节：03-模式匹配
//! 运行：cargo run --example 03-pattern-matching-enums

#[derive(Debug)]
enum State {
    #[allow(dead_code)]
    Pending,
    Running,
    Completed,
    Failed(String),
}

fn main() {
    // ✅ 正确示例：match 匹配枚举
    let state = State::Running;

    match state {
        State::Pending => println!("任务等待中"),
        State::Running => println!("任务运行中"),
        State::Completed => println!("任务完成"),
        State::Failed(reason) => println!("任务失败：{}", reason),
    }

    // ✅ 正确示例：match 返回值
    let state2 = State::Failed("连接超时".to_string());
    let message = match state2 {
        State::Pending => "等待",
        State::Running => "运行",
        State::Completed => "完成",
        State::Failed(ref reason) => &reason[..],
    };
    println!("状态：{}", message);

    // ✅ 正确示例：if let 简化
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("值是 3");
    }

    // ✅ 正确示例：matches! 宏
    let state3 = State::Completed;
    if matches!(state3, State::Completed) {
        println!("状态是完成");
    }

    // 尝试修改：
    // 1. 添加更多枚举变体
    // 2. 使用 match 处理 Option
    // 3. 思考：match 必须穷尽所有可能，为什么？
}
