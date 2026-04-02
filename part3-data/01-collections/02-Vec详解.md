## 12.3 Vec（可增长向量）详解

### 什么是 Vec？

```
Vec（Vector）是 Rust 标准库提供的可增长数组：

┌─────────────────────────────────────────┐
│           Vec 的内部结构                 │
├─────────────────────────────────────────┤
│                                         │
│  栈（Stack）                            │
│  ┌───────────┬───────────┬─────────┐   │
│  │   指针    │   长度    │  容量   │   │
│  │ (指向堆)  │  (元素数) │ (空间)  │   │
│  └─────┬─────┴─────┬─────┴────┬────┘   │
│        │           │          │        │
│        ▼           │          │        │
│  堆（Heap）         │          │        │
│  ┌─────────────────┘          │        │
│  │ ┌─────┬─────┬─────┬─────┐  │        │
│  │ │  1  │  2  │  3  │  4  │  │        │
│  │ └─────┴─────┴─────┴─────┘  │        │
│  │ ←─── 已使用 (len=4) ───→   │        │
│  │ ←────── 容量 (cap=4) ─────→│        │
│  └────────────────────────────┘        │
│                                         │
│  当 push 超过容量时：                    │
│  1. 分配更大的新内存（通常 2 倍）          │
│  2. 复制所有元素到新内存                │
│  3. 释放旧内存                          │
│                                         │
└─────────────────────────────────────────┘
```

### 创建 Vec 的多种方式

```rust
fn main() {
    // 方式 1：使用 Vec::new()
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);

    // 方式 2：使用 vec! 宏（最常用）
    let v2 = vec![1, 2, 3, 4, 5];
    let v3: Vec<i32> = vec![];  // 空 Vec

    // 方式 3：预分配容量（性能优化）
    let mut v4: Vec<i32> = Vec::with_capacity(100);
    println!("容量：{}", v4.capacity());  // 100
    for i in 0..50 {
        v4.push(i);  // 不会重新分配
    }
    println!("长度：{}, 容量：{}", v4.len(), v4.capacity());

    // 方式 4：从迭代器创建
    let v5: Vec<i32> = (1..=5).collect();
    println!("从范围创建：{:?}", v5);  // [1, 2, 3, 4, 5]

    // 方式 5：重复元素
    let v6 = vec![0; 10];  // [0, 0, 0, ...]（10 个 0）
    let v7 = vec![1, 2, 3].repeat(3);  // [1, 2, 3, 1, 2, 3, 1, 2, 3]

    // 方式 6：类型推断
    let v8 = Vec::new();  // Vec<_>，需要类型标注
    let v9: Vec<String> = Vec::new();  // 明确类型
}
```

### Vec 的容量管理

```rust
fn main() {
    let mut v = Vec::with_capacity(4);
    println!("初始 - len: {}, capacity: {}", v.len(), v.capacity());

    v.push(1);
    println!("push 1 个 - len: {}, capacity: {}", v.len(), v.capacity());

    v.push(2);
    v.push(3);
    v.push(4);
    println!("push 4 个 - len: {}, capacity: {}", v.len(), v.capacity());

    // 超过容量，触发重新分配
    v.push(5);
    println!("push 5 个 - len: {}, capacity: {}", v.len(), v.capacity());
    // 容量从 4 增长到 8（或其他值，取决于实现）

    // 手动调整容量
    v.reserve(20);  // 确保至少有 20 的额外容量
    println!("reserve 后 - capacity: {}", v.capacity());

    v.shrink_to_fit();  // 减少容量到实际长度
    println!("shrink 后 - len: {}, capacity: {}", v.len(), v.capacity());
}
```

### 添加元素

```rust
fn main() {
    let mut numbers = Vec::new();

    // push - 在末尾添加单个元素
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("push 后：{:?}", numbers);  // [1, 2, 3]

    // insert - 在指定位置插入
    numbers.insert(1, 99);  // 在索引 1 插入 99
    println!("insert 后：{:?}", numbers);  // [1, 99, 2, 3]

    // extend - 批量添加多个元素
    numbers.extend([4, 5, 6]);
    println!("extend 后：{:?}", numbers);  // [1, 99, 2, 3, 4, 5, 6]

    // 从另一个 Vec 添加
    let more = vec![7, 8, 9];
    numbers.extend(more);
    println!("extend Vec 后：{:?}", numbers);
}
```

