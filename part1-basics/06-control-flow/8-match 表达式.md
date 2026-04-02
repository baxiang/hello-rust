## 6.8 match 表达式

### 基础 match

```rust
fn main() {
    let number = 42;

    match number {
        0 => println!("零"),
        1..=9 => println!("个位数"),
        10..=99 => println!("十位数"),
        100..=999 => println!("百位数"),
        _ => println!("其他"),  // _ 是通配符
    }
}
```

### match 返回值

```rust
fn main() {
    let number = 5;

    let description = match number {
        0 => "零",
        n if n > 0 && n < 10 => "小的正数",
        10..=99 => "两位数",
        _ => "很大的数",
    };

    println!("{}", description);
}
```

### 匹配 Option 和 Result

```rust
fn main() {
    // 匹配 Option
    let maybe_number: Option<i32> = Some(42);

    match maybe_number {
        Some(n) => println!("数字是：{}", n),
        None => println!("没有数字"),
    }

    // 匹配 Result
    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(value) => println!("成功：{}", value),
        Err(error) => println!("错误：{}", error),
    }
}
```




