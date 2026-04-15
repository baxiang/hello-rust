//! # 示例：迭代器基础
//!
//! 对应章节：01-迭代器基础
//! 运行：cargo run --example 01-iterator-basics

fn main() {
    // ✅ 正确示例：创建迭代器
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    // ✅ 正确示例：next 方法
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());

    // ✅ 正确示例：消费适配器
    let v2 = vec![1, 2, 3, 4, 5];
    let sum: i32 = v2.iter().sum();
    println!("sum: {}", sum);

    let collected: Vec<&i32> = v2.iter().collect();
    println!("collected: {:?}", collected);

    // ✅ 正确示例：for 循环使用迭代器
    for item in &v2 {
        print!("{} ", item);
    }
    println!();

    // 尝试修改：
    // 1. 创建自己的迭代器
    // 2. 思考：迭代器和索引访问的区别？
    // 3. 尝试使用 into_iter()
}
