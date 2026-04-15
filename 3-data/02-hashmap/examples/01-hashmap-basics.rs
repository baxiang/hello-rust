//! # 示例：HashMap 基础
//!
//! 对应章节：01-HashMap基础
//! 运行：cargo run --example 01-hashmap-basics

use std::collections::HashMap;

fn main() {
    // ✅ 正确示例：创建 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    // ✅ 正确示例：访问值
    let team = String::from("Blue");
    let score = scores.get(&team);
    match score {
        Some(s) => println!("{} 队得分：{}", team, s),
        None => println!("队伍不存在"),
    }

    // ✅ 正确示例：遍历
    println!("\n所有队伍：");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // ✅ 正确示例：更新值
    scores.insert(String::from("Blue"), 25);
    println!("\n更新后 Blue: {:?}", scores.get("Blue"));

    // 尝试修改：
    // 1. 创建自己的 HashMap
    // 2. 尝试获取不存在的键
    // 3. 思考：HashMap 和 Vec 的区别？
}
