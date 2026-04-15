//! # 示例：HashMap 高级特性
//!
//! 对应章节：03-高级特性
//! 运行：cargo run --example 03-hashmap-advanced

use std::collections::HashMap;

fn main() {
    // ✅ 正确示例：自定义键类型
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut map = HashMap::new();
    map.insert(Point { x: 0, y: 0 }, "原点");
    map.insert(Point { x: 1, y: 0 }, "x 轴");
    println!("自定义键：{:?}", map.get(&Point { x: 0, y: 0 }));

    // ✅ 正确示例：HashMap 容量
    let mut data: HashMap<i32, i32> = HashMap::with_capacity(100);
    println!("初始容量：{}", data.capacity());

    for i in 0..50 {
        data.insert(i, i * 2);
    }
    println!("插入 50 个元素后容量：{}", data.capacity());

    // ✅ 正确示例：合并 HashMap
    let mut a = HashMap::from([(1, 10), (2, 20)]);
    let b = HashMap::from([(2, 200), (3, 300)]);
    for (k, v) in b {
        a.entry(k).or_insert(v);
    }
    println!("合并后：{:?}", a);

    // 尝试修改：
    // 1. 实现一个 LRU 缓存
    // 2. 思考：HashMap 性能瓶颈在哪里？
    // 3. 尝试使用 BTreeMap 替代
}
