## 6.5 for 循环（最常用）

### 遍历集合

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 遍历数组（推荐方式）
    for number in numbers.iter() {
        println!("{}", number);
    }

    // 简写（自动解引用）
    for &number in &numbers {
        println!("{}", number);
    }

    // 可变遍历
    let mut mutable_numbers = [1, 2, 3];
    for num in &mut mutable_numbers {
        *num *= 2;
    }
    println!("{:?}", mutable_numbers);  // [2, 4, 6]
}
```

### 范围（Range）

```rust
fn main() {
    // 不包含结束值：1, 2, 3
    for i in 1..4 {
        print!("{} ", i);
    }
    println!();

    // 包含结束值：1, 2, 3, 4
    for i in 1..=4 {
        print!("{} ", i);
    }
    println!();

    // 倒序：3, 2, 1
    for i in (1..4).rev() {
        print!("{} ", i);
    }
    println!();

    // 步长（需要使用 step_by）
    for i in (1..=10).step_by(2) {
        print!("{} ", i);  // 1 3 5 7 9
    }
    println!();
}
```

### enumerate - 带索引遍历

```rust
fn main() {
    let names = ["Alice", "Bob", "Charlie"];

    // 方法 1：手动跟踪索引（不推荐）
    let mut index = 0;
    for name in &names {
        println!("{}: {}", index, name);
        index += 1;
    }

    // 方法 2：使用 enumerate（推荐）
    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name);
    }
}
```

**输出：**
```
0: Alice
1: Bob
2: Charlie
```

### zip - 同时遍历两个集合

```rust
fn main() {
    let names = ["Alice", "Bob", "Charlie"];
    let ages = [25, 30, 35];

    // 同时遍历两个数组
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} 是 {} 岁", name, age);
    }

    // 长度不同时，以短的为准
    let short = [1, 2];
    let long = [1, 2, 3, 4, 5];
    for (a, b) in short.iter().zip(long.iter()) {
        println!("{} - {}", a, b);  // 只输出 2 对
    }
}
```

---

---

## 下一步

[loop 循环](../6-loop 循环.md)