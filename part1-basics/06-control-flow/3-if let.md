## 6.3 if let

### 简化单一模式匹配

```rust
fn main() {
    let config_max = Some(3u8);

    // 使用 match（冗长）
    match config_max {
        Some(max) => println!("最大值：{}", max),
        _ => println!("没有最大值"),
    }

    // 使用 if let（简洁）
    if let Some(max) = config_max {
        println!("最大值：{}", max);
    }

    // 带 else
    let coin = Some(5u8);

    if let Some(value) = coin {
        println!("面值是：{}", value);
    } else {
        println!("这不是硬币");
    }
}
```

### 实际应用场景

```rust
fn main() {
    // 场景 1：处理 Option
    let name: Option<String> = Some("Alice".to_string());

    if let Some(n) = name {
        println!("名字是：{}", n);
    }

    // 场景 2：处理 Result
    let result: Result<i32, &str> = Ok(42);

    if let Ok(value) = result {
        println!("成功：{}", value);
    }

    // 场景 3：带条件的 if let
    let numbers = vec![1, 2, 3];
    let first = numbers.first();

    if let Some(&first_num) = first {
        if first_num > 0 {
            println!("第一个数是正数：{}", first_num);
        }
    }
}
```

---

---

## 下一步

[while 循环](../4-while 循环.md)