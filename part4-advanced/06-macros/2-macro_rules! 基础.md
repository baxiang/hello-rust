## 25.2 macro_rules! 基础

### 基本语法

```rust
// 定义宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();  // 调用宏
}
```

### 带参数的宏

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("函数 {:?} 被调用", stringify!($func_name));
        }
    };
}

// 使用宏生成函数
create_function!(foo);
create_function!(bar);

fn main() {
    foo();  // 函数 "foo" 被调用
    bar();  // 函数 "bar" 被调用
}
```

### 宏展开

```rust
// 宏调用
say_hello!();

// 展开后（概念上）
{
    println!("Hello!");
}
```

---

---

## 下一步

[宏片段说明符](../3-宏片段说明符.md)