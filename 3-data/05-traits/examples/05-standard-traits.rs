//! # 示例：标准库 Trait
//!
//! 对应章节：05-标准库Trait
//! 运行：cargo run --example 05-standard-traits

use std::fmt;
use std::ops::Add;

// ✅ 正确示例：派生 Trait
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

// ✅ 正确示例：Display Trait
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ✅ 正确示例：Add Trait
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    // 派生 Trait
    let p1 = Person {
        name: "张三".into(),
        age: 25,
    };
    let p2 = p1.clone();
    println!("p1 == p2: {}", p1 == p2);
    println!("p1: {:?}", p1);

    // Display
    let point = Point { x: 10, y: 20 };
    println!("\nPoint: {}", point);

    // Add
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("Point 相加：{}", p3);

    // 常见派生 Trait 总结
    println!("\n常见派生 Trait：");
    println!("  Debug    - 调试输出");
    println!("  Clone    - 克隆");
    println!("  Copy     - 复制");
    println!("  PartialEq - 相等比较");
    println!("  Default  - 默认值");
    println!("  Hash     - 哈希");

    // 尝试修改：
    // 1. 实现自己的 Display
    // 2. 尝试实现其他标准库 Trait
}
