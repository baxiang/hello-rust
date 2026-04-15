//! # 示例：泛型类型
//!
//! 对应章节：02-泛型类型
//! 运行：cargo run --example 02-generic-types

// ✅ 正确示例：泛型结构体方法
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// ✅ 正确示例：特定类型的方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ✅ 正确示例：多泛型参数
struct MultiPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("Point x: {}", p.x());

    let pf = Point { x: 3.0, y: 4.0 };
    println!("距离原点：{}", pf.distance_from_origin());

    let mp = MultiPoint { x: 5, y: 4.0 };
    println!("MultiPoint: ({}, {})", mp.x, mp.y);

    // 尝试修改：
    // 1. 添加更多泛型方法
    // 2. 思考：什么时候需要特定类型的方法？
}
