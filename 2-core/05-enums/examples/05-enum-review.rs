//! # 示例：枚举实战总结
//!
//! 对应章节：05-实战总结
//! 运行：cargo run --example 05-enum-review

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn main() {
    println!("=== 枚举实战总结 ===\n");

    // 1. 基本枚举
    let dir = Direction::North;
    println!("方向：{:?}", dir);
    println!("反方向：{:?}", dir.opposite());

    // 2. Option<T>
    let numbers = [1, 2, 3, 4, 5];
    let first = numbers.first();
    match first {
        Some(n) => println!("第一个元素：{}", n),
        None => println!("空列表"),
    }

    // 3. Result<T, E>
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误：{}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("错误：{}", e),
    }

    // 尝试修改：
    // 1. 创建你自己的枚举和方法
    // 2. 使用 Option 处理可能失败的操作
    // 3. 思考：枚举如何替代继承？

    println!("\n枚举章节完成！核心概念部分结束。");
}
