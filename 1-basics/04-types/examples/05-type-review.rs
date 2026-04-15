//! # 示例：类型实战总结
//!
//! 对应章节：05-实战总结
//! 运行：cargo run --example 05-type-review

fn main() {
    // ✅ 正确示例：综合使用所有类型

    // 基本类型
    let age: u32 = 25;
    let height: f64 = 1.75;
    let is_student: bool = true;
    let initial: char = 'A';

    println!("基本类型：");
    println!("年龄：{} 岁", age);
    println!("身高：{} 米", height);
    println!("学生：{}", is_student);
    println!("首字母：{}", initial);

    // 复合类型
    let point: (f64, f64) = (3.0, 4.0);
    let scores: [u32; 3] = [95, 87, 92];

    println!("\n复合类型：");
    println!("坐标：{:?}", point);
    println!("成绩：{:?}", scores);

    // 字符串类型
    let name = String::from("张三");
    let greeting: &str = "你好，Rust！";

    println!("\n字符串类型：");
    println!("名字：{}", name);
    println!("问候：{}", greeting);

    // 类型转换
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;
    println!("\n类型转换：");
    println!("i32 → f64: {} → {}", int_val, float_val);

    // 尝试修改：
    // 1. 创建更多复合类型示例
    // 2. 尝试隐式类型转换（会失败，思考为什么）
    // 3. 使用 dbg! 打印所有变量查看详细信息

    println!("\n类型系统完成！继续学习所有权。");
}
