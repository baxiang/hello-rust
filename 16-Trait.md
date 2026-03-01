# 第 16 章：Trait 详解

> 定义共享行为的抽象——Rust 多态的基石

---

## 16.1 为什么需要 Trait？

### 代码组织问题

```rust
// 问题：不同类型的相似操作

struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    fn get_summary(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
}

impl Tweet {
    fn get_summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 问题：无法编写通用函数处理两者
// fn print_summary(item: ???) {
//     println!("{}", item.get_summary());
// }
```

### Trait 的解决方案

```rust
// 使用 Trait 定义共享行为
trait Summary {
    fn get_summary(&self) -> String;
}

impl Summary for NewsArticle {
    fn get_summary(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn get_summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 现在可以编写通用函数
fn print_summary(item: &impl Summary) {
    println!("{}", item.get_summary());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        author: String::from("John"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Hello world!"),
    };

    print_summary(&article);  // Breaking News, by John
    print_summary(&tweet);    // @rustlang: Hello world!
}
```

### Trait 的核心作用

```
┌─────────────────────────────────────────────────────┐
│              Trait 的核心作用                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. 抽象行为（接口）                                 │
│     • 定义一组方法的签名                            │
│     • 不关心具体实现                                │
│     • 类似其他语言的 interface                      │
│                                                     │
│  2. 泛型约束                                        │
│     • 限制泛型参数的可用方法                        │
│     • 编译时类型检查                                │
│     • 零运行时开销                                  │
│                                                     │
│  3. 代码复用                                        │
│     • 默认实现减少重复代码                         │
│     • 为现有类型添加行为（扩展）                   │
│     • 组合优于继承                                  │
│                                                     │
│  4. 多态支持                                        │
│     • Trait 对象（动态分发）                        │
│     • 运行时可以选择不同实现                        │
│     • 灵活但有小开销                                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## 16.2 定义 Trait

### 基本语法

```rust
// Trait 定义
pub trait Summary {
    // 抽象方法（必须由实现者提供）
    fn summarize(&self) -> String;
}

// Trait 可以有多个方法
pub trait Formatter {
    // 抽象方法
    fn format(&self) -> String;

    // 抽象方法
    fn format_json(&self) -> String;
}
```

### 方法签名

```rust
trait Example {
    // 使用&self（最常见）
    fn read_only(&self);

    // 使用&mut self（修改自身）
    fn modify(&mut self);

    // 使用 self（获取所有权）
    fn consume(self) -> i32;

    // 关联函数（类似静态方法）
    fn create() -> Self;

    // 带参数的方法
    fn process(&self, data: &str) -> bool;
}
```

### 关联类型

```rust
// Trait 可以有类型占位符
trait Iterator {
    type Item;  // 关联类型

    fn next(&mut self) -> Option<Self::Item>;
}

// 实现时指定具体类型
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;  // 指定关联类型

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}
```

---

## 16.3 实现 Trait

### 基本实现

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Trait for Type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply_to: Option<String>,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("John Doe"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Hello, world!"),
        reply_to: None,
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
```

### 孤儿规则（Orphan Rule）

```
┌─────────────────────────────────────────────────────┐
│              孤儿规则                                │
├─────────────────────────────────────────────────────┤
│                                                     │
│  规则：                                              │
│  不能为外部的 Trait 实现外部的类型                   │
│                                                     │
│  允许的情况：                                        │
│  ✓ 为本地 Trait 实现本地类型                        │
│  ✓ 为本地 Trait 实现外部类型                        │
│  ✓ 为外部 Trait 实现本地类型                        │
│                                                     │
│  不允许的情况：                                      │
│  ✗ 为外部 Trait 实现外部类型                        │
│    例：impl Display for Vec<String>  // ❌          │
│    （Display 和 Vec 都是 std 的）                    │
│                                                     │
│  解决方案：                                          │
│  • 使用新类型模式（Newtype Pattern）                │
│  • 包装外部类型                                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 新类型模式

```rust
use std::fmt::Display;

// ❌ 错误：违反孤儿规则
// impl Display for Vec<String> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // ...
//     }
// }

// ✅ 正确：使用新类型模式
struct MyVec(Vec<String>);

