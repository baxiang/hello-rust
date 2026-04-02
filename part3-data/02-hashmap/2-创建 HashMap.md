## 13.2 创建 HashMap

### 导入 HashMap

```rust
// HashMap 不在标准库的预导入模块中
// 必须显式导入
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```

### 创建 HashMap 的五种方式

```rust
use std::collections::HashMap;

fn main() {
    // 方式 1：使用 new() 创建空的 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 方式 2：使用宏直接初始化（需要多次 insert）
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    // 方式 3：从迭代器 collect（推荐）
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    // 方式 4：使用数组和 collect
    let scores: HashMap<_, _> = [
        ("Blue", 10),
        ("Yellow", 50),
        ("Red", 30),
    ].into_iter()
    .collect();

    // 方式 5：预分配容量
    let mut large_map: HashMap<i32, String> = HashMap::with_capacity(1000);

    println!("scores: {:?}", scores);
    println!("large_map capacity: {}", large_map.capacity());
}
```

### 类型推断

```rust
use std::collections::HashMap;

fn main() {
    // Rust 通常可以推断类型
    let mut map = HashMap::new();
    map.insert(1, "one");
    // 类型推断为 HashMap<i32, &str>

    // 空 HashMap 需要类型标注
    let mut empty: HashMap<String, i32> = HashMap::new();
    let mut empty2 = HashMap::<String, i32>::new();

    // 使用下划线占位符
    let map: HashMap<_, _> = [(1, "one"), (2, "two")].into_iter().collect();
}
```

---

---

## 下一步

[插入和访问元素](../3-插入和访问元素.md)