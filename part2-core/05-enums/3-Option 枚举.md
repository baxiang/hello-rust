## 11.3 Option 枚举

### 什么是 Option？

```rust
// Rust 标准库中的定义
enum Option<T> {
    Some(T),  // 有值
    None,     // 无值
}
```

### 为什么 Option 更好？

```
其他语言的 null 问题：
┌─────────────────────────────────────────┐
│  String name = null;                    │
│  name.length();  // NullPointerException! │
│                                         │
│  问题：null 可以赋值给任何引用类型        │
│  编译器不强制检查 null                   │
└─────────────────────────────────────────┘

Rust 的 Option：
┌─────────────────────────────────────────┐
│  let name: Option<String> = None;       │
│                                         │
│  // 必须处理 None 情况才能使用值         │
│  match name {                           │
│      Some(n) => println!("{}", n.len()),│
│      None => println!("无名字"),         │
│  }                                      │
│                                         │
│  优势：编译器强制处理，无意外 panic      │
└─────────────────────────────────────────┘
```

### Option 基本用法

```rust
fn main() {
    // 创建 Option
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // 显式类型标注
    let absent_number: Option<i32> = None;

    // 处理 Option
    match some_number {
        Some(n) => println!("数字是：{}", n),
        None => println!("没有数字"),
    }

    // 类型推断
    let x = Some(5);  // Option<i32>
    let y: Option<_> = None;  // Option<T>
}
```

### Option 常用方法

```rust
fn main() {
    let some = Some(5);
    let none: Option<i32> = None;

    // unwrap - 获取值，没有则 panic
    println!("{}", some.unwrap());  // 5
    // println!("{}", none.unwrap());  // panic!

    // unwrap_or - 获取值，没有则返回默认值
    println!("{}", some.unwrap_or(0));  // 5
    println!("{}", none.unwrap_or(0));  // 0

    // unwrap_or_else - 使用闭包提供默认值
    println!("{}", some.unwrap_or_else(|| 42));
    println!("{}", none.unwrap_or_else(|| 42));

    // is_some / is_none
    println!("{}", some.is_some());  // true
    println!("{}", none.is_none());  // true

    // map - 转换值
    let doubled = some.map(|x| x * 2);
    println!("{:?}", doubled);  // Some(10)

    // and_then - 链式操作
    let result = some.and_then(|x| Some(x * 2));
    println!("{:?}", result);  // Some(10)
}
```

### Option 实际应用

```rust
// 查找用户
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),
        2 => Some(String::from("Bob")),
        _ => None,
    }
}

fn main() {
    // 处理查找结果
    match find_user(1) {
        Some(name) => println!("找到用户：{}", name),
        None => println!("用户不存在"),
    }

    // 链式调用
    let user = find_user(1)
        .map(|name| name.to_uppercase())
        .unwrap_or(String::from("UNKNOWN"));

    println!("{}", user);  // ALICE
}
```

---

---

## 下一步

[Result 枚举](../4-Result 枚举.md)