impl Display for MyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let my_vec = MyVec(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("{}", my_vec);  // [hello, world]
}
```

---

## 16.4 默认实现

### 定义默认方法

```rust
pub trait Summary {
    // 默认实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// 可以使用默认实现
impl Summary for Tweet {}

fn main() {
    let tweet = Tweet {
        username: String::from("user"),
        content: String::from("content"),
    };

    println!("{}", tweet.summarize());  // (Read more...)
}
```

### 重写默认实现

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for Tweet {
    // 重写默认实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### 调用默认实现

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Default)")
    }

    // 默认实现调用其他方法
    fn summarize_fancy(&self) -> String {
        format!("*** {} ***", self.summarize())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    // 重写 summarize
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // 可以显式调用默认实现
    fn summarize_fancy(&self) -> String {
        // 使用完全限定语法调用默认实现
        <Tweet as Summary>::summarize_fancy(self)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("user"),
        content: String::from("hello"),
    };

    println!("{}", tweet.summarize());       // user: hello
    println!("{}", tweet.summarize_fancy()); // *** user: hello ***
}
```

### 相互依赖的默认实现

```rust
pub trait Equal {
    // 默认实现使用 other_equal
    fn equal(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        self.other_equal(other)
    }

    // 默认实现使用 equal
    fn other_equal(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        self.equal(other)
    }

    // 必须实现的方法
    fn is_equal(&self, other: &Self) -> bool;
}

// 实现者只需实现 is_equal
impl Equal for i32 {
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}
```

---

## 16.5 Trait 作为函数参数

### impl Trait 语法（推荐）

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

// 使用 impl Trait 语法
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet { /* ... */ };
    let article = NewsArticle { /* ... */ };

    notify(&tweet);
    notify(&article);
}
```

### 等同于泛型约束

```rust
// 以下两种写法在大多数情况下等价

// impl Trait 语法（更简洁）
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 泛型约束语法（更明确）
pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

### 多个 impl Trait 参数

```rust
// 每个 impl Trait 是独立的类型
pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// 等价于
pub fn notify_two<T: Summary>(item1: &T, item2: &T) {
    // item1 和 item2 必须是同一类型
}

pub fn notify_two<T: Summary, U: Summary>(item1: &T, item2: &U) {
    // item1 和 item2 可以是不同类型
}
```

---

## 16.6 Trait 约束

### 单个约束

```rust
fn print_item<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}
```

### 多个约束（使用 +）

```rust
// T 必须同时实现 Summary 和 Display
pub fn notify(item: &(impl Summary + std::fmt::Display)) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}

// 泛型语法
pub fn notify<T: Summary + std::fmt::Display>(item: &T) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}
```

### where 子句

```rust
// 复杂约束使用 where 更清晰
fn some_function<T, U>(t: T, u: U) -> String
where
    T: Summary + std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone + PartialEq,
{
    format!("t: {}, u: {:?}", t, u)
}
```

### 约束关联类型

```rust
use std::fmt::Display;

// 约束 Iterator 的 Item 类型
fn print_all<T>(iter: T)
where
    T: IntoIterator,
    T::Item: Display,
{
    for item in iter {
        println!("{}", item);
    }
}

fn main() {
    print_all(vec![1, 2, 3]);
    print_all(vec!["a", "b", "c"]);
}
```

---

## 16.7 返回实现 Trait 的类型

### impl Trait 返回类型

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

// 返回实现 Summary 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
    }
}

fn main() {
    let item = returns_summarizable();
    println!("{}", item.summarize());
}
```

### 限制：只能返回单一类型

```rust
// ❌ 错误：返回了不同类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Tweet { /* ... */ }
//     } else {
//         NewsArticle { /* ... */ }
//     }
// }

// ✅ 正确：使用 Trait 对象（Box）
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course"),
        })
    } else {
        Box::new(NewsArticle {
            headline: String::from("Breaking"),
            location: String::from("NYC"),
            author: String::from("John"),
            content: String::from("..."),
        })
    }
}
```

### impl Trait vs Box<dyn Trait>

```
┌─────────────────────────────────────────────────────┐
│      impl Trait vs Box<dyn Trait>                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  impl Trait（静态分发）                             │
│  ├── 编译时确定类型                                 │
│  ├── 零运行时开销                                   │
│  ├── 只能返回单一类型                             │
│  └── 更高效的调用                                   │
│                                                     │
│  Box<dyn Trait>（动态分发）                         │
│  ├── 运行时确定类型                                 │
│  ├── 有小开销（虚函数表查找）                      │
│  ├── 可以返回不同类型                             │
│  └── 更灵活                                         │
│                                                     │
│  选择：                                              │
│  • 需要返回不同类型 → Box<dyn Trait>               │
│  • 追求性能 → impl Trait                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## 16.8 条件实现

