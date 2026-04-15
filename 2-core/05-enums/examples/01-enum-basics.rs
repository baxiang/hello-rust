//! # 示例：枚举基础
//!
//! 对应章节：01-枚举基础
//! 运行：cargo run --example 01-enum-basics

// ✅ 正确示例：基本枚举
enum IpAddrKind {
    V4,
    V6,
}

// ✅ 正确示例：带数据的枚举
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

// ✅ 正确示例：不同类型数据
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // ✅ 创建枚举值
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // ✅ 带数据的枚举
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    // ✅ 枚举变量
    let _m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 30 };
    let m3 = Message::Write(String::from("hello"));
    let _m4 = Message::ChangeColor(255, 0, 0);

    // 使用枚举值
    if let Message::Move { x, y } = m2 {
        println!("移动到：({}, {})", x, y);
    }
    if let Message::Write(s) = m3 {
        println!("写入：{}", s);
    }

    println!("枚举值创建完成！");
    println!("IP 类型：V4, V6");
    println!("消息类型：Quit, Move, Write, ChangeColor");

    // 尝试修改：
    // 1. 创建你自己的枚举
    // 2. 添加更多变体
    // 3. 思考：枚举和结构体的区别？
}
