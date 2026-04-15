//! # 示例：HashMap 实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-hashmap-review

use std::collections::HashMap;

fn main() {
    println!("=== HashMap 实战总结 ===\n");

    // 1. 基本操作
    let mut phone_book = HashMap::new();
    phone_book.insert("张三", "138-0001");
    phone_book.insert("李四", "139-0002");
    phone_book.insert("王五", "137-0003");

    println!("电话簿：");
    for (name, phone) in &phone_book {
        println!("  {}: {}", name, phone);
    }

    // 2. 词频统计
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    println!("\n词频统计：");
    for (word, count) in &word_count {
        println!("  {}: {}", word, count);
    }

    // 3. 分组
    let people = vec![("张三", "开发部"), ("李四", "开发部"), ("王五", "设计部")];
    let mut departments: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, dept) in people {
        departments.entry(dept).or_default().push(name);
    }
    println!("\n部门分组：");
    for (dept, members) in &departments {
        println!("  {}: {:?}", dept, members);
    }

    // 尝试修改：
    // 1. 创建自己的数据分组
    // 2. 尝试使用 BTreeMap
    // 3. 思考：HashMap 的哈希算法如何工作？

    println!("\nHashMap 章节完成！");
}
