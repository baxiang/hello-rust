## 12.2 数组（Array）详解

### 创建数组的五种方式

```rust
fn main() {
    // 方式 1：直接列出所有元素
    let numbers = [1, 2, 3, 4, 5];

    // 方式 2：显式类型标注
    let explicit: [i32; 5] = [1, 2, 3, 4, 5];

    // 方式 3：重复元素（非常实用）
    let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    let ones = [1; 5];    // [1, 1, 1, 1, 1]

    // 方式 4：空数组
    let empty: [i32; 0] = [];
    let empty2: [bool; 0] = [];

    // 方式 5：从表达式创建
    let computed = [1 + 2, 3 * 4, 5];  // [3, 12, 5]

    println!("numbers: {:?}", numbers);
    println!("zeros: {:?}", zeros);
    println!("empty: {:?}", empty);
}
```

### 数组的内存布局

```
let arr = [10, 20, 30, 40, 50];

内存布局（连续存储）：
栈（Stack）
┌─────┬─────┬─────┬─────┬─────┐
│ 10  │ 20  │ 30  │ 40  │ 50  │
└─────┴─────┴─────┴─────┴─────┘
  ↑     ↑     ↑     ↑     ↑
  0     1     2     3     4    ← 索引

特点：
• 元素连续存储，访问高效
• 通过索引直接计算偏移量：base + index * sizeof(T)
• 缓存友好，遍历时预取效果好
```

### 访问数组元素

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    // 方式 1：索引访问（使用 []）
    let first = numbers[0];   // 10
    let third = numbers[2];   // 30
    let last = numbers[4];    // 50

    // 动态索引
    let index = 3;
    let value = numbers[index];  // 40

    // 反向索引（Rust 1.77+）
    let last = numbers[5..][0];  // 50（切片后取第一个）

    println!("first: {}, third: {}, last: {}", first, third, last);

    // 方式 2：安全访问（使用 get 方法）
    match numbers.get(2) {
        Some(value) => println!("索引 2 的值：{}", value),
        None => println!("索引超出范围"),
    }

    // 越界访问会返回 None，不会 panic
    match numbers.get(10) {
        Some(value) => println!("{}", value),
        None => println!("索引 10 超出范围！"),  // 会执行这里
    }
}
```

### 数组越界的危险

```rust
fn main() {
    let numbers = [1, 2, 3];

    // ❌ 危险：直接索引越界会 panic
    // let value = numbers[10];  // 程序崩溃！

    // ✅ 安全：使用 get 方法
    if let Some(value) = numbers.get(10) {
        println!("{}", value);
    } else {
        println!("安全处理越界情况");
    }
}

// panic 输出：
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
```

### 数组常用方法

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 获取长度
    println!("长度：{}", numbers.len());  // 5

    // 检查是否为空
    println!("是否为空：{}", numbers.is_empty());  // false

    // 检查是否包含某元素
    println!("包含 3: {}", numbers.contains(&3));  // true
    println!("包含 10: {}", numbers.contains(&10));  // false

    // 获取首尾元素（返回 Option）
    println!("第一个：{:?}", numbers.first());  // Some(1)
    println!("最后一个：{:?}", numbers.last());  // Some(5)

    let empty: [i32; 0] = [];
    println!("空数组的第一个：{:?}", empty.first());  // None
}
```

### 数组切片（Slice）

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 创建切片（借用部分数组）
    let slice: &[i32] = &numbers[2..5];  // [3, 4, 5]

    println!("原数组：{:?}", numbers);
    println!("切片 [2..5]: {:?}", slice);

    // 切片的各种形式
    let all = &numbers[..];        // 整个数组 [1..10]
    let first_three = &numbers[..3];  // [1, 2, 3]
    let last_three = &numbers[7..];   // [8, 9, 10]
    let single = &numbers[4..5];      // [5]（切片，不是值）

    println!("全部：{:?}", all);
    println!("前三个：{:?}", first_three);
    println!("后三个：{:?}", last_three);

    // 切片的长度
    println!("切片长度：{}", slice.len());  // 3
}
```

### 数组迭代

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    // 方式 1：for in 迭代（借用）
    println!("迭代（借用）:");
    for num in &numbers {
        println!("{}", num);
    }
    // numbers 仍然可用
    println!("原数组：{:?}", numbers);

    // 方式 2：迭代并修改
    let mut mutable = [1, 2, 3, 4, 5];
    for num in &mut mutable {
        *num *= 2;
    }
    println!("翻倍后：{:?}", mutable);  // [2, 4, 6, 8, 10]

    // 方式 3：带索引迭代
    for (index, value) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", index, value);
    }

    // 方式 4：获取所有权（适用于可 Copy 类型）
    for num in numbers {
        println!("{}", num);
    }
}
```

---

---

## 下一步

[Vec（可增长向量）详解](../3-Vec（可增长向量）详解.md)