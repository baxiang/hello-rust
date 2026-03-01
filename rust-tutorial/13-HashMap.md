# 第 13 章：HashMap 详解

> 键值对存储与 O(1) 查找——处理映射关系的核心数据结构

---

## 13.1 为什么需要 HashMap？

### 问题场景

```rust
// 问题：存储和查找学生的分数
// 方案 1：使用 Vec 存储元组（低效）
let students: Vec<(String, i32)> = vec![
    (String::from("Alice"), 90),
    (String::from("Bob"), 85),
    (String::from("Charlie"), 95),
];

// 查找 Alice 的分数需要遍历整个 Vec
fn find_score(students: &[(String, i32)], name: &str) -> Option<i32> {
    for (n, score) in students {
        if n == name {
            return Some(*score);
        }
    }
    None
}
// 时间复杂度：O(n)

// 方案 2：使用 HashMap（高效）
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 90);
scores.insert(String::from("Bob"), 85);

let alice_score = scores.get("Alice");  // O(1) 查找！
```

### HashMap 的核心特点

```
┌─────────────────────────────────────────────────────┐
│              HashMap 特点                            │
├─────────────────────────────────────────────────────┤
│                                                     │
│  核心特性                                            │
│  ├── 键值对存储（Key-Value）                        │
│  ├── O(1) 平均时间复杂度查找/插入/删除              │
│  ├── 键必须唯一（自动去重）                         │
│  └── 无序存储（不保证插入顺序）                     │
│                                                     │
│  与 Vec/数组对比                                     │
│  ┌────────────┬────────────┬─────────────┐          │
│  │ 操作       │ Vec/数组   │ HashMap     │          │
│  ├────────────┼────────────┼─────────────┤          │
│  │ 按索引访问 │ O(1)       │ N/A         │          │
│  │ 按键查找   │ O(n)       │ O(1)        │          │
│  │ 插入       │ O(1)*      │ O(1)        │          │
│  │ 删除       │ O(n)       │ O(1)        │          │
│  └────────────┴────────────┴─────────────┘          │
│  * Vec 末尾插入 O(1)，中间插入 O(n)                  │
│                                                     │
│  使用场景                                            │
│  ✓ 需要通过键快速查找值                             │
│  ✓ 统计词频/数量                                    │
│  ✓ 缓存系统                                         │
│  ✓ 建立映射关系                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### HashMap 工作原理（简化）

```
HashMap 使用哈希函数实现快速查找：

┌─────────────────────────────────────────────────────┐
│              HashMap 内部结构                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  键"apple" → 哈希函数 → 哈希值 (如 1726384)         │
│                      ↓                              │
│              映射到桶索引 (如 5)                     │
│                      ↓                              │
│  桶数组：                                            │
│  ┌───┬───┬───┬───┬───┬───┬───┐                     │
│  │ 0 │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ ...                 │
│  ├───┼───┼───┼───┼───┼───┼───┤                     │
│  │   │   │   │   │   │ → │   │                     │
│  └───┴───┴───┴───┴───┴─┬─┴───┘                     │
│                        ↓                            │
│                  ┌──────────┐                       │
│                  │ "apple"  │                       │
│                  │    5     │                       │
│                  └──────────┘                       │
│                                                     │
│  哈希冲突处理：                                      │
│  • 链地址法：同一位置存储多个元素（链表）           │
│  • 开放寻址：寻找下一个空位                         │
│  • Rust 使用： 开放寻址 + 探测                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

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

## 13.4 entry API：插入或更新的神器

### 为什么需要 entry？

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    // ❌ 冗长的写法：需要两次查找
    for word in text.split_whitespace() {
        match word_count.get(word) {
            Some(count) => {
                let new_count = count + 1;
                word_count.insert(word.to_string(), new_count);
            }
            None => {
                word_count.insert(word.to_string(), 1);
            }
        }
    }

    // ✅ 使用 entry：一次查找，简洁高效
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    println!("{:?}", word_count);
}
```

### entry API 详解

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // or_insert - 如果键不存在则插入默认值
    let score = scores.entry("Blue".to_string()).or_insert(0);
    *score += 10;  // 通过可变引用修改

    // 再次调用 or_insert，不会覆盖已存在的值
    let score = scores.entry("Blue".to_string()).or_insert(100);
    println!("Blue 分数：{}", score);  // 10（不是 100）

    println!("{:?}", scores);  // {"Blue": 10}
}
```

