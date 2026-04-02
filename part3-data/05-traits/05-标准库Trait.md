## 16.9 常用标准库 Trait

### 派生 Trait 速查

```rust
// Debug - 调试输出
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Clone - 深拷贝
#[derive(Clone)]
struct Data {
    value: String,
}

// Copy - 隐式复制（只适用于栈上类型）
#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

// PartialEq - 相等比较
#[derive(PartialEq)]
struct Color(u8, u8, u8);

// Eq - 完全相等（需要 PartialEq）
#[derive(PartialEq, Eq)]
struct Id(u32);

// PartialOrd - 部分排序
#[derive(PartialEq, PartialOrd)]
struct Score(f64);

// Ord - 完全排序（需要 PartialEq, Eq, PartialOrd）
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Rank(u32);

// Hash - 哈希
#[derive(PartialEq, Eq, Hash)]
struct Key(String);

// Default - 默认值
#[derive(Default)]
struct Config {
    debug: bool,
    port: u16,
}

// 组合派生
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: u32,
    name: String,
}
```

### Display 和 Debug

```rust
use std::fmt::{self, Display, Debug};

// Debug 可以派生
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Display 需要手动实现
impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 25,
    };

    // Debug
    println!("{:?}", p);   // Person { name: "Alice", age: 25 }
    println!("{:#?}", p);  // 美化格式

    // Display
    println!("{}", p);     // Alice (25)
}
```

### From 和 Into

```rust
// From 和 Into 是相互关联的
// 实现 From 会自动获得 Into

struct Wrapper(String);

// 实现 From<&str>
impl From<&str> for Wrapper {
    fn from(s: &str) -> Self {
        Wrapper(s.to_string())
    }
}

// 自动获得 Into<Wrapper>
fn main() {
    // 使用 From
    let w = Wrapper::from("hello");

    // 使用 Into（自动实现）
    let w: Wrapper = "hello".into();

    // 显式类型转换
    let s = String::from("hello");
    let w: Wrapper = s.into();
}
```

### From 的标准库示例

```rust
fn main() {
    // String 相关转换
    let s: String = String::from("hello");
    let s: String = "hello".to_string();
    let s: String = "hello".into();

    // 数字转换
    let a: i32 = 5;
    let b: i64 = a.into();  // i32 → i64

    // 错误转换
    let err: Box<dyn std::error::Error> =
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, "error"));
}
```







