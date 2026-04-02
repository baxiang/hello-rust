## 19.6 产生适配器（Producing Adapters）

### map

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // map - 转换每个元素
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    println!("翻倍：{:?}", doubled);

    // 转换为字符串
    let strings: Vec<String> = numbers
        .iter()
        .map(|x| format!("数字 {}", x))
        .collect();
    println!("格式化：{:?}", strings);

    // 链式 map
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .map(|x| x + 1)
        .collect();
    println!("链式：{:?}", result);  // [3, 5, 7, 9, 11]
}
```

### filter

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter - 过滤元素
    let evens: Vec<&i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("偶数：{:?}", evens);

    // filter 链式
    let result: Vec<&i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .filter(|&x| *x > 5)
        .collect();
    println!("大于 5 的偶数：{:?}", result);

    // filter 与其他组合
    let sum_even: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .sum();
    println!("偶数和：{}", sum_even);
}
```

### filter_map

```rust
fn main() {
    // filter_map - 过滤并转换
    let strings = vec!["1", "two", "3", "four", "5"];

    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("解析结果：{:?}", numbers);  // [1, 3, 5]

    // 等价于 filter + map
    let numbers2: Vec<i32> = strings
        .iter()
        .filter_map(|s| {
            s.parse::<i32>()
                .ok()
                .map(|n| n * 2)  // 只对成功解析的翻倍
        })
        .collect();
    println!("解析并翻倍：{:?}", numbers2);  // [2, 6, 10]
}
```

### take 和 skip

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // take - 取前 n 个
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("前 3 个：{:?}", first_three);

    // skip - 跳过前 n 个
    let skip_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("跳过前 3 个：{:?}", skip_three);

    // take_while - 取到条件不满足
    let less_than_5: Vec<&i32> = numbers
        .iter()
        .take_while(|&x| *x < 5)
        .collect();
    println!("小于 5: {:?}", less_than_5);  // [1, 2, 3, 4]

    // skip_while - 跳过直到条件不满足
    let rest: Vec<&i32> = numbers
        .iter()
        .skip_while(|&x| *x < 5)
        .collect();
    println!("跳过小于 5 后剩余：{:?}", rest);  // [5, 6, 7, 8, 9, 10]
}
```

### enumerate

```rust
fn main() {
    let fruits = vec!["apple", "banana", "cherry"];

    // enumerate - 添加索引
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }

    // 带起始索引（Rust 1.62+）
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index + 1, fruit);
    }

    // 收集为带索引的元组
    let indexed: Vec<(usize, &&str)> = fruits.iter().enumerate().collect();
    println!("{:?}", indexed);
}
```

### zip

```rust
fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec!["a", "b", "c", "d", "e"];

    // zip - 合并两个迭代器
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("zip 结果：{:?}", zipped);
    // [(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")]

    // 使用 zip 遍历
    for (num, letter) in a.iter().zip(b.iter()) {
        println!("{}: {}", num, letter);
    }

    // zip 处理不同长度的迭代器（以短的为准）
    let short = vec![1, 2, 3];
    let long = vec!["a", "b", "c", "d", "e"];
    let zipped: Vec<_> = short.iter().zip(long.iter()).collect();
    println!("不同长度：{:?}", zipped);  // 只有 3 个
}
```

### chain

```rust
fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    // chain - 连接两个迭代器
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("连接结果：{:?}", chained);
    // [1, 2, 3, 4, 5, 6]

    // 链式操作
    let result: Vec<i32> = a.iter()
        .chain(b.iter())
        .map(|&x| x * 2)
        .collect();
    println!("连接并翻倍：{:?}", result);
}
```

### cycle 和 repeat

```rust
fn main() {
    // cycle - 循环迭代
    let mut cycle_iter = [1, 2, 3].iter().cycle();
    println!("{:?}", cycle_iter.next());  // Some(&1)
    println!("{:?}", cycle_iter.next());  // Some(&2)
    println!("{:?}", cycle_iter.next());  // Some(&3)
    println!("{:?}", cycle_iter.next());  // Some(&1)，重新开始

    // 限制循环次数
    let limited: Vec<_> = [1, 2].iter().cycle().take(5).collect();
    println!("{:?}", limited);  // [1, 2, 1, 2, 1]

    // std::iter::repeat - 重复单个值
    use std::iter::repeat;
    let nines: Vec<_> = repeat(9).take(5).collect();
    println!("{:?}", nines);  // [9, 9, 9, 9, 9]

    // std::iter::repeat_with - 使用闭包生成
    use std::iter::repeat_with;
    let mut counter = 0;
    let generated: Vec<_> = repeat_with(|| {
        counter += 1;
        counter * 10
    }).take(5).collect();
    println!("{:?}", generated);  // [10, 20, 30, 40, 50]
}
```

---

---

## 下一步

[惰性求值](../7-惰性求值.md)