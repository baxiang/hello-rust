//! # 示例：结构体语法
//!
//! 对应章节：02-结构体语法
//! 运行：cargo run --example 02-struct-syntax

// ✅ 正确示例：带推导宏的结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ✅ 正确示例：结构体作为参数
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形：{:?}", rect1);
    println!("面积：{}", area(&rect1));

    // ✅ 结构体字段访问
    println!("宽度：{}", rect1.width);
    println!("高度：{}", rect1.height);

    // ✅ 结构体嵌套
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        address: Address,
    }

    #[derive(Debug)]
    struct Address {
        city: String,
        country: String,
    }

    let person = Person {
        name: String::from("张三"),
        age: 25,
        address: Address {
            city: String::from("北京"),
            country: String::from("中国"),
        },
    };
    println!("\n人物：{:#?}", person);
    println!("城市：{}", person.address.city);

    // 尝试修改：
    // 1. 添加更多字段到 Rectangle
    // 2. 创建嵌套结构体
    // 3. 思考：为什么需要 #[derive(Debug)]？
}
