## 6.2 if 表达式

### 基本语法

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("小于 5");
    } else if number < 10 {
        println!("在 5 到 10 之间");
    } else {
        println!("大于等于 10");
    }
}
```

### if 是表达式（可以返回值）

这是 Rust 与其他语言的重要区别：

```rust
fn main() {
    let number = 7;

    // if 可以返回值，赋值给变量
    let description = if number < 5 {
        "small"      // 没有分号，是表达式
    } else if number < 10 {
        "medium"
    } else {
        "large"
    };

    println!("数字是 {}", description);  // medium

    // 可以直接在 println! 中使用
    println!("数字是 {}", if number > 5 { "大" } else { "小" });
}
```

### 重要规则：分支类型必须一致

```rust
fn main() {
    // ✅ 正确：所有分支返回相同类型
    let result = if true {
        5
    } else {
        10
    };
    println!("result = {}", result);  // i32

    // ❌ 错误：分支类型不一致
    // let result = if true {
    //     5           // i32
    // } else {
    //     "five"      // &str
    // };

    // ✅ 正确：使用 () 统一类型
    let result = if true {
        println!("执行了");
        // 隐式返回 ()
    } else {
        5
        // 这里会报错，因为 () != i32
    };
}
```

### 条件必须是布尔类型

```rust
fn main() {
    let number = 7;

    // ✅ 正确：条件是布尔表达式
    if number > 0 {
        println!("正数");
    }

    // ❌ 错误：Rust 没有"真值转换"
    // if number {
    //     println!("真");
    // }

    // C/JavaScript/Python 中，非零值被认为是 true
    // Rust 要求明确的布尔条件

    // ✅ 正确写法
    if number != 0 {
        println!("非零");
    }
}
```

**错误信息：**
```
error[E0308]: mismatched types
 --> src/main.rs:5:8
  |
5 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

### 复杂条件

```rust
fn main() {
    let age = 25;
    let has_id = true;
    let has_ticket = true;

    // 逻辑与 (&&) - 短路求值
    if age >= 18 && has_id {
        println!("可以进入（成年且有身份证）");
    }

    // 逻辑或 (||) - 短路求值
    if has_id || has_ticket {
        println!("有身份证或门票，可以进入");
    }

    // 组合条件
    if age >= 18 && (has_id || has_ticket) {
        println!("成年人，且有身份证或门票");
    }

    // 短路求值演示
    let result = false && {
        println!("这行不会打印（因为左边是 false）");
        true
    };

    let result = true || {
        println!("这行也不会打印（因为左边是 true）");
        true
    };
}
```

---

---

## 下一步

[if let](../3-if let.md)