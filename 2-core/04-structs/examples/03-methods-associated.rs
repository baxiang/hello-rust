//! # 示例：方法与关联函数
//!
//! 对应章节：03-方法与关联函数
//! 运行：cargo run --example 03-methods-associated

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ✅ 正确示例：方法实现
impl Rectangle {
    // 实例方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数（构造函数）
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // ✅ 使用关联函数创建
    let rect1 = Rectangle::new(30, 50);
    let square = Rectangle::square(20);

    println!("矩形 1：{:?}", rect1);
    println!("面积：{}", rect1.area());
    println!("周长：{}", rect1.perimeter());

    println!("\n正方形：{:?}", square);
    println!("面积：{}", square.area());

    // ✅ 方法调用
    let rect2 = Rectangle::new(10, 20);
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));

    // ✅ 多个方法
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1 能容纳 rect3 吗？{}", rect1.can_hold(&rect3));

    // 尝试修改：
    // 1. 添加一个新方法计算对角线长度
    // 2. 创建一个 scale 方法放大矩形
    // 3. 思考：方法和函数的区别？
}
