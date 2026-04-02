## 11.6 if let 和 while let

### if let 简洁匹配

```rust
fn main() {
    let config_max = Some(3u8);

    // 使用 if let（简洁）
    if let Some(max) = config_max {
        println!("最大值：{}", max);
    }

    // 等同于 match（冗长）
    match config_max {
        Some(max) => println!("最大值：{}", max),
        _ => {}
    }
}
```

### if let else

```rust
fn main() {
    let coin = Some(5u8);

    if let Some(value) = coin {
        println!("值是：{}", value);
    } else {
        println!("没有值");
    }

    // 链式 if let
    let x = Some(5);

    if let Some(n) = x {
        if n > 0 {
            println!("正数");
        }
    }
}
```

### while let 循环

```rust
fn main() {
    // 弹出栈直到空
    let mut stack = vec![1, 2, 3, 4, 5];

    while let Some(top) = stack.pop() {
        println!("弹出：{}", top);
    }

    // 迭代器
    let mut iter = vec![1, 2, 3].into_iter();

    while let Some(value) = iter.next() {
        println!("值：{}", value);
    }
}
```

---

---

## 下一步

[为枚举定义方法](../7-为枚举定义方法.md)