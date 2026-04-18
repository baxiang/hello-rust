# HashMap操作

> 掌握HashMap的entry API、插入更新模式和常用操作方法。

## entry API：插入或更新的神器

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

## 小结

- entry API：一次查找完成"不存在则插入，存在则更新"
- `or_insert(v)`：不存在时插入默认值，返回可变引用
- `or_insert_with(|| v)`：使用闭包延迟计算默认值
- 常用模式：计数 `*entry.or_insert(0) += 1`

## 练习题

详见：[练习题](../../exercises/02-hashmap.md)
```




