## 15.5 Trait 约束（Trait Bounds）

### 为什么需要约束？

```rust
// ❌ 错误：T 可能没有 > 运算符
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {  // 编译错误！
//             largest = item;
//         }
//     }
//     largest
// }

// ✅ 正确：约束 T 实现 PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 约束的语法

```rust
use std::fmt::{Display, Debug};

// 单个约束
fn print<T: Display>(item: T) {
    println!("{}", item);
}

// 多个约束（使用 + 连接）
fn print_and_clone<T: Display + Clone>(item: T) {
    let cloned = item.clone();
    println!("{}", cloned);
}

// 多个泛型参数各有约束
fn compare<T: PartialOrd, U: PartialOrd>(t: T, u: U) -> bool {
    // T 和 U 各自需要 PartialOrd
    false
}
```

### where 子句

```rust
use std::fmt::{Display, Debug};

// 不使用 where（约束写在参数列表，可读性差）
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    // 函数体
    0
}

// 使用 where（约束单独写，更清晰）
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // 函数体
    0
}

// 复杂示例：返回类型也依赖泛型
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone + 'static,
    U: Clone + Debug + PartialEq<T>,
{
    format!("t: {}, u: {:?}", t, u)
}

fn main() {
    println!("{}", some_function(42, "hello"));
}
```

### 常用 Trait 约束

```rust
use std::fmt::{Display, Debug};
use std::ops::{Add, Sub, Mul, Div};

// 可打印
fn print_display<T: Display>(item: T) {
    println!("{}", item);
}

fn print_debug<T: Debug>(item: T) {
    println!("{:?}", item);
}

// 可比较
fn is_equal<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

fn find_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 可克隆
fn duplicate<T: Clone>(item: &T) -> (T, T) {
    (item.clone(), item.clone())
}

// 可运算
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    print_display(42);
    print_debug(vec![1, 2, 3]);
    println!("{}", is_equal(&5, &5));
    println!("{}", find_max(10, 20));
    println!("{:?}", duplicate(&String::from("hello")));
    println!("{}", add(2, 3));
}
```

---

---

## 下一步

[where 子句的高级用法](../6-where 子句的高级用法.md)