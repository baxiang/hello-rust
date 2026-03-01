# 第 12 章：数组与 Vec 详解

> 存储和管理多个相同类型的值——集合类型的基石

---

## 12.1 为什么需要集合类型？

### 问题场景

```rust
// 问题：存储多个学生的分数
// 方案 1：使用独立变量（糟糕）
let score1 = 90;
let score2 = 85;
let score3 = 78;
let score4 = 92;
// ...如果有 100 个学生怎么办？

// 方案 2：使用数组或 Vec（优秀）
let scores = [90, 85, 78, 92];  // 固定数量
let mut scores_vec = vec![90, 85, 78, 92];  // 可增长
```

### 数组与 Vec 的对比

```
┌─────────────────────────────────────────────────────┐
│              数组 vs Vec 对比                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  数组 (Array)                                       │
│  ├── 固定长度（编译时确定）                         │
│  ├── 存储在栈上（通常更快）                         │
│  ├── 类型：[T; N]（T 是元素类型，N 是长度）            │
│  └── 适用：元素数量固定的场景                       │
│                                                     │
│  Vec（可增长向量）                                   │
│  ├── 可变长度（运行时动态调整）                     │
│  ├── 数据存储在堆上                                 │
│  ├── 类型：Vec<T>                                   │
│  └── 适用：元素数量变化的场景                       │
│                                                     │
│  共同点：                                            │
│  • 所有元素必须是相同类型                           │
│  • 索引从 0 开始                                     │
│  • 提供迭代器支持                                   │
│  • 边界检查（安全）                                 │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

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

---

## 12.5 数组与 Vec 的转换

### 数组转 Vec

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    // 方法 1：to_vec()（推荐，最简洁）
    let v1 = arr.to_vec();

    // 方法 2：Vec::from()
    let v2 = Vec::from(arr);

    // 方法 3：into_iter().collect()
    let v3: Vec<i32> = arr.into_iter().collect();

    // 方法 4：iter().copied().collect()
    let v4: Vec<i32> = arr.iter().copied().collect();

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    // 对于非 Copy 类型（如 String）
    let str_arr = [String::from("a"), String::from("b")];
    let str_vec: Vec<String> = str_arr.into_iter().collect();
    // 注意：into_iter() 会转移所有权，str_arr 不再可用
}
```

### Vec 转数组

```rust
fn main() {
    // 方法 1：try_into()（Rust 1.48+）
    let v = vec![1, 2, 3, 4, 5];
    let arr: Result<[i32; 5], _> = v.clone().try_into();

    match arr {
        Ok(a) => println!("成功：{:?}", a),
        Err(e) => println!("失败：{:?}", e),
    }

    // 方法 2：try_into().unwrap()
    let v2 = vec![1, 2, 3];
    let arr2: [i32; 3] = v2.try_into().unwrap();
    println!("arr2: {:?}", arr2);

    // 方法 3：使用 slice 和 copy_from_slice（适用于可 Copy 类型）
    let v3 = vec![10, 20, 30];
    let mut arr3 = [0; 3];
    arr3.copy_from_slice(&v3);
    println!("arr3: {:?}", arr3);

    // 注意：长度必须匹配
    let v4 = vec![1, 2, 3];
    // let arr4: [i32; 5] = v4.try_into().unwrap();  // panic!
}
```

### 切片与数组/Vev 的关系

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];

    // 数组和 Vec 都可以借用为切片
    let slice1: &[i32] = &arr;
    let slice2: &[i32] = &vec;

    // 函数接受切片参数（最灵活）
    fn process(data: &[i32]) {
        println!("{:?}", data);
    }

    process(&arr);  // ✅
    process(&vec);  // ✅
    process(&arr[1..4]);  // ✅
}
```

---

## 12.6 数组 vs Vec：选择指南

```
┌─────────────────────────────────────────────────────┐
│           数组 vs Vec 选择指南                       │
├─────────────────────────────────────────────────────┤
│                                                     │
│  使用数组的场景：                                    │
│  ✓ 元素数量在编译时已知且固定                        │
│  ✓ 需要最高性能（无堆分配）                         │
│  ✓ 小型固定集合（如 RGB 颜色、坐标）                 │
│  ✓ 作为函数参数传递切片                             │
│                                                     │
│  使用 Vec 的场景：                                   │
│  ✓ 元素数量在运行时确定                             │
│  ✓ 需要动态添加/删除元素                            │
│  ✓ 从文件/网络读取数据                              │
│  ✓ 构建结果集合（未知大小）                         │
│                                                     │
│  性能对比：                                          │
│  ┌────────────┬───────────┬───────────┐             │
│  │ 操作       │ 数组      │ Vec       │             │
│  ├────────────┼───────────┼───────────┤             │
│  │ 索引访问   │ O(1)      │ O(1)      │             │
│  │ 创建       │ O(1)      │ O(1)      │             │
│  │ push       │ N/A       │ O(1)*     │             │
│  │ 内存       │ 栈上      │ 堆上      │             │
│  │ 缓存局部性 │ 优秀      │ 良好      │             │
│  └────────────┴───────────┴───────────┘             │
│  * 摊还 O(1)，重新分配时 O(n)                        │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 实际代码示例

