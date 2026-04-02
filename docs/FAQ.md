# Rust 学习常见问题

> 收集整理 Rust 学习过程中的常见问题和解答

## 目录

- [学习路径问题](#学习路径问题)
- [编译错误问题](#编译错误问题)
- [所有权与借用](#所有权与借用)
- [类型系统问题](#类型系统问题)
- [工具链问题](#工具链问题)
- [项目实战问题](#项目实战问题)
- [性能优化问题](#性能优化问题)

---

## 学习路径问题

### 1. Rust 难学吗？

**答**：Rust 学习曲线较陡，但可克服。

**难点**：
- 所有权系统（第 07-09 章）
- 生命周期标注（第 17 章）
- 异步编程（第 23 章）

**建议**：
1. 不要急于求成，前 11 章务必扎实
2. 多动手练习，编译器是最好的老师
3. 利用编译器错误提示学习
4. 参见[学习路径](./learning-paths.md)规划学习

### 2. 需要其他编程语言基础吗？

**答**：不是必需，但有帮助。

**无编程经验**：
- 按初学者路径学习（2 周）
- 每章练习必须完成
- 遇到概念不懂可查阅资料

**有编程经验**：
- 可按快速路径学习（2-3 周）
- 重点学习 Rust 独特概念
- 可跳过熟悉的基础语法

### 3. 学习顺序可以调整吗？

**答**：部分章节可以，关键章节必须按顺序。

**必须按顺序**：
- 第 07-09 章：所有权 → 引用 → 切片
- 第 15-17 章：泛型 → Trait → 生命周期

**可调整顺序**：
- 第 12-13 章（Vec、HashMap）可并行
- 第 20-21 章（模块、Cargo）可提前
- 第 26-28 章（CLI、Web、测试）按需选择

参见[前置知识图谱](./prerequisites.md)

### 4. 学完基础后做什么？

**答**：选择实战项目巩固知识。

**建议路径**：
1. 完成项目一或项目二（CLI 工具）
2. 选择感兴趣的领域深入学习
3. 参与开源项目或贡献 crates.io

**领域选择**：
- Web 开发 → 第 27 章 + 项目三/五
- CLI 工具 → 第 26 章 + 项目一/二/四
- 系统编程 → 第 22-24 章 + 项目六/九

### 5. 遇到学习瓶颈怎么办？

**答**：不同瓶颈不同解决方法。

**所有权理解困难**：
- 回顾第 03 章（变量与作用域）
- 手动画内存布局图
- 使用 Rust Playground 实验

**生命周期理解困难**：
- 确保理解引用（第 08 章）
- 从简单示例开始
- 先让编译器帮你标注

**泛型和 Trait 困惑**：
- 回顾类型系统基础
- 对比其他语言（Java 接口、Go 接口）
- 阅读标准库 Trait 实现

---

## 编译错误问题

### 1. "borrow of moved value" 错误

**错误示例**：
```rust
let s1 = String::from("hello");
let s2 = s1;        // s1 的所有权转移给 s2
println!("{}", s1); // ❌ 错误：s1 已移动
```

**原因**：所有权已转移，原变量不能再使用。

**解决方案**：
```rust
// 方案 1：克隆
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}", s1); // ✅

// 方案 2：使用引用
let s1 = String::from("hello");
let s2 = &s1;
println!("{}", s1); // ✅
```

参见第 07 章：所有权与借用

### 2. "cannot borrow as mutable" 错误

**错误示例**：
```rust
let s = String::from("hello");
let r = &mut s; // ❌ 错误：不能借入可变引用
```

**原因**：变量默认不可变。

**解决方案**：
```rust
let mut s = String::from("hello");  // 声明为可变
let r = &mut s; // ✅
```

### 3. "cannot borrow as mutable more than once" 错误

**错误示例**：
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ❌ 错误：不能同时有多个可变引用
println!("{} {}", r1, r2);
```

**原因**：Rust 不允许同时存在多个可变引用。

**解决方案**：
```rust
// 方案 1：限制作用域
let mut s = String::from("hello");
{
    let r1 = &mut s;
    println!("{}", r1);
} // r1 离开作用域
let r2 = &mut s;
println!("{}", r2); // ✅

// 方案 2：使用 RefCell（运行时检查）
use std::cell::RefCell;
let s = RefCell::new(String::from("hello"));
let r1 = s.borrow_mut();
let r2 = s.borrow_mut(); // ⚠️ 运行时会 panic
```

参见第 08 章：引用与借用

### 4. "lifetime may not live long enough" 错误

**错误示例**：
```rust
fn longest(x: &str, y: &str) -> &str {  // ❌ 缺少生命周期标注
    if x.len() > y.len() { x } else { y }
}
```

**原因**：编译器无法确定返回值的生命周期。

**解决方案**：
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

参见第 17 章：生命周期

### 5. "the trait bound ... is not satisfied" 错误

**错误示例**：
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_value<T>(value: T) {
    println!("{:?}", value); // ❌ T 没有实现 Debug
}
```

**原因**：泛型类型 T 没有约束。

**解决方案**：
```rust
fn print_value<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value); // ✅
}
```

参见第 15 章：泛型、第 16 章：Trait

### 6. "use of moved value" 在循环中

**错误示例**：
```rust
let data = vec![1, 2, 3];
for item in data {  // data 被移动
    println!("{}", item);
}
println!("{:?}", data); // ❌ data 已移动
```

**原因**：for 循环获取了 data 的所有权。

**解决方案**：
```rust
// 方案 1：使用引用
let data = vec![1, 2, 3];
for item in &data {
    println!("{}", item);
}
println!("{:?}", data); // ✅

// 方案 2：迭代器方法
let data = vec![1, 2, 3];
data.iter().for_each(|item| println!("{}", item));
```

### 7. "cannot return reference to temporary value" 错误

**错误示例**：
```rust
fn get_string() -> &str {
    String::from("hello").as_str() // ❌ 返回临时值的引用
}
```

**原因**：临时值在函数结束时被销毁，引用悬垂。

**解决方案**：
```rust
// 方案 1：返回所有权
fn get_string() -> String {
    String::from("hello")
}

// 方案 2：返回 'static 生命周期
fn get_string() -> &'static str {
    "hello"  // 字符串字面量
}
```

### 8. 如何理解编译器错误提示？

**答**：Rust 编译器错误提示非常友好，充分利用。

**步骤**：
1. **阅读错误信息**：错误类型 + 位置 + 原因
2. **查看帮助提示**：编译器通常会给出建议
3. **使用 `cargo explain`**：
   ```bash
   cargo explain E0502
   ```
4. **查阅文档**：错误码对应详细说明

**示例**：
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let r1 = &s;
  |              -- immutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
5 |     println!("{} {}", r1, r2);
  |                    -- immutable borrow later used here
```

---

## 所有权与借用

### 1. 什么时候发生移动（Move）？

**答**：赋值、函数传参、函数返回时可能发生移动。

**发生移动的情况**：
```rust
// 1. 赋值
let s1 = String::from("hello");
let s2 = s1;  // s1 移动到 s2

// 2. 函数传参
let s = String::from("hello");
take_ownership(s);  // s 移动到函数

// 3. 函数返回
fn create_string() -> String {
    String::from("hello")  // 所有权转移到调用者
}
```

**不发生移动的情况**：
```rust
// Copy 类型
let x = 5;
let y = x;  // x 仍然有效（i32 实现了 Copy）

// 引用
let s = String::from("hello");
let r = &s;  // s 仍然有效（借用）

// 克隆
let s1 = String::from("hello");
let s2 = s1.clone();  // s1 仍然有效
```

参见第 07 章：所有权与借用

### 2. 哪些类型实现了 Copy trait？

**答**：基本标量类型和只包含 Copy 类型的复合类型。

**实现了 Copy 的类型**：
- 所有整数类型：i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
- 浮点类型：f32, f64
- 布尔类型：bool
- 字符类型：char
- 元组（如果所有元素都实现 Copy）：(i32, i32)
- 数组（如果元素实现 Copy）：[i32; 3]

**未实现 Copy 的类型**：
- String
- Vec<T>
- Box<T>
- HashMap<K, V>
- 自定义结构体（默认不实现）

**自定义 Copy**：
```rust
#[derive(Copy, Clone)]  // 必须同时实现 Clone
struct Point {
    x: i32,
    y: i32,
}
```

参见第 07 章：所有权与借用

### 3. 如何选择 &T、&mut T 和 T？

**答**：根据使用场景选择。

| 类型 | 使用场景 | 所有权 | 示例 |
|------|----------|--------|------|
| `T` | 需要所有权 | 转移 | 返回计算结果 |
| `&T` | 只读访问 | 借用 | 读取数据不修改 |
| `&mut T` | 读写访问 | 可变借用 | 修改数据 |

**示例**：
```rust
// T - 转移所有权
fn process(data: String) { /* data 被消费 */ }

// &T - 只读
fn read(data: &String) { println!("{}", data); }

// &mut T - 可变
fn modify(data: &mut String) { data.push_str(" world"); }
```

**性能考虑**：
- 小类型（如 i32）直接传值（Copy）
- 大类型使用引用避免复制

### 4. 如何处理复杂数据结构的所有权？

**答**：使用智能指针或引用计数。

**场景与解决方案**：

| 场景 | 解决方案 | 示例 |
|------|----------|------|
| 堆分配 | Box<T> | 递归类型 |
| 多所有权 | Rc<T> | 只读共享 |
| 多所有权+可变 | RefCell<Rc<T>> | 运行时检查 |
| 多线程共享 | Arc<T> | 原子引用计数 |

参见第 22 章：智能指针

### 5. 为什么有些函数返回 Self？

**答**：用于方法链和构建器模式。

**示例**：
```rust
struct Builder {
    name: String,
    age: u32,
}

impl Builder {
    fn new() -> Self {
        Self {
            name: String::new(),
            age: 0,
        }
    }
    
    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }
}

// 方法链
let builder = Builder::new()
    .name("Alice")
    .age(30);
```

---

## 类型系统问题

### 1. String 和 &str 有什么区别？

**答**：所有权和存储位置不同。

| 类型 | 所有权 | 存储位置 | 用途 |
|------|--------|----------|------|
| `String` | 拥有所有权 | 堆 | 可变字符串 |
| `&str` | 借用 | 栈/堆/静态 | 字符串切片 |

**示例**：
```rust
// String - 拥有所有权
let mut s = String::from("hello");
s.push_str(" world");  // ✅ 可以修改

// &str - 借用
let s: &str = "hello";
// s.push_str(" world");  // ❌ 不可变引用

// &str 可以指向 String
let string = String::from("hello");
let slice: &str = &string;  // ✅

// 函数参数优先使用 &str
fn greet(name: &str) {  // 更灵活
    println!("Hello, {}!", name);
}

greet("Alice");           // ✅ 字符串字面量
greet(&String::from("Bob")); // ✅ String 引用
```

参见第 09 章：切片

### 2. Option 和 Result 有什么区别？

**答**：用途不同。

| 类型 | 用途 | 变体 |
|------|------|------|
| `Option<T>` | 可能缺失的值 | Some(T), None |
| `Result<T, E>` | 可能失败的操作 | Ok(T), Err(E) |

**Option 使用场景**：
```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

// 使用
match find_user(1) {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}
```

**Result 使用场景**：
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

// 使用 ? 运算符
fn calculate() -> Result<f64, String> {
    let result = divide(10.0, 2.0)?;
    Ok(result * 2.0)
}
```

参见第 11 章：枚举与模式匹配、第 14 章：错误处理

### 3. 如何在结构体中存储引用？

**答**：需要添加生命周期标注。

**错误示例**：
```rust
struct User {
    name: &str,  // ❌ 缺少生命周期
}
```

**正确示例**：
```rust
struct User<'a> {
    name: &'a str,  // ✅
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

let name = String::from("Alice");
let user = User::new(&name);
```

**替代方案**：
```rust
// 使用 String 拥有所有权（推荐）
struct User {
    name: String,
}

// 使用 Cow 智能指针
use std::borrow::Cow;
struct User<'a> {
    name: Cow<'a, str>,
}
```

参见第 17 章：生命周期

### 4. 什么是 newtype 模式？

**答**：用结构体包装类型，创建新类型。

**用途**：
1. 类型安全
2. 实现 Trait
3. 避免孤儿规则

**示例**：
```rust
// 类型安全
struct Meters(u32);
struct Kilometers(u32);

let m = Meters(100);
let km = Kilometers(1);
// let total = m + km;  // ❌ 类型不匹配

// 实现 Trait
struct Wrapper(String);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Wrapped: {}", self.0)
    }
}

// 避免孤儿规则
struct MyVec(Vec<i32>);

impl MyVec {
    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
}
```

参见第 16 章：Trait

### 5. 如何处理不同类型的统一？

**答**：使用 Trait 对象或枚举。

**Trait 对象**：
```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { 3.14 * self.radius * self.radius }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

// 统一存储不同类型
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Rectangle { width: 2.0, height: 3.0 }),
];
```

**枚举**：
```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

let shapes = vec![
    Shape::Circle { radius: 1.0 },
    Shape::Rectangle { width: 2.0, height: 3.0 },
];
```

---

## 工具链问题

### 1. 如何安装和更新 Rust？

**答**：使用 rustup 工具。

**安装**：
```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 https://win.rustup.rs/
```

**更新**：
```bash
rustup update        # 更新 Rust 版本
rustup self update   # 更新 rustup 自身
```

**查看版本**：
```bash
rustc --version      # 编译器版本
cargo --version      # Cargo 版本
rustup show          # 显示已安装版本
```

参见第 01 章：安装与配置

### 2. 如何管理不同版本的 Rust？

**答**：使用 rustup 的工具链管理。

**常用命令**：
```bash
# 安装特定版本
rustup install 1.75.0
rustup install stable
rustup install nightly

# 切换默认版本
rustup default stable
rustup default 1.75.0

# 项目使用特定版本
cd my-project
rustup override set nightly

# 查看当前版本
rustup show
```

### 3. Cargo 常用命令有哪些？

**答**：以下是常用命令。

| 命令 | 说明 |
|------|------|
| `cargo new project_name` | 创建新项目 |
| `cargo init` | 在当前目录初始化项目 |
| `cargo build` | 编译项目 |
| `cargo run` | 编译并运行 |
| `cargo test` | 运行测试 |
| `cargo check` | 快速检查（不生成二进制） |
| `cargo doc --open` | 生成并打开文档 |
| `cargo add crate_name` | 添加依赖 |
| `cargo update` | 更新依赖 |
| `cargo publish` | 发布到 crates.io |
| `cargo clippy` | 代码检查 |
| `cargo fmt` | 格式化代码 |

参见第 02 章、第 21 章：Cargo 与 Crates

### 4. 如何添加和管理依赖？

**答**：使用 Cargo.toml 或 cargo add。

**手动添加**（Cargo.toml）：
```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
my-crate = { git = "https://github.com/user/repo" }
```

**命令添加**：
```bash
cargo add serde
cargo add tokio --features full
cargo add my-crate --git https://github.com/user/repo
```

**查看依赖树**：
```bash
cargo tree
```

**更新依赖**：
```bash
cargo update              # 更新所有依赖
cargo update -p serde     # 更新特定依赖
```

### 5. 如何配置开发环境？

**答**：推荐 VSCode + rust-analyzer。

**VSCode 扩展**：
1. rust-analyzer（必需）
2. CodeLLDB（调试）
3. Even Better Toml（Cargo.toml 支持）
4. crates（依赖版本提示）

**配置文件**（.vscode/settings.json）：
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true
}
```

**调试配置**（.vscode/launch.json）：
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "cargo": {
        "args": ["build", "--bin=app", "--package=app"],
        "filter": {
          "name": "app",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### 6. 如何格式化和检查代码？

**答**：使用 cargo fmt 和 cargo clippy。

**格式化**：
```bash
cargo fmt           # 格式化所有文件
cargo fmt -- --check # 检查格式（不修改）
```

**Clippy 检查**：
```bash
cargo clippy                # 运行 clippy
cargo clippy -- -W warnings # 显示所有警告
cargo clippy --fix          # 自动修复
```

**CI 配置**：
```yaml
# .github/workflows/ci.yml
- name: Check formatting
  run: cargo fmt -- --check

- name: Run clippy
  run: cargo clippy -- -D warnings
```

---

## 项目实战问题

### 1. 如何组织多文件项目？

**答**：使用模块系统。

**项目结构**：
```
my-project/
├── Cargo.toml
├── src/
│   ├── main.rs        # 二进制入口
│   ├── lib.rs         # 库入口
│   ├── config.rs      # 配置模块
│   ├── models/
│   │   ├── mod.rs     # 模块声明
│   │   ├── user.rs
│   │   └── post.rs
│   └── utils/
│       └── mod.rs
```

**模块声明**（src/lib.rs）：
```rust
pub mod config;
pub mod models;
pub mod utils;
```

**使用模块**（src/main.rs）：
```rust
use my_project::config::Config;
use my_project::models::user::User;

fn main() {
    let config = Config::new();
    let user = User::new("Alice");
}
```

参见第 20 章：包和模块

### 2. 如何处理配置文件？

**答**：使用 serde 和配置文件库。

**推荐库**：
- `config` - 通用配置管理
- `serde` - 序列化/反序列化

**示例**：
```rust
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

**config.toml**：
```toml
[database]
url = "postgres://localhost/mydb"
max_connections = 10

[server]
host = "0.0.0.0"
port = 8080
```

### 3. 如何进行错误处理？

**答**：使用 Result 和自定义错误类型。

**应用层错误**（推荐 anyhow）：
```rust
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    Ok(config)
}

fn main() -> Result<()> {
    let config = read_config()?;
    println!("Config loaded: {:?}", config);
    Ok(())
}
```

**库层错误**（推荐 thiserror）：
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, MyError>;
```

参见第 14 章：错误处理

### 4. 如何编写测试？

**答**：使用 cargo test。

**单元测试**：
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This should panic");
    }
    
    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

**集成测试**（tests/ 目录）：
```rust
// tests/integration_test.rs
use my_project::add;

#[test]
fn test_add_integration() {
    assert_eq!(add(2, 3), 5);
}
```

**运行测试**：
```bash
cargo test                  # 运行所有测试
cargo test test_add         # 运行特定测试
cargo test -- --nocapture   # 显示输出
cargo test -- --test-threads=1  # 单线程运行
```

参见第 28 章：测试与文档

### 5. 如何实现并发？

**答**：使用线程或 async/await。

**多线程**：
```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
```

**异步编程**（推荐 tokio）：
```rust
use tokio;

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(async {
        println!("Task 1");
    });
    
    let handle2 = tokio::spawn(async {
        println!("Task 2");
    });
    
    handle1.await.unwrap();
    handle2.await.unwrap();
}
```

参见第 23 章：并发编程

### 6. 如何处理命令行参数？

**答**：使用 clap 库。

**示例**：
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "myapp")]
#[command(about = "A CLI application", long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
    
    /// Number of threads
    #[arg(short, long, default_value_t = 4)]
    threads: u32,
    
    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("Input: {}", args.input);
    println!("Output: {:?}", args.output);
    println!("Threads: {}", args.threads);
    println!("Verbose: {}", args.verbose);
}
```

**使用**：
```bash
myapp -i input.txt -o output.txt -t 8 -v
myapp --input input.txt --verbose
myapp --help
```

参见第 26 章：命令行工具

---

## 性能优化问题

### 1. 如何优化 Rust 程序性能？

**答**：多个层面优化。

**编译优化**：
```toml
# Cargo.toml
[profile.release]
opt-level = 3        # 最高优化级别
lto = true          # 链接时优化
codegen-units = 1   # 单个代码生成单元
strip = true        # 移除符号信息
```

**算法优化**：
- 使用合适的数据结构（HashMap vs BTreeMap）
- 避免不必要的克隆（使用引用）
- 使用迭代器（零成本抽象）

**内存优化**：
- 使用 `Box<T>` 减少栈分配
- 预分配容量（Vec::with_capacity）
- 重用对象（避免频繁分配）

**并行优化**：
- 使用 Rayon 进行数据并行
- 使用 Tokio 进行异步 I/O
- 利用多核 CPU

### 2. 如何进行性能分析？

**答**：使用基准测试和性能分析工具。

**基准测试**（criterion）：
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

**性能分析工具**：
- `cargo flamegraph` - 火焰图
- `cargo instruments` - macOS 性能分析
- `perf` - Linux 性能分析

### 3. 如何避免性能陷阱？

**答**：注意常见陷阱。

| 陷阱 | 问题 | 解决方案 |
|------|------|----------|
| 过度克隆 | 不必要的数据复制 | 使用引用 |
| 字符串拼接 | 频繁分配 | 使用 String::with_capacity |
| 循环中分配 | 每次迭代分配 | 提前分配 |
| 锁竞争 | Mutex 阻塞 | 减少临界区 |
| 无效迭代器 | 中间集合 | 使用惰性求值 |

**示例**：
```rust
// ❌ 性能差
let mut s = String::new();
for i in 0..1000 {
    s.push_str(&i.to_string());  // 每次分配
}

// ✅ 性能好
let mut s = String::with_capacity(4000);
for i in 0..1000 {
    s.push_str(&i.to_string());
}

// ❌ 性能差
let v: Vec<i32> = (0..1000).collect();
let sum: i32 = v.iter().filter(|x| *x % 2 == 0).sum();

// ✅ 性能好
let sum: i32 = (0..1000).filter(|x| x % 2 == 0).sum();
```

---

## 更多资源

- [学习路径](./learning-paths.md) - 详细学习规划
- [前置知识图谱](./prerequisites.md) - 知识依赖关系
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 中文社区](https://rustcc.cn/)