### 基于泛型约束的实现

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有当 T 实现 Display + PartialOrd 时，Pair<T> 才有这个方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("较大的是 x: {}", self.x);
        } else {
            println!("较大的是 y: {}", self.y);
        }
    }
}

// 为所有 T 实现 Debug
impl<T: std::fmt::Debug> Pair<T> {
    fn debug_print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let p1 = Pair { x: 5, y: 10 };
    p1.cmp_display();        // 较大的是 y: 10
    p1.debug_print();        // Pair { x: 5, y: 10 }

    let p2 = Pair { x: 1.0, y: 2.0 };
    p2.cmp_display();        // 较大的是 y: 2.0
}
```

### blanket implementation

```rust
// 为所有实现 Summary 的类型实现 toString
trait Summary {
    fn summarize(&self) -> String;
}

trait ToString {
    fn to_string(&self) -> String;
}

// 为所有实现 Summary 的类型自动实现 ToString
impl<T: Summary> ToString for T {
    fn to_string(&self) -> String {
        self.summarize()
    }
}
```

---

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

---

## 16.10 Trait 对象（动态分发）

### 什么是 Trait 对象

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("绘制圆，半径 = {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("绘制矩形，{}x{}", self.width, self.height);
    }
}

// Trait 对象：dyn Draw
fn main() {
    // 存储不同类型的 Trait 对象
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];

    // 动态调用各自的方法
    for shape in shapes {
        shape.draw();
    }
}
```

### 动态 vs 静态分发

```
┌─────────────────────────────────────────────────────┐
│          动态分发 vs 静态分发                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  静态分发（泛型/impl Trait）                        │
│  ├── 编译时确定调用哪个方法                        │
│  ├── 直接函数调用，无开销                          │
│  ├── 代码可能膨胀（单态化）                        │
│  └── 类型必须在编译时已知                          │
│                                                     │
│  动态分发（Trait 对象/dyn Trait）                    │
│  ├── 运行时通过虚函数表查找方法                    │
│  ├── 有小开销（间接调用）                          │
│  ├── 代码不膨胀                                    │
│  └── 可以在运行时存储不同类型                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## 16.11 完整示例

### 示例 1：可绘制形状系统

```rust
use std::fmt::Display;

/// 可绘制的形状
trait Draw {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
}

/// 圆形
#[derive(Debug, Clone)]
struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) -> String {
        format!("○ 圆形 (半径 = {})", self.radius)
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// 矩形
#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) -> String {
        format!("□ 矩形 ({}x{})", self.width, self.height)
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// 三角形
#[derive(Debug, Clone)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Draw for Triangle {
    fn draw(&self) -> String {
        format!("△ 三角形 (底 = {}, 高 = {})", self.base, self.height)
    }

    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// 画布
struct Canvas {
    shapes: Vec<Box<dyn Draw>>,
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            shapes: Vec::new(),
        }
    }

    fn add(&mut self, shape: impl Draw + 'static) {
        self.shapes.push(Box::new(shape));
    }

    fn render(&self) {
        println!("╔════════════════════════════════════╗");
        println!("║           画布内容                  ║");
        println!("╠════════════════════════════════════╣");
        for (i, shape) in self.shapes.iter().enumerate() {
            println!("║ {}: {:<28} ║", i + 1, shape.draw());
        }
        println!("╠════════════════════════════════════╣");
        println!("║ 总面积：{:<26.2f} ║", self.total_area());
        println!("╚════════════════════════════════════╝");
    }

    fn total_area(&self) -> f64 {
        self.shapes.iter().map(|s| s.area()).sum()
    }
}

fn main() {
    let mut canvas = Canvas::new();

    canvas.add(Circle { radius: 5.0 });
    canvas.add(Rectangle { width: 10.0, height: 20.0 });
    canvas.add(Triangle { base: 6.0, height: 8.0 });
    canvas.add(Circle { radius: 3.0 });

    canvas.render();
}
```

**输出：**
```
╔════════════════════════════════════╗
║           画布内容                  ║
╠════════════════════════════════════╣
║ 1: ○ 圆形 (半径 = 5)            ║
║ 2: □ 矩形 (10x20)               ║
║ 3: △ 三角形 (底 = 6, 高 = 8)    ║
║ 4: ○ 圆形 (半径 = 3)            ║
╠════════════════════════════════════╣
║ 总面积：310.54                    ║
╚════════════════════════════════════╝
```

### 示例 2：插件系统

```rust
/// 插件 Trait
trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

