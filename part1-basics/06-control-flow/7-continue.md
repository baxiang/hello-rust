## 6.7 continue

```rust
fn main() {
    // 跳过偶数，只打印奇数
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;  // 跳过本次迭代
        }
        println!("{}", i);
    }
}
```

**输出：**
```
1
3
5
7
9
```

### continue 在嵌套循环中

```rust
fn main() {
    for i in 1..=3 {
        for j in 1..=3 {
            if j == 2 {
                continue;  // 只跳过内层循环的当前迭代
            }
            println!("i = {}, j = {}", i, j);
        }
    }
}
```

**输出：**
```
i = 1, j = 1
i = 1, j = 3
i = 2, j = 1
i = 2, j = 3
i = 3, j = 1
i = 3, j = 3
```

---

---

## 下一步

[match 表达式](../8-match 表达式.md)