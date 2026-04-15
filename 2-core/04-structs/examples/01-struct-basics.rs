//! # 示例：结构体基础
//!
//! 对应章节：01-结构体基础
//! 运行：cargo run --example 01-struct-basics

// ✅ 正确示例：结构体定义
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u64,
}

// ✅ 正确示例：元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ✅ 正确示例：单元结构体
struct AlwaysEqual;

fn main() {
    // ✅ 创建结构体实例
    let user1 = User {
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        active: true,
        login_count: 1,
    };

    println!("用户：{}", user1.username);
    println!("邮箱：{}", user1.email);
    println!("活跃：{}", user1.active);

    // ✅ 修改可变结构体
    let mut user2 = User {
        username: String::from("anotheruser"),
        email: String::from("another@example.com"),
        active: false,
        login_count: 0,
    };
    user2.active = true;
    user2.login_count += 1;

    // ✅ 元组结构体
    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    println!("黑色：RGB({}, {}, {})", black.0, black.1, black.2);

    // ✅ 单元结构体
    let _always_equal = AlwaysEqual;

    // ✅ 结构体更新语法
    let user3 = User {
        email: String::from("new@example.com"),
        ..user2
    };
    println!("user3 用户名：{}", user3.username);

    // 尝试修改：
    // 1. 创建你自己的结构体
    // 2. 使用更新语法创建新实例
    // 3. 思考：结构体和元组的区别？
}