### entry 的其他方法

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    // or_insert_with - 使用闭包提供默认值
    let val = map.entry("key".to_string()).or_insert_with(|| {
        println!("计算默认值...");
        42
    });

    // or_default - 使用 Default trait 的默认值
    let counts: HashMap<String, i32> = HashMap::new();
    let count = counts.get("missing").unwrap_or(&i32::default());

    // and_modify - 如果键存在则修改
    map.entry("key".to_string()).and_modify(|v| *v += 1);

    // or_insert + and_modify 组合
    map.entry("count".to_string())
        .or_insert(0)
        .then(|v| *v += 1);  // 先插入 0，再加 1

    // entry 返回的枚举
    use std::collections::hash_map::Entry;

    match map.entry("test".to_string()) {
        Entry::Occupied(entry) => {
            println!("键已存在，值：{}", entry.get());
        }
        Entry::Vacant(entry) => {
            entry.insert(100);
            println!("键不存在，已插入");
        }
    }
}
```

### 实战：词频统计

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        // 清理单词：转小写，去除标点
        let cleaned: String = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if !cleaned.is_empty() {
            *counts.entry(cleaned).or_insert(0) += 1;
        }
    }

    counts
}

fn main() {
    let text = "Hello world! Hello Rust. Rust is great, Rust is fast!";

    let counts = count_words(text);

    // 按频率排序输出
    let mut items: Vec<_> = counts.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));

    println!("词频统计:");
    for (word, count) in items {
        println!("  {}: {}", word, count);
    }
}
```

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

## 13.7 HashMap 与所有权

### 键和值的所有权

```rust
use std::collections::HashMap;

fn main() {
    // String 类型的键（转移所有权）
    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    let score = 10;

    scores.insert(team_name, score);
    // team_name 和 score 已移动

    // println!("{}", team_name);  // ❌ 错误

    // 但可以使用借用类型作为键
    let team_name = String::from("Yellow");
    scores.insert(&team_name[..], 20);  // &str 类型
    // team_name 仍然可用
    println!("{}", team_name);  // ✅
}
```

### 借用键查找

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // 可以使用&String 查找
    let team = String::from("Blue");
    let score = scores.get(&team);
    println!("分数：{:?}", score);
    // team 仍然可用
    println!("队伍：{}", team);

    // 也可以使用&str 查找（更灵活）
    let score = scores.get("Blue");  // &str
    println!("分数：{:?}", score);
}
```

### 使用借用类型的键

```rust
use std::collections::HashMap;

fn main() {
    // 使用&str 作为键类型
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Blue", 10);
    map.insert("Yellow", 50);

    // 可以使用&str 查找
    let score = map.get("Blue");
    println!("分数：{:?}", score);
}
```

---

## 13.8 自定义类型作为 HashMap 的键

### 需要实现的 Trait

```
要使自定义类型可以作为 HashMap 的键，必须实现：

1. Eq + PartialEq - 用于比较键是否相等
2. Hash - 用于计算哈希值

┌─────────────────────────────────────────────────────┐
│           自定义类型作为键的要求                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  必须实现的 Trait：                                  │
│  ├── PartialEq - 判断两个值是否相等                 │
│  ├── Eq - 完全相等关系（自反、对称、传递）          │
│  └── Hash - 计算哈希值                              │
│                                                     │
│  推荐派生：                                          │
│  #[derive(Debug, Clone, PartialEq, Eq, Hash)]       │
│                                                     │
│  注意事项：                                          │
│  • 字段都必须是 Hash 的                              │
│  • 浮点数不能直接作为键（f32/f64 不实现 Hash）        │
│  • 哈希值相同的键可能冲突                          │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 使用 derive 自动派生

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let mut users = HashMap::new();

    let user1 = User {
        id: 1,
        name: String::from("Alice"),
    };

    let user2 = User {
        id: 2,
        name: String::from("Bob"),
    };

    users.insert(user1.clone(), "alice@example.com");
    users.insert(user2.clone(), "bob@example.com");

    // 使用相同的键查找
    let lookup = User {
        id: 1,
        name: String::from("Alice"),
    };

    match users.get(&lookup) {
        Some(email) => println!("找到邮箱：{}", email),
        None => println!("未找到用户"),
    }
}
```

### 手动实现 Hash

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// 手动实现 Hash
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn main() {
    let mut map = HashMap::new();

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    map.insert(p1, "点 A");
    map.insert(p2, "点 B");

    // 查找
    let lookup = Point { x: 1, y: 2 };
    println!("{:?}", map.get(&lookup));  // Some("点 A")
}
```

### 注意事项：浮点数不能作为键

