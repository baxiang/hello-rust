//! # 示例：Trait 约束
//!
//! 对应章节：03-Trait约束
//! 运行：cargo run --example 03-trait-bounds

// ✅ 正确示例：Trait 约束
use std::fmt::Display;

fn display_pair<T: Display>(a: &T, b: &T) {
    println!("{}, {}", a, b);
}

// ✅ 正确示例：多 Trait 约束
fn display_and_debug<T: Display + std::fmt::Debug>(item: &T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// ✅ 正确示例：where 子句
fn complex_function<T, U>(a: T, b: U) -> String
where
    T: Display + Clone,
    U: std::fmt::Debug,
{
    format!("{}: {:?}", a.clone(), b)
}

// ✅ 正确示例：返回 Trait 约束
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    display_pair(&"hello", &"world");

    let num = 42;
    display_and_debug(&num);

    let result = complex_function("test", vec![1, 2, 3]);
    println!("\ncomplex: {}", result);

    let numbers = vec![34, 50, 25, 100];
    println!("\nlargest: {}", largest(&numbers));

    // 尝试修改：
    // 1. 创建自己的 Trait 约束
    // 2. 使用 where 子句简化签名
    // 3. 思考：Trait 约束如何提供灵活性？
}
