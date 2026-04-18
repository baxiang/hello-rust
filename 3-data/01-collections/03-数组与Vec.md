# 数组与Vec

> 学习数组与Vec的相互转换，理解切片作为通用接口的作用，掌握选择指南。

## 数组与 Vec 的转换

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

## 数组 vs Vec：选择指南

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

## 小结

- 数组转Vec：`arr.to_vec()`、`Vec::from(arr)`、`into_iter().collect()`
- Vec转数组：`v.try_into()`（返回 Result）、`copy_from_slice()`
- 切片 `&[T]`：数组和Vec的共同接口，函数参数优先使用切片
- 选择：固定大小用数组，动态大小用Vec

## 练习题

详见：[练习题](../../exercises/01-collections.md)
```




