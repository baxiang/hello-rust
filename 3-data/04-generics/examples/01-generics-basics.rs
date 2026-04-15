//! # 示例：泛型基础
//!
//! 对应章节：01-泛型基础
//! 运行：cargo run --example 01-generics-basics

// ✅ 正确示例：泛型函数
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ✅ 正确示例：泛型函数（适用于所有 PartialOrd）
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ✅ 正确示例：泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// ✅ 正确示例：泛型枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // ✅ 泛型函数
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大数字：{}", largest_i32(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符：{}", largest(&chars));

    // ✅ 泛型结构体
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点：({}, {})", int_point.x, int_point.y);
    println!("浮点点：({}, {})", float_point.x, float_point.y);

    // ✅ 泛型枚举
    let ok: Result<i32, &str> = Result::Ok(42);
    let err: Result<i32, &str> = Result::Err("error");
    match ok {
        Result::Ok(v) => println!("Ok: {}", v),
        Result::Err(e) => println!("Err: {}", e),
    }
    match err {
        Result::Ok(v) => println!("Ok: {}", v),
        Result::Err(e) => println!("Err: {}", e),
    }

    // 尝试修改：
    // 1. 创建一个泛型函数
    // 2. 添加泛型类型约束
    // 3. 思考：泛型如何避免代码重复？
}
