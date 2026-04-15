//! # 示例：HashMap 操作
//!
//! 对应章节：02-HashMap操作
//! 运行：cargo run --example 02-hashmap-operations

use std::collections::HashMap;

fn main() {
    // ✅ 正确示例：插入和更新
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("插入后：{:?}", map);

    // 只在键不存在时插入
    map.entry("c").or_insert(3);
    map.entry("a").or_insert(100); // 不会覆盖
    println!("or_insert 后：{:?}", map);

    // ✅ 正确示例：更新已有值
    let text = "hello world hello";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("\n词频统计：{:?}", counts);

    // ✅ 正确示例：删除
    let mut scores = HashMap::new();
    scores.insert(1, 100);
    scores.insert(2, 200);
    if let Some(val) = scores.remove(&1) {
        println!("删除了键 1，值：{}", val);
    }
    println!("删除后：{:?}", scores);

    // 尝试修改：
    // 1. 实现一个简单的缓存
    // 2. 使用 entry API 优化代码
    // 3. 思考：HashMap 的哈希冲突如何处理？
}
