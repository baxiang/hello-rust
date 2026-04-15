//! # 示例：高级函数
//!
//! 对应章节：04-高级函数
//! 运行：cargo run --example 04-advanced-functions

// ✅ 正确示例：函数作为参数（函数指针）
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn double(x: i32) -> i32 {
    x * 2
}

fn square(x: i32) -> i32 {
    x * x
}

// ✅ 正确示例：泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // 函数作为参数
    let result = apply_twice(double, 5);
    println!("double(double(5)) = {}", result);

    let result = apply_twice(square, 3);
    println!("square(square(3)) = {}", result);

    // 泛型函数
    let numbers = vec![34, 50, 25, 100, 65];
    println!("\n最大数字：{}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符：{}", largest(&chars));

    // 尝试修改：
    // 1. 创建一个新的函数并传递给 apply_twice
    // 2. 创建自己的泛型函数
    // 3. 思考：函数指针和闭包的区别？
}
