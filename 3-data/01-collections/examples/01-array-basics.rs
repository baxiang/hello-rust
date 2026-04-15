//! # 示例：数组基础
//!
//! 对应章节：01-数组基础
//! 运行：cargo run --example 01-array-basics

fn main() {
    // ✅ 正确示例：数组声明
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组：{:?}", numbers);

    // ✅ 正确示例：相同值数组
    let zeros = [0; 10];
    println!("10 个零：{:?}", zeros);

    // ✅ 正确示例：访问元素
    let first = numbers[0];
    let last = numbers[4];
    println!("第一个：{}，最后一个：{}", first, last);

    // ✅ 正确示例：安全访问
    if let Some(val) = numbers.get(10) {
        println!("索引 10：{}", val);
    } else {
        println!("索引 10 越界");
    }

    // 尝试修改：
    // 1. 创建自己的数组
    // 2. 尝试越界访问（会 panic）
    // 3. 思考：数组和 Vec 的区别？
}