### 访问和修改元素

```rust
fn main() {
    let mut numbers = vec![10, 20, 30, 40, 50];

    // 索引访问
    let third = numbers[2];  // 30
    println!("第三个元素：{}", third);

    // 索引修改
    numbers[0] = 100;
    println!("修改后：{:?}", numbers);  // [100, 20, 30, 40, 50]

    // 安全访问（返回 Option）
    match numbers.get(2) {
        Some(value) => println!("索引 2: {}", value),
        None => println!("索引不存在"),
    }

    // 获取首尾元素
    println!("第一个：{:?}", numbers.first());  // Some(100)
    println!("最后一个：{:?}", numbers.last());  // Some(50)

    // 获取可变引用
    if let Some(first) = numbers.first_mut() {
        *first = 999;
    }
    println!("修改第一个后：{:?}", numbers);  // [999, 20, 30, 40, 50]
}
```

### 移除元素

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // pop - 移除并返回最后一个元素
    let last = numbers.pop();
    println!("pop 返回：{:?}", last);  // Some(10)
    println!("pop 后：{:?}", numbers);  // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    // remove - 移除指定位置的元素
    let removed = numbers.remove(0);
    println!("remove 返回：{}", removed);  // 1
    println!("remove 后：{:?}", numbers);  // [2, 3, 4, 5, 6, 7, 8, 9]

    // swap_remove - 移除并交换最后一个元素（O(1)，不保持顺序）
    let v = vec![1, 2, 3, 4, 5];
    let removed = v.swap_remove(1);  // 移除 2，用 5 填补
    println!("swap_remove: {:?}, 移除：{}", v, removed);  // [1, 5, 3, 4]

    // truncate - 截断到指定长度
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(2);
    println!("truncate(2): {:?}", v);  // [1, 2]

    // clear - 清空所有元素
    numbers.clear();
    println!("clear 后：{:?}", numbers);  // []
    println!("clear 后长度：{}", numbers.len());  // 0
}
```






---

## 12.4 Vec 常用方法大全

```rust
fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // ========== 查询方法 ==========
    println!("长度：{}", v.len());           // 8
    println!("容量：{}", v.capacity());      // >= 8
    println!("是否为空：{}", v.is_empty());  // false
    println!("包含 5: {}", v.contains(&5));  // true
    println!("第一个：{:?}", v.first());     // Some(3)
    println!("最后一个：{:?}", v.last());    // Some(6)

    // ========== 修改方法 ==========
    v.push(7);              // 末尾添加
    v.pop();                // 移除末尾
    v.insert(0, 0);         // 索引 0 插入 0
    v.remove(0);            // 移除索引 0
    v[0] = 10;              // 修改索引 0

    // ========== 批量操作 ==========
    v.extend([11, 12, 13]);           // 批量添加
    v.truncate(5);                    // 截断
    v.clear();                        // 清空

    // ========== 排序 ==========
    v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort();                         // 原地排序
    println!("排序后：{:?}", v);       // [1, 1, 2, 3, 4, 5, 6, 9]

    v.sort_by(|a, b| b.cmp(a));       // 降序排序
    println!("降序：{:?}", v);         // [9, 6, 5, 4, 3, 2, 1, 1]

    v.reverse();                      // 反转
    println!("反转：{:?}", v);         // [1, 1, 2, 3, 4, 5, 6, 9]

    // ========== 查找 ==========
    println!("3 的索引：{:?}", v.iter().position(|&x| x == 3));  // Some(3)
    println!("是否有偶数：{}", v.iter().any(|&x| x % 2 == 0));   // true
    println!("是否全为正：{}", v.iter().all(|&x| x > 0));        // true

    // ========== 去重 ==========
    v.sort();
    v.dedup();  // 移除连续重复元素
    println!("去重后：{:?}", v);  // [1, 2, 3, 4, 5, 6, 9]

    // ========== 迭代 ==========
    for (i, val) in v.iter().enumerate() {
        println!("v[{}] = {}", i, val);
    }
}
```