```rust
fn main() {
    // ✅ 使用数组：一周的天数（固定 7 个）
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    // ✅ 使用数组：RGB 颜色（固定 3 个分量）
    let color: [u8; 3] = [255, 128, 64];

    // ✅ 使用数组：3D 坐标（固定 3 个值）
    let position: [f64; 3] = [1.0, 2.0, 3.0];

    // ✅ 使用 Vec：用户输入的数字（数量不定）
    let mut numbers = Vec::new();
    // numbers.push(...);

    // ✅ 使用 Vec：从文件读取的行数
    let lines: Vec<String> = std::fs::read_to_string("file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // ✅ 使用 Vec：搜索结果（数量不定）
    let results: Vec<i32> = vec![1, 2, 3, 4, 5]
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
}
```

---

## 12.7 完整示例

### 示例 1：统计计算

```rust
fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // 总和
    let sum: i32 = numbers.iter().sum();

    // 平均值
    let avg = sum as f64 / numbers.len() as f64;

    // 最大值和最小值
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    // 中位数（需要排序）
    let mut sorted = numbers.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
    };

    // 众数（出现最多的元素）
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &num in &numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mode = counts.iter().max_by_key(|&(_, count)| count).unwrap().0;

    println!("数据：{:?}", numbers);
    println!("总和：{}", sum);
    println!("平均值：{:.2}", avg);
    println!("最大值：{}", max);
    println!("最小值：{}", min);
    println!("中位数：{:.2}", median);
    println!("众数：{}", mode);
}
```

### 示例 2：待办事项列表

```rust
struct TodoList {
    items: Vec<TodoItem>,
}

#[derive(Debug)]
struct TodoItem {
    description: String,
    completed: bool,
}

impl TodoItem {
    fn new(description: String) -> Self {
        TodoItem {
            description,
            completed: false,
        }
    }
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            items: Vec::new(),
        }
    }

    fn add(&mut self, description: String) {
        self.items.push(TodoItem::new(description));
        println!("已添加：{}", description);
    }

    fn complete(&mut self, index: usize) -> Result<(), String> {
        if index < self.items.len() {
            self.items[index].completed = true;
            Ok(())
        } else {
            Err(format!("索引 {} 不存在", index))
        }
    }

    fn remove(&mut self, index: usize) -> Result<TodoItem, String> {
        if index < self.items.len() {
            Ok(self.items.remove(index))
        } else {
            Err(format!("索引 {} 不存在", index))
        }
    }

    fn list(&self) {
        if self.items.is_empty() {
            println!("待办事项为空");
            return;
        }

        println!("\n待办事项列表:");
        println!("{:-<40}", "");
        for (i, item) in self.items.iter().enumerate() {
            let status = if item.completed { "✓" } else { "○" };
            println!("{}. [{}] {}", i + 1, status, item.description);
        }
        println!("{:-<40}", "");
    }

    fn pending_count(&self) -> usize {
        self.items.iter().filter(|item| !item.completed).count()
    }
}

fn main() {
    let mut todo = TodoList::new();

    todo.add(String::from("学习 Rust"));
    todo.add(String::from("写代码"));
    todo.add(String::from("跑步"));
    todo.add(String::from("读书"));

    todo.list();
    println!("待完成：{} 项", todo.pending_count());

    todo.complete(1).unwrap();
    todo.complete(3).unwrap();

    println!("\n完成两项后:");
    todo.list();

    todo.remove(2).unwrap();

    println!("\n移除一项后:");
    todo.list();
}
```

### 示例 3：矩阵运算