```rust
use std::collections::HashMap;

fn main() {
    // ❌ 错误：f64 不实现 Hash
    // let mut map: HashMap<f64, &str> = HashMap::new();
    // map.insert(3.14, "pi");

    // ✅ 解决方案 1：使用包装类型
    use std::num::NonZeroU32;

    // ✅ 解决方案 2：转换为整数（如果适用）
    let mut map: HashMap<i64, &str> = HashMap::new();
    map.insert((3.14 * 100.0) as i64, "pi");

    // ✅ 解决方案 3：使用 BTreeMap（允许自定义比较）
    use std::collections::BTreeMap;
    let mut btree: BTreeMap<f64, &str> = BTreeMap::new();
    btree.insert(3.14, "pi");
}
```

---

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

## 13.10 完整示例

### 示例 1：通讯录管理系统

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: String,
    group: String,
}

struct AddressBook {
    contacts: HashMap<String, Contact>,
}

impl AddressBook {
    fn new() -> Self {
        AddressBook {
            contacts: HashMap::new(),
        }
    }

    fn add(&mut self, name: String, phone: String, email: String, group: String) {
        let contact = Contact {
            name: name.clone(),
            phone,
            email,
            group,
        };
        self.contacts.insert(name, contact);
        println!("✓ 已添加联系人");
    }

    fn find(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Contact> {
        self.contacts.get_mut(name)
    }

    fn remove(&mut self, name: &str) -> Option<Contact> {
        self.contacts.remove(name)
    }

    fn list_by_group(&self, group: &str) {
        println!("\n{} 组联系人:", group);
        println!("{:-<50}", "");

        for (name, contact) in &self.contacts {
            if contact.group == group {
                println!("  {}: {} - {}", name, contact.phone, contact.email);
            }
        }
    }

    fn list_all(&self) {
        if self.contacts.is_empty() {
            println!("通讯录为空");
            return;
        }

        println!("\n全部联系人:");
        println!("{:-<50}", "");

        // 按名字排序输出
        let mut names: Vec<_> = self.contacts.keys().collect();
        names.sort();

        for name in names {
            let contact = &self.contacts[name];
            println!("  {} [{}] {} - {}",
                     contact.name,
                     contact.group,
                     contact.phone,
                     contact.email);
        }
    }

    fn statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        for contact in self.contacts.values() {
            *stats.entry(contact.group.clone()).or_insert(0) += 1;
        }

        stats
    }
}

fn main() {
    let mut book = AddressBook::new();

    // 添加联系人
    book.add(
        String::from("Alice"),
        String::from("123-4567"),
        String::from("alice@example.com"),
        String::from("同事"),
    );
    book.add(
        String::from("Bob"),
        String::from("987-6543"),
        String::from("bob@example.com"),
        String::from("朋友"),
    );
    book.add(
        String::from("Charlie"),
        String::from("555-1234"),
        String::from("charlie@example.com"),
        String::from("同事"),
    );
    book.add(
        String::from("Diana"),
        String::from("444-5678"),
        String::from("diana@example.com"),
        String::from("家人"),
    );

    // 列出所有联系人
    book.list_all();

    // 按组列出
    book.list_by_group("同事");

    // 查找联系人
    if let Some(contact) = book.find("Alice") {
        println!("\n找到 Alice:");
        println!("  电话：{}", contact.phone);
        println!("  邮箱：{}", contact.email);
        println!("  组别：{}", contact.group);
    }

    // 修改联系人
    if let Some(contact) = book.find_mut("Bob") {
        contact.phone = String::from("999-9999");
        println!("\n✓ 已更新 Bob 的电话");
    }

    // 删除联系人
    if let Some(contact) = book.remove("Charlie") {
        println!("\n✓ 已删除联系人：{}", contact.name);
    }

    // 统计
    println!("\n联系人统计:");
    for (group, count) in book.statistics() {
        println!("  {}: {} 人", group, count);
    }

    // 最终列表
    book.list_all();
}
```

### 示例 2：简单的缓存系统

```rust
use std::collections::HashMap;

struct Cache<K, V> {
    data: HashMap<K, V>,
    capacity: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
    fn new(capacity: usize) -> Self {
        Cache {
            data: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        self.data.get(key).cloned()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // 如果缓存已满且是新键，删除最旧的键
        if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
            // 简单实现：删除第一个键
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value)
    }

    fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.keys()
    }
}

