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

---

## 下一步

[数组 vs Vec：选择指南](../6-数组 vs Vec：选择指南.md)