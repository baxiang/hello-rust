//! # 示例：消费适配器
//!
//! 对应章节：02-消费适配器
//! 运行：cargo run --example 02-consuming-adapters

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // ✅ sum: 求和
    let sum: i32 = numbers.iter().sum();
    println!("sum: {}", sum);

    // ✅ collect: 收集
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // ✅ count: 计数
    let count = numbers.iter().count();
    println!("count: {}", count);

    // ✅ max/min: 最大最小
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("max: {:?}, min: {:?}", max, min);

    // ✅ find: 查找
    let found = numbers.iter().find(|&&x| x > 3);
    println!("find > 3: {:?}", found);

    // ✅ fold: 折叠
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("product: {}", product);

    // ✅ any/all: 任意/所有
    let any_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("any_even: {}, all_positive: {}", any_even, all_positive);

    // 尝试修改：
    // 1. 使用 fold 实现 sum
    // 2. 尝试 partition 适配器
}
