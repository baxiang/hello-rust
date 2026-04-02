## 8.6 悬垂引用（Dangling Reference）

### 什么是悬垂引用？

```
悬垂引用 = 指向已释放内存的引用

危险：
1. 访问无效内存
2. 程序崩溃
3. 安全漏洞
```

### Rust 如何防止

```rust
// ❌ 错误：返回悬垂引用
// fn create_dangle() -> &String {
//     let s = String::from("hello");
//     &s  // s 在函数结束时被释放
// }
```

**错误信息：**
```
error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:4:5
  |
4 |     &s
  |     ^^ returns a reference to data owned by the current function
```

### 正确的做法

```rust
// 方案 1：返回所有权
fn create_string() -> String {
    String::from("hello")
}

fn main() {
    let s = create_string();
    println!("{}", s);
}

// 方案 2：使用静态生命周期
fn get_static() -> &'static str {
    "hello"  // 字符串字面量，整个程序生命周期
}

fn main() {
    let s = get_static();
    println!("{}", s);
}
```




