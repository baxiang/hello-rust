## 4.7 数组（Array）

### 创建数组

```rust
fn main() {
    // 类型推断
    let numbers = [1, 2, 3, 4, 5];

    // 显式类型标注
    let floats: [f64; 4] = [1.0, 2.0, 3.0, 4.0];

    // 重复元素（语法糖）
    let zeros = [0; 5];      // [0, 0, 0, 0, 0]
    let ones = [1; 10];      // [1, 1, ..., 1] (10 个)
    let names = ["Alice"; 3]; // ["Alice", "Alice", "Alice"]

    println!("numbers = {:?}", numbers);
    println!("zeros = {:?}", zeros);
    println!("ones = {:?}", ones);
    println!("names = {:?}", names);

    // 数组长度（编译时已知）
    println!("numbers 长度：{}", numbers.len());
    println!("numbers 内存大小：{} 字节", std::mem::size_of_val(&numbers));
}
```

### 访问数组元素

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    // 索引访问（从 0 开始）
    let first = numbers[0];
    let middle = numbers[2];
    let last = numbers[4];

    println!("第一个：{}", first);
    println!("中间：{}", middle);
    println!("最后一个：{}", last);

    // 安全访问（使用 get 方法）
    match numbers.get(2) {
        Some(value) => println!("索引 2 的值：{}", value),
        None => println!("索引不存在"),
    }

    // 越界访问返回 None（而不是 panic）
    match numbers.get(10) {
        Some(value) => println!("{}", value),
        None => println!("索引 10 超出范围"),
    }

    // 修改数组元素
    let mut mutable = [1, 2, 3];
    mutable[1] = 99;
    println!("修改后：{:?}", mutable);
}
```

### 数组迭代

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 方法 1：直接迭代（不可变引用）
    println!("迭代（引用）:");
    for n in &numbers {
        println!("  {}", n);
    }

    // 方法 2：带索引迭代
    println!("\n带索引：");
    for (index, value) in numbers.iter().enumerate() {
        println!("  numbers[{}] = {}", index, value);
    }

    // 方法 3：可变迭代
    let mut scores = [80, 90, 70, 85];
    for score in &mut scores {
        *score += 5;  // 每人加 5 分
    }
    println!("\n加分后：{:?}", scores);
}
```

### 数组 vs 列表

```rust
fn main() {
    // 数组特点：
    // 1. 长度固定（编译时确定）
    // 2. 存储在栈上
    // 3. 类型：[T; N]

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // 数组不能动态增长
    // arr.push(6);  // ❌ 错误！数组没有 push 方法

    // 需要动态数组时使用 Vec（第 12 章讲解）
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("Vec 可以增长：{:?}", vec);
}
```

---

---

## 下一步

[String 和 &str（重点难点）](../8-String 和 &str（重点难点）.md)