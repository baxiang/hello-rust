## 13.9 HashMap 常用方法大全

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    // ========== 基本操作 ==========
    // 插入
    map.insert(String::from("a"), 1);

    // 获取
    let val = map.get(&String::from("a"));  // Option<&i32>

    // 获取可变引用
    let val = map.get_mut(&String::from("a"));  // Option<&mut i32>

    // 删除
    let removed = map.remove(&String::from("a"));  // Option<i32>

    // ========== 查询 ==========
    // 是否包含键
    let exists = map.contains_key(&String::from("a"));  // bool

    // 是否为空
    let empty = map.is_empty();  // bool

    // 长度
    let len = map.len();  // usize

    // ========== entry API ==========
    // 获取或插入
    let val = map.entry(String::from("b")).or_insert(2);

    // 获取或插入（使用闭包）
    let val = map.entry(String::from("c")).or_insert_with(|| 3);

    // 如果存在则修改
    map.entry(String::from("a")).and_modify(|v| *v += 1);

    // ========== 容量管理 ==========
    // 容量
    let cap = map.capacity();

    // 缩小容量
    map.shrink_to_fit();

    // 保留容量
    map.reserve(100);

    // ========== 批量操作 ==========
    // 清空
    map.clear();

    // 保留满足条件的元素
    map.retain(|k, v| *v > 0);

    // ========== 迭代器 ==========
    // 迭代（借用）
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }

    // 迭代（可变借用）
    for (k, v) in map.iter_mut() {
        *v += 1;
    }

    // 只迭代键
    for k in map.keys() {
        println!("{}", k);
    }

    // 只迭代值
    for v in map.values() {
        println!("{}", v);
    }
}
```

---

---

## 下一步

[完整示例](../10-完整示例.md)