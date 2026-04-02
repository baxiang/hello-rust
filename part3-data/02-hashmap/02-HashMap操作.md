## 13.3 插入和访问元素

### 插入元素（insert）

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // insert 返回 Option<V>
    // 如果键已存在，返回旧值（Some）
    // 如果键不存在，返回 None
    let old = scores.insert(String::from("Blue"), 10);
    println!("旧值：{:?}", old);  // None

    // 再次插入相同的键，会覆盖
    let old = scores.insert(String::from("Blue"), 20);
    println!("旧值：{:?}", old);  // Some(10)

    println!("当前：{:?}", scores);  // {"Blue": 20}
}
```

### 访问元素（get）

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get 返回 Option<&V>（值的引用）
    let blue_score = scores.get(&String::from("Blue"));
    println!("Blue 分数：{:?}", blue_score);  // Some(10)

    // 使用 match 处理
    match scores.get(&String::from("Red")) {
        Some(score) => println!("Red 分数：{}", score),
        None => println!("Red 不在 HashMap 中"),
    }

    // 使用 if let
    if let Some(score) = scores.get(&String::from("Yellow")) {
        println!("Yellow 分数：{}", score);
    }

    // 可以使用借用的键（不需要所有权）
    let key = String::from("Blue");
    let score = scores.get(&key);  // &key 是 &String
    println!("分数：{:?}", score);

    // key 仍然可用
    println!("键：{}", key);
}
```

### contains_key 检查键是否存在

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    // contains_key 返回 bool
    if scores.contains_key(&"Blue") {
        println!("Blue 在 HashMap 中");
    }

    if !scores.contains_key(&"Red") {
        println!("Red 不在 HashMap 中");
    }
}
```

---

---

## 下一步

[entry API：插入或更新的神器](../4-entry API：插入或更新的神器.md)

---

## 13.5 删除元素

### remove 方法

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    scores.insert("Red", 30);

    println!("删除前：{:?}", scores);

    // remove 返回 Option<V>
    // 如果键存在，返回 Some(值)
    // 如果键不存在，返回 None
    let removed = scores.remove(&"Blue");
    println!("删除 Blue: {:?}", removed);  // Some(10)

    let removed = scores.remove(&"Green");
    println!("删除 Green: {:?}", removed);  // None

    println!("删除后：{:?}", scores);  // {"Yellow": 50, "Red": 30}
}
```

### 保留满足条件的元素（retain）

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 70);
    scores.insert("David", 95);

    // retain - 保留满足条件的元素
    scores.retain(|name, &score| score >= 80);

    println!("80 分以上：{:?}", scores);
    // {"Alice": 90, "Bob": 85, "David": 95}
}
```

### 清空所有元素

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("清空前长度：{}", scores.len());

    scores.clear();

    println!("清空后长度：{}", scores.len());  // 0
    println!("清空后：{:?}", scores);  // {}
}
```

---

---

## 下一步

[遍历 HashMap](../6-遍历 HashMap.md)

---

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

---

## 返回

[返回目录](../../README.md)