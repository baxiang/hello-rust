## 8.8 &str vs &String

### 最佳实践：使用 &str

```rust
// ❌ 不推荐：只接受 &String
fn greet_string(s: &String) {
    println!("Hello, {}!", s);
}

// ✅ 推荐：接受 &str
fn greet_str(s: &str) {
    println!("Hello, {}!", s);
}

fn main() {
    let owned = String::from("Alice");
    let borrowed = "Bob";

    // &String 函数只能接受 String 的引用
    greet_string(&owned);
    // greet_string(borrowed);  // ❌ 错误

    // &str 函数可以接受多种类型
    greet_str(&owned);    // &String → &str（自动转换）
    greet_str(borrowed);  // &'static str
    greet_str(&owned[0..3]);  // 切片也是&str
}
```

### 为什么 &str 更灵活

```
类型转换关系：

String ──────→ &str  (通过 Deref)
  ↓                      ↑
  │                      │
  │                      │
&String ─────→ &str  (通过 Deref coercion)


结论：&str 是"通用接口"，可以接受：
• 字符串字面量
• String 的引用
• 字符串切片
```

---

---

## 下一步

[完整示例](../9-完整示例.md)