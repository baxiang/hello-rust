## 9.5 函数参数使用 &str

### 最佳实践

```rust
// ✅ 推荐：接受 &str（最灵活）
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let s1 = String::from("Alice");
    let s2 = "Bob";
    let s3 = String::from("Charlie");

    // &str 可以接受多种类型
    greet(&s1);           // &String → &str
    greet(s2);            // &'static str
    greet(&s3[..]);       // 切片
    greet(&s3[0..4]);     // 部分切片
}
```

### 不推荐的做法

```rust
// ❌ 不推荐：只接受 &String
fn greet_bad(name: &String) {
    println!("Hello, {}!", name);
}

fn main() {
    let s1 = String::from("Alice");
    let s2 = "Bob";

    greet_bad(&s1);   // ✅ 可以
    // greet_bad(s2); // ❌ 错误！&str 不能转&String
}
```

### 为什么&str 更好？

```
&str 的通用性：

                ┌───────────────┐
                │   函数参数    │
                │   fn(&str)    │
                └───────┬───────┘
                        │
         ┌──────────────┼──────────────┐
         │              │              │
         ▼              ▼              ▼
   ┌─────────┐   ┌─────────┐   ┌─────────┐
   │ &String │   │  &str   │   │  切片   │
   │  转换   │   │  直接   │   │  直接   │
   └─────────┘   └─────────┘   └─────────┘

结论：&str 是字符串参数的"通用接口"
```

---

---

## 下一步

[数组切片](../6-数组切片.md)