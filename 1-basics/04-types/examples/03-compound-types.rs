//! # 示例：复合类型
//!
//! 对应章节：03-复合类型
//! 运行：cargo run --example 03-compound-types

fn main() {
    // ✅ 正确示例：元组 (Tuple)
    let tuple: (i32, f64, &str) = (42, std::f64::consts::PI, "hello");
    println!("元组：{:?}", tuple);

    // 解构元组
    let (x, y, z) = tuple;
    println!("解构：x={}, y={}, z={}", x, y, z);

    // 索引访问
    let first = tuple.0;
    println!("索引访问：tuple.0 = {}", first);

    // ✅ 正确示例：数组 (Array)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\n数组：{:?}", numbers);

    // 访问数组元素
    let first_num = numbers[0];
    println!("第一个元素：{}", first_num);

    // 相同值数组
    let zeros = [0; 10];
    println!("10 个零：{:?}", zeros);

    // 数组长度
    let len = numbers.len();
    println!("数组长度：{}", len);

    // 尝试修改：
    // 1. 创建一个包含不同类型的元组
    // 2. 尝试访问越界的数组索引 (numbers[5])
    // 3. 思考：元组和数组的区别是什么？
}