/// 大写插件
struct UppercasePlugin;

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str {
        "Uppercase"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn execute(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

/// 反转插件
struct ReversePlugin;

impl Plugin for ReversePlugin {
    fn name(&self) -> &str {
        "Reverse"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn execute(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

/// 去空格插件
struct TrimPlugin;

impl Plugin for TrimPlugin {
    fn name(&self) -> &str {
        "Trim"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn execute(&self, input: &str) -> String {
        input.trim().to_string()
    }
}

/// 插件管理器
struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    fn register(&mut self, plugin: impl Plugin + 'static) {
        println!("注册插件：{} v{}", plugin.name(), plugin.version());
        self.plugins.push(Box::new(plugin));
    }

    fn process(&self, input: &str) -> String {
        let mut result = input.to_string();
        for plugin in &self.plugins {
            result = plugin.execute(&result);
        }
        result
    }

    fn list_plugins(&self) {
        println!("已注册的插件:");
        for plugin in &self.plugins {
            println!("  - {} v{}", plugin.name(), plugin.version());
        }
    }
}

fn main() {
    let mut manager = PluginManager::new();

    manager.register(TrimPlugin);
    manager.register(UppercasePlugin);
    manager.register(ReversePlugin);

    manager.list_plugins();

    let input = "  hello world  ";
    println!("\n输入：'{}'", input);
    println!("输出：'{}'", manager.process(input));
}
```

---

## 16.12 常见错误

### 错误 1：缺少必要的方法实现

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    content: String,
}

// ❌ 错误：缺少 summarize 实现
// impl Summary for Tweet {}

// ✅ 正确
impl Summary for Tweet {
    fn summarize(&self) -> String {
        self.content.clone()
    }
}
```

**错误信息：**
```
error[E0046]: not all trait items implemented, missing: `summarize`
```

### 错误 2：方法签名不匹配

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    content: String,
}

// ❌ 错误：签名不匹配
// impl Summary for Tweet {
//     fn summarize(&self) { }  // 缺少返回类型
// }

// ✅ 正确
impl Summary for Tweet {
    fn summarize(&self) -> String {
        self.content.clone()
    }
}
```

### 错误 3：Sized 问题

```rust
// ❌ 错误：dyn Trait 大小不固定
// fn process(item: dyn Summary) { }

// ✅ 正确：使用引用或 Box
fn process(item: &dyn Summary) { }
fn process_box(item: Box<dyn Summary>) { }
```

---

## 16.13 练习

### 练习 1：Area Trait

定义一个 `Area` trait，为不同形状计算面积：
- Circle（圆）
- Rectangle（矩形）
- Triangle（三角形）

### 练习 2：Serialize Trait

实现一个 `Serialize` trait：
```rust
trait Serialize {
    fn to_json(&self) -> String;
}
```

为 User 结构体实现该 trait。

### 练习 3：Compare Trait

创建 `Compare` trait，实现：
- `max(a, b)` - 返回较大值
- `min(a, b)` - 返回较小值

---

## 16.14 小结

### Trait 核心概念

| 概念 | 说明 |
|------|------|
| 定义 | `trait Name { fn method(); }` |
| 实现 | `impl Trait for Type` |
| 默认实现 | 提供方法的默认行为 |
| 泛型约束 | `T: Trait` 或 `impl Trait` |
| Trait 对象 | `Box<dyn Trait>`（动态分发） |

### 常用模式

```rust
// 1. 作为参数
fn process(item: &impl Trait) { }
fn process<T: Trait>(item: &T) { }

// 2. 作为返回值
fn create() -> impl Trait { }
fn create() -> Box<dyn Trait> { }

// 3. 多个约束
fn process<T: Trait1 + Trait2>(item: &T) { }

// 4. where 子句
fn process<T>(item: &T) where T: Trait1 + Trait2 { }
```

### 关键要点

1. **Trait 定义行为**，结构体/枚举实现行为
2. **孤儿规则**限制为外部类型实现外部 Trait
3. **默认实现**可以减少重复代码
4. **impl Trait** 用于静态分发（高效）
5. **dyn Trait** 用于动态分发（灵活）

---

## 下一章

[第 17 章：生命周期](17-生命周期.md)
