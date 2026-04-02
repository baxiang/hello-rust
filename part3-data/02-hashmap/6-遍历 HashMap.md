## 13.6 遍历 HashMap

### 基本遍历

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    scores.insert("Red", 30);

    // 遍历所有键值对（借用）
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 遍历所有键值对（可变借用）
    for (key, value) in &mut scores {
        *value += 10;
    }
    println!("加分后：{:?}", scores);

    // 获取所有权遍历（消耗 HashMap）
    for (key, value) in scores {
        println!("{} 的分数是 {}", key, value);
    }
    // scores 已移动，不能再使用
}
```

### keys() 和 values() 迭代器

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    scores.insert("Red", 30);

    // 只遍历键
    println!("所有队伍:");
    for team in scores.keys() {
        println!("  {}", team);
    }

    // 只遍历值
    println!("所有分数:");
    for score in scores.values() {
        println!("  {}", score);
    }

    // 收集键或值
    let teams: Vec<&str> = scores.keys().copied().collect();
    let scores_vec: Vec<i32> = scores.values().copied().collect();
}
```

### 注意：无序性

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    // HashMap 不保证遍历顺序
    // 每次运行的顺序可能不同！
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    // 如果需要有序遍历，先收集再排序
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));

    println!("排序后:");
    for (k, v) in items {
        println!("{}: {}", k, v);
    }
}
```

---

---

## 下一步

[HashMap 与所有权](../7-HashMap 与所有权.md)