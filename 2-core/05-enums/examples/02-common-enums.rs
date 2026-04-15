//! # 示例：常用枚举
//!
//! 对应章节：02-常用枚举
//! 运行：cargo run --example 02-common-enums

fn main() {
    // ✅ 正确示例：Option<T> 枚举
    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;

    println!("Some(5) = {:?}", some_number);
    println!("Some(\"hello\") = {:?}", some_string);
    println!("None = {:?}", absent_number);

    // ✅ 正确示例：使用 Option
    fn find_user(id: u32) -> Option<&'static str> {
        match id {
            1 => Some("张三"),
            2 => Some("李四"),
            _ => None,
        }
    }

    let user = find_user(1);
    match user {
        Some(name) => println!("找到用户：{}", name),
        None => println!("用户不存在"),
    }

    // ✅ 正确示例：Option 方法
    let some_value = Some(10);
    let none_value: Option<i32> = None;
    println!("is_some: {}", some_value.is_some());
    println!("is_none: {}", some_value.is_none());
    println!("none.is_none: {}", none_value.is_none());

    // ✅ 正确示例：Result<T, E> 简介
    let ok_result: Result<i32, &str> = Ok(42);
    let _err_result: Result<i32, &str> = Err("error");

    match ok_result {
        Ok(val) => println!("成功：{}", val),
        Err(e) => println!("失败：{}", e),
    }

    // 尝试修改：
    // 1. 创建自己的 Option 处理函数
    // 2. 尝试使用 unwrap（会 panic）
    // 3. 思考：为什么 Rust 没有 null？
}
