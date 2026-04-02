## 10.9 派生 Trait

### 常用派生宏

```rust
// Debug - 调试输出
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Clone - 克隆
#[derive(Clone)]
struct Data {
    value: i32,
}

// Copy - 复制（只适用于栈上类型）
#[derive(Copy, Clone)]
struct Point {
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

// 组合派生
#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}
```

### 使用派生 Trait

```rust
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p1 = Person {
        name: String::from("Alice"),
        age: 25,
    };

    // Debug
    println!("p1: {:?}", p1);
    println!("p1: {:#?}", p1);  // 美化格式

    // Clone
    let p2 = p1.clone();

    // PartialEq
    let p3 = Person {
        name: String::from("Alice"),
        age: 25,
    };

    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);  // false（String 内容相等但不是同一实例）
}
```

### 手动实现 Debug

```rust
struct Point {
    x: f64,
    y: f64,
}

// 手动实现 Debug
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p);  // Point(1.0, 2.0)
}
```




