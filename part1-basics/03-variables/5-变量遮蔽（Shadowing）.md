## 03.5 变量遮蔽（Shadowing）

### 基本概念

```rust
fn main() {
    let x = 5;
    let x = x + 1;  // 遮蔽之前的 x
    let x = x * 2;  // 再次遮蔽

    println!("x = {}", x);  // 输出 12
}
```

### 遮蔽 vs 可变

```rust
fn main() {
    // 使用 mut
    let mut y = 5;
    y = y + 1;  // 修改原值
    y = y * 2;
    println!("y = {}", y);  // 12

    // 使用 shadowing
    let z = 5;
    let z = z + 1;  // 新变量
    let z = z * 2;  // 新变量
    println!("z = {}", z);  // 12
}
```

### 遮蔽的用途

```rust
fn main() {
    // 1. 类型转换
    let spaces = "   ";
    let spaces = spaces.len();  // &str -> usize

    // 2. 复用变量名
    let numbers = vec![1, 2, 3];
    let numbers = numbers.iter().sum::<i32>();  // Vec -> i32

    // 3. 条件转换
    let input = "42";
    let input: i32 = match input.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
}
```

---

---

## 下一步

[类型推断](../6-类型推断.md)