```rust
type Matrix = Vec<Vec<f64>>;

fn create_matrix(rows: usize, cols: usize, value: f64) -> Matrix {
    vec![vec![value; cols]; rows]
}

fn identity_matrix(size: usize) -> Matrix {
    let mut mat = create_matrix(size, size, 0.0);
    for i in 0..size {
        mat[i][i] = 1.0;
    }
    mat
}

fn print_matrix(matrix: &Matrix, name: &str) {
    println!("{}:", name);
    for row in matrix {
        print!("  [");
        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{:6.2}", val);
        }
        println!("]");
    }
    println!();
}

fn matrix_add(a: &Matrix, b: &Matrix) -> Matrix {
    a.iter()
        .zip(b.iter())
        .map(|(row_a, row_b)| {
            row_a.iter().zip(row_b.iter()).map(|(&a, &b)| a + b).collect()
        })
        .collect()
}

fn scalar_multiply(matrix: &Matrix, scalar: f64) -> Matrix {
    matrix
        .iter()
        .map(|row| row.iter().map(|&val| val * scalar).collect())
        .collect()
}

fn transpose(matrix: &Matrix) -> Matrix {
    if matrix.is_empty() {
        return Vec::new();
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| matrix[i][j]).collect())
        .collect()
}

fn main() {
    // 创建矩阵
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
    ];

    let b = vec![
        vec![7.0, 8.0, 9.0],
        vec![10.0, 11.0, 12.0],
    ];

    print_matrix(&a, "矩阵 A");
    print_matrix(&b, "矩阵 B");

    // 矩阵加法
    let sum = matrix_add(&a, &b);
    print_matrix(&sum, "A + B");

    // 标量乘法
    let scaled = scalar_multiply(&a, 2.0);
    print_matrix(&scaled, "A * 2");

    // 转置
    let transposed = transpose(&a);
    print_matrix(&transposed, "A 的转置");

    // 单位矩阵
    let identity = identity_matrix(4);
    print_matrix(&identity, "4x4 单位矩阵");
}
```

### 示例 4：简单的字符串处理器

```rust
fn main() {
    let text = String::from("Hello, Rust! This is a test.");

    // 转换为字符 Vec
    let chars: Vec<char> = text.chars().collect();
    println!("字符数：{}", chars.len());

    // 过滤元音
    let vowels: Vec<char> = chars
        .iter()
        .copied()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .collect();
    println!("元音：{:?}", vowels);

    // 转换为大写
    let upper: Vec<char> = chars
        .iter()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect();
    let upper_string: String = upper.into_iter().collect();
    println!("大写：{}", upper_string);

    // 分词
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("单词：{:?}", words);

    // 单词长度统计
    let word_lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    println!("单词长度：{:?}", word_lengths);

    // 连接字符串
    let joined = words.join("-");
    println!("连接：{}", joined);
}
```

---

## 12.8 性能优化技巧

### 预分配容量

```rust
// ❌ 低效：可能多次重新分配
fn inefficient() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
    }
    v
}

// ✅ 高效：预分配容量
fn efficient() -> Vec<i32> {
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i);
    }
    v
}

// ✅ 使用 collect（通常能推断大小）
fn best() -> Vec<i32> {
    (0..1000).collect()
}
```

### 避免不必要的复制

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // ❌ 不必要的复制
    fn process_owned(v: Vec<i32>) -> i32 {
        v.iter().sum()
    }
    let result = process_owned(data.clone());  // 复制了整个 Vec

    // ✅ 使用借用
    fn process_borrowed(v: &[i32]) -> i32 {
        v.iter().sum()
    }
    let result = process_borrowed(&data);  // 只传递引用
}
```

### 使用 drain 清空并获取元素

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // ❌ 先迭代再清空（两次遍历）
    let sum: i32 = v.iter().sum();
    v.clear();

    // ✅ 使用 drain（一次遍历并清空）
    let sum: i32 = v.drain(..).sum();
    // v 现在是空的
}
```

---

## 12.9 常见错误

### 错误 1：忘记 mut

```rust
fn main() {
    // ❌ 错误：需要 mut 才能修改
    // let v = Vec::new();
    // v.push(1);

    // ✅ 正确
    let mut v = Vec::new();
    v.push(1);

    // 数组同理
    // let arr = [1, 2, 3];
    // arr[0] = 10;  // ❌ 需要 mut

    let mut arr = [1, 2, 3];
    arr[0] = 10;  // ✅
}
```

### 错误 2：数组越界

```rust
fn main() {
    let numbers = [1, 2, 3];

    // ❌ 会导致 panic
    // let value = numbers[10];

    // ✅ 使用 get 安全访问
    match numbers.get(10) {
        Some(v) => println!("{}", v),
        None => println!("索引超出范围"),
    }

    // ✅ 或者先检查长度
    let index = 10;
    if index < numbers.len() {
        println!("{}", numbers[index]);
    } else {
        println!("索引 {} 超出范围", index);
    }
}
```

