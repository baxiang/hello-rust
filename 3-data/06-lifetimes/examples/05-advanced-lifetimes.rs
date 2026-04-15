//! # 示例：高级生命周期
//!
//! 对应章节：05-高级生命周期
//! 运行：cargo run --example 05-advanced-lifetimes

// ✅ 正确示例：'static 生命周期
const GREETING: &str = "Hello, Rust!";
static VERSION: &str = "1.85";

fn get_static() -> &'static str {
    "这是一个静态字符串"
}

// ✅ 正确示例：生命周期约束
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x: {}, y: {}", x, y);
}

// ✅ 正确示例：Trait 对象生命周期
trait AsRefStr {
    fn as_str(&self) -> &str;
}

impl AsRefStr for String {
    fn as_str(&self) -> &str {
        self
    }
}

fn main() {
    // 'static 使用
    println!("静态常量：{}", GREETING);
    println!("静态变量：{}", VERSION);
    println!("静态函数：{}", get_static());

    // 生命周期约束
    let a = 5;
    let b = 10;
    print_refs(&a, &b);

    // 尝试修改：
    // 1. 创建 'static 引用
    // 2. 思考：什么时候使用 'static？

    println!("\n生命周期章节完成！");
}