fn main() {
    let mut cache = Cache::new(3);

    // 添加数据
    cache.insert("url1", "data1");
    cache.insert("url2", "data2");
    cache.insert("url3", "data3");

    println!("缓存内容:");
    for key in cache.keys() {
        if let Some(value) = cache.get(key) {
            println!("  {}: {}", key, value);
        }
    }

    // 添加新数据（应该淘汰最旧的）
    cache.insert("url4", "data4");

    println!("\n添加 url4 后:");
    for key in cache.keys() {
        if let Some(value) = cache.get(key) {
            println!("  {}: {}", key, value);
        }
    }

    // 检查缓存命中
    println!("\n缓存命中率测试:");
    println!("url1 在缓存中：{}", cache.contains(&"url1"));
    println!("url2 在缓存中：{}", cache.contains(&"url2"));
}
```

### 示例 3：图的邻接表表示

```rust
use std::collections::{HashMap, HashSet};

type Node = String;
type Graph = HashMap<Node, Vec<Node>>;

fn create_graph() -> Graph {
    let mut graph = HashMap::new();

    graph.insert(String::from("A"), vec![String::from("B"), String::from("C")]);
    graph.insert(String::from("B"), vec![String::from("D"), String::from("E")]);
    graph.insert(String::from("C"), vec![String::from("F")]);
    graph.insert(String::from("D"), vec![]);
    graph.insert(String::from("E"), vec![String::from("F")]);
    graph.insert(String::from("F"), vec![]);

    graph
}

// 广度优先搜索
fn bfs(graph: &Graph, start: &str, end: &str) -> Option<Vec<String>> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    queue.push_back(vec![start.to_string()]);

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(path) = queue.pop_front() {
        let node = path.last().unwrap();

        if node == end {
            return Some(path);
        }

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back(new_path);
                }
            }
        }
    }

    None
}

// 深度优先搜索
fn dfs(graph: &Graph, start: &str, end: &str) -> Option<Vec<String>> {
    fn dfs_helper(
        graph: &Graph,
        current: &str,
        end: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(current.to_string());
        path.push(current.to_string());

        if current == end {
            return Some(path.clone());
        }

        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(result) = dfs_helper(graph, neighbor, end, visited, path) {
                        return Some(result);
                    }
                }
            }
        }

        path.pop();
        None
    }

    let mut visited = HashSet::new();
    let mut path = Vec::new();
    dfs_helper(graph, start, end, &mut visited, &mut path)
}

fn main() {
    let graph = create_graph();

    println!("图结构:");
    for (node, neighbors) in &graph {
        println!("  {} -> {:?}", node, neighbors);
    }

    // BFS 查找路径
    if let Some(path) = bfs(&graph, "A", "F") {
        println!("\nBFS 路径：{}", path.join(" -> "));
    } else {
        println!("\n未找到路径");
    }

    // DFS 查找路径
    if let Some(path) = dfs(&graph, "A", "F") {
        println!("DFS 路径：{}", path.join(" -> "));
    }
}
```

---

## 13.11 性能优化技巧

### 预分配容量

```rust
use std::collections::HashMap;

// ❌ 低效：可能多次重新分配
fn inefficient() -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert(i, format!("value{}", i));
    }
    map
}

// ✅ 高效：预分配容量
fn efficient() -> HashMap<i32, String> {
    let mut map = HashMap::with_capacity(1000);
    for i in 0..1000 {
        map.insert(i, format!("value{}", i));
    }
    map
}
```

### 使用 entry 避免多次查找

```rust
use std::collections::HashMap;

// ❌ 低效：两次查找
fn inefficient_count(map: &mut HashMap<String, usize>, word: &str) {
    if map.contains_key(word) {
        let count = map.get(word).unwrap();
        map.insert(word.to_string(), count + 1);
    } else {
        map.insert(word.to_string(), 1);
    }
}

// ✅ 高效：一次查找
fn efficient_count(map: &mut HashMap<String, usize>, word: &str) {
    *map.entry(word.to_string()).or_insert(0) += 1;
}
```

### 避免不必要的克隆

```rust
use std::collections::HashMap;

// ❌ 不必要的克隆
fn unnecessary_clone(map: &HashMap<String, i32>, key: &str) -> Option<i32> {
    if map.contains_key(key) {
        map.get(key).cloned()  // 这里可以优化
    } else {
        None
    }
}

// ✅ 直接使用引用
fn use_reference(map: &HashMap<String, i32>, key: &str) -> Option<&i32> {
    map.get(key)  // 返回引用，无需克隆
}
```

---

## 13.12 常见错误

### 错误 1：键类型不匹配

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), 1);

    // ❌ 错误：类型不匹配
    // let val = map.get("key");  // &str vs &String

    // ✅ 正确
    let val = map.get(&String::from("key"));
    let val = map.get("key");  // HashMap 支持 Borrow  trait
}
```