**错误信息：**
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
```

### 错误 3：迭代时修改 Vec

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // ❌ 错误：迭代时不能修改
    // for i in &v {
    //     v.push(*i);  // 借用冲突
    // }

    // ✅ 正确：收集后扩展
    let to_add: Vec<i32> = v.iter().copied().collect();
    v.extend(to_add);

    // ✅ 或使用索引
    let len = v.len();
    for i in 0..len {
        let val = v[i] * 2;
        v.push(val);
    }
}
```

**错误信息：**
```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

### 错误 4：返回局部 Vec 的切片

```rust
// ❌ 错误：返回悬垂引用
// fn get_slice() -> &[i32] {
//     let v = vec![1, 2, 3];
//     &v  // v 在函数结束时被释放
// }

// ✅ 正确：返回 Vec
fn get_vec() -> Vec<i32> {
    vec![1, 2, 3]
}

fn main() {
    let v = get_vec();
    println!("{:?}", v);
}
```

### 错误 5：误用 dedup

```rust
fn main() {
    let mut v = vec![3, 1, 2, 1, 3];

    // ❌ dedup 只移除连续的重复元素
    v.dedup();
    println!("{:?}", v);  // [3, 1, 2, 1, 3]（没有变化！）

    // ✅ 正确：先排序再去重
    let mut v = vec![3, 1, 2, 1, 3];
    v.sort();
    v.dedup();
    println!("{:?}", v);  // [1, 2, 3]
}
```

---

## 12.10 调试技巧

### 打印调试信息

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 基本调试输出
    println!("{:?}", v);

    // 美化格式
    println!("{:#?}", v);

    // 使用 dbg! 宏
    let result = dbg!(v.len()) * 2;

    // dbg! 返回值的引用，可以用于链式调用
    let v = vec![1, 2, 3];
    let sum: i32 = dbg!(v).iter().sum();
}
```

### 检查容量变化

```rust
fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(4);

    println!("初始：len={}, cap={}", v.len(), v.capacity());

    for i in 0..10 {
        v.push(i);
        println!("push({}): len={}, cap={}", i, v.len(), v.capacity());
    }
}
```

---

## 12.11 练习

### 练习 1：基本操作

创建一个 Vec，实现以下功能：
1. 添加数字 1-10
2. 计算所有偶数的和
3. 移除最后一个元素
4. 在开头插入 0
5. 打印最终结果

### 练习 2：字符串处理

编写函数，接受字符串 Vec，返回：
1. 所有字符串连接成一个大写字符串
2. 最长字符串的长度
3. 所有字符串的平均长度

### 练习 3：栈实现

使用 Vec 实现一个简单的栈：
- `push(item)` - 入栈
- `pop()` - 出栈
- `peek()` - 查看栈顶
- `is_empty()` - 判断是否为空

### 练习 4：向量运算

实现向量（Vec<f64>）的基本运算：
- `dot_product(a, b)` - 点积
- `magnitude(v)` - 模长
- `normalize(v)` - 归一化
- `add(a, b)` - 向量加法

---

## 12.12 小结

### 数组与 Vec 对比

| 特性 | 数组 | Vec |
|------|------|-----|
| 类型 | `[T; N]` | `Vec<T>` |
| 长度 | 固定（编译时） | 可变（运行时） |
| 存储 | 栈上（通常） | 堆上 |
| 大小 | `N * sizeof(T)` | `len * sizeof(T) + 开销` |
| 性能 | 稍快 | 稍慢（但更灵活） |
| 方法 | 较少 | 丰富 |

### 常用方法速查

| 方法 | 数组 | Vec | 说明 |
|------|------|-----|------|
| `len()` | ✅ | ✅ | 获取长度 |
| `is_empty()` | ✅ | ✅ | 判断为空 |
| `first()/last()` | ✅ | ✅ | 首尾元素 |
| `get(i)` | ✅ | ✅ | 安全访问 |
| `iter()/iter_mut()` | ✅ | ✅ | 迭代器 |
| `push()/pop()` | ❌ | ✅ | 末尾操作 |
| `insert()/remove()` | ❌ | ✅ | 插入/删除 |
| `extend()` | ❌ | ✅ | 批量添加 |
| `clear()` | ❌ | ✅ | 清空 |

### 关键要点

1. **数组**用于固定大小的集合，存储在栈上，性能最优
2. **Vec**用于动态集合，可自动增长，使用最广泛
3. 索引访问会进行边界检查，越界会 panic
4. 使用 `get()` 方法可以安全访问，返回 `Option`
5. Vec 扩容时会重新分配内存，预分配容量可优化性能
6. 函数参数优先使用切片 `&[T]`，可同时接受数组和 Vec

---

## 下一章

[第 13 章：HashMap](13-HashMap.md)
