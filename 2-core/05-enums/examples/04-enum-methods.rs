//! # 示例：枚举方法
//!
//! 对应章节：04-枚举方法
//! 运行：cargo run --example 04-enum-methods

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

impl WebEvent {
    fn inspect(&self) {
        match self {
            WebEvent::PageLoad => println!("页面加载"),
            WebEvent::PageUnload => println!("页面卸载"),
            WebEvent::KeyPress(c) => println!("按键：{}", c),
            WebEvent::Paste(s) => println!("粘贴：{}", s),
            WebEvent::Click { x, y } => println!("点击：({}, {})", x, y),
        }
    }

    fn is_click(&self) -> bool {
        matches!(self, WebEvent::Click { .. })
    }
}

fn main() {
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Click { x: 100, y: 200 },
        WebEvent::Paste("hello".to_string()),
        WebEvent::PageUnload,
    ];

    for event in events {
        event.inspect();
        println!("是点击吗？{}", event.is_click());
    }

    // ✅ Option 方法
    let some = Some(5);
    let none: Option<i32> = None;
    let default = none.unwrap_or(0);

    println!("\nOption 方法：");
    println!("some.map(|x| x + 1) = {:?}", some.map(|x| x + 1));
    println!("none.map(|x| x + 1) = {:?}", none.map(|x| x + 1));
    println!("none.unwrap_or(0) = {}", default);

    // 尝试修改：
    // 1. 添加更多 WebEvent 变体
    // 2. 创建枚举的构造方法
    // 3. 思考：枚举方法 vs 结构体方法？
}