### 错误 2：迭代时修改 HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    // ❌ 错误：迭代时不能修改
    // for (k, v) in &map {
    //     map.insert(*k, *v + 1);
    // }

    // ✅ 正确：先收集再更新
    let updates: Vec<_> = map
        .iter()
        .map(|(&k, &v)| (k, v + 1))
        .collect();

    for (k, v) in updates {
        map.insert(k, v);
    }
}
```

### 错误 3：忘记 entry 返回的是引用

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // ❌ 错误：尝试直接赋值
    // scores.entry("Blue").or_insert(0) = 10;

    // ✅ 正确：通过解引用修改
    *scores.entry("Blue".to_string()).or_insert(0) = 10;

    // ✅ 或者累加
    *scores.entry("Blue".to_string()).or_insert(0) += 10;
}
```

### 错误 4：依赖遍历顺序

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    // ❌ 错误：HashMap 不保证顺序
    // 不要依赖这个顺序
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    // ✅ 正确：如果需要顺序，使用 BTreeMap
    use std::collections::BTreeMap;
    let mut sorted = BTreeMap::new();
    sorted.insert(1, "one");
    sorted.insert(2, "two");
    sorted.insert(3, "three");

    for (k, v) in &sorted {
        println!("{}: {}", k, v);  // 保证按键排序
    }
}
```

---

## 13.13 调试技巧

### 打印 HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    // 基本输出
    println!("{:?}", map);

    // 美化格式
    println!("{:#?}", map);
}
```

### 调试容量和负载

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);

    println!("初始容量：{}", map.capacity());

    for i in 0..20 {
        map.insert(i, i * 2);
        println!("插入 {} 后：len={}, cap={}", i, map.len(), map.capacity());
    }
}
```

---

## 13.14 练习

### 练习 1：字符频率统计

编写函数，统计字符串中每个字符出现的次数：
```rust
fn char_frequency(text: &str) -> HashMap<char, usize> {
    // 实现
}
```

### 练习 2：两数之和

给定一个整数数组和一个目标值，找出两个数的索引，使它们的和等于目标值：
```rust
fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    // 使用 HashMap 实现 O(n) 解法
}
```

### 练习 3：分组聚合

给定一个 (姓名，分数) 的列表，计算每个人的平均分数：
```rust
fn average_scores(scores: Vec<(&str, i32)>) -> HashMap<String, f64> {
    // 实现
}
```

### 练习 4：LRU 缓存

实现一个简单的 LRU（最近最少使用）缓存：
- `get(key)` - 获取值，并将其标记为最近使用
- `put(key, value)` - 插入值，如果超出容量则删除最久未使用的

---

## 13.15 小结

### HashMap 核心概念

| 概念 | 说明 |
|------|------|
| 键值对 | Key-Value 存储 |
| 哈希函数 | 将键映射到桶索引 |
| O(1) 查找 | 平均时间复杂度 |
| 无序 | 不保证遍历顺序 |
| 键唯一 | 自动覆盖已存在的键 |

### 常用方法速查

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `insert(k, v)` | 插入键值对 | `Option<V>` |
| `get(&k)` | 获取值 | `Option<&V>` |
| `get_mut(&k)` | 获取可变引用 | `Option<&mut V>` |
| `remove(&k)` | 删除键值对 | `Option<V>` |
| `contains_key(&k)` | 检查键存在 | `bool` |
| `entry(k)` | entry API | `Entry` |
| `keys()` | 键迭代器 | `Keys` |
| `values()` | 值迭代器 | `Values` |
| `retain(\|k, v\| ...)` | 保留满足条件的元素 | - |

### entry API 模式

```rust
// 模式 1：计数
*map.entry(key).or_insert(0) += 1;

// 模式 2：构建列表
map.entry(key).or_insert_with(Vec::new).push(value);

// 模式 3：条件更新
map.entry(key).and_modify(|v| *v = new_value);
```

### 关键要点

1. **HashMap 无序**：需要有序时使用 BTreeMap
2. **键必须实现 Hash + Eq**：自定义类型需要派生这些 trait
3. **entry API 高效**：避免多次查找
4. **借用灵活**：可以使用不同类型的键进行查找
5. **预分配容量**：已知大小时优化性能

---

## 下一章

[第 14 章：错误处理](14-错误处理.md)
