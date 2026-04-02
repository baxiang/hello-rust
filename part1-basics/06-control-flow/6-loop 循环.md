## 6.6 loop 循环

### 基本用法

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("counter = {}", counter);

        if counter == 5 {
            break;  // 退出循环
        }
    }
}
```

### loop 可以返回值

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // 返回值 20
        }
    };

    println!("result = {}", result);  // 20
}
```

### 循环标签（嵌套循环控制）

```rust
fn main() {
    // 默认 break 只跳出最内层循环
    'outer: for i in 1..=3 {
        'inner: for j in 1..=3 {
            if i == 2 && j == 2 {
                break 'outer;  // 直接跳出外层循环
            }
            println!("i = {}, j = {}", i, j);
        }
    }
    println!("循环结束");
}
```

**输出：**
```
i = 1, j = 1
i = 1, j = 2
i = 1, j = 3
i = 2, j = 1
循环结束
```




