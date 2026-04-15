//! # 示例：集合实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-collections-review

fn main() {
    println!("=== 集合实战总结 ===\n");

    // 1. 数组
    let scores = [95, 87, 92, 78, 100];
    println!("成绩数组：{:?}", scores);
    let sum: i32 = scores.iter().sum();
    let avg = sum as f64 / scores.len() as f64;
    println!("平均分：{:.1}", avg);

    // 2. Vec
    let mut todo_list = Vec::new();
    todo_list.push("学习 Rust");
    todo_list.push("写代码");
    todo_list.push("测试");
    println!("\n待办事项：");
    for (i, task) in todo_list.iter().enumerate() {
        println!("  {}. {}", i + 1, task);
    }

    // 3. 综合：数据处理
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let mut sorted = data.clone();
    sorted.sort();
    println!("\n原始数据：{:?}", data);
    println!("排序后：{:?}", sorted);

    // 4. 过滤和转换
    let evens: Vec<i32> = data.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let doubled: Vec<i32> = data.iter().map(|&x| x * 2).collect();
    println!("偶数：{:?}", evens);
    println!("翻倍：{:?}", doubled);

    // 尝试修改：
    // 1. 创建自己的数据集合
    // 2. 使用迭代器方法处理
    // 3. 思考：集合类型如何选择？

    println!("\n集合章节完成！");
}
