## 再探 Hello World

### 逐行解析

**概念名称：** `fn main()` 是 Rust 程序的入口点，`println!` 是用于格式化输出的宏。

```
语法结构：
┌──────────────────────────────────────┐
│  fn main() {                          │
│      println!("格式串", 参数...);     │
│  }                                    │
│                                       │
│  fn       → 定义函数的关键字          │
│  main     → 程序入口（固定名称）      │
│  println! → 打印宏（! 表示宏）        │
│  {}       → 占位符，会被参数替换      │
└──────────────────────────────────────┘
```

### 最简示例

```rust
fn main() {
    println!("Hello, World!");
}
```

### 详细示例

```rust
fn main() {
    // 简单打印
    println!("Hello, World!");

    // 打印变量
    let name = "Rust";
    println!("Hello, {}!", name);

    // 打印多个变量
    let x = 10;
    let y = 20;
    println!("x = {}, y = {}", x, y);

    // 调试输出
    let numbers = vec![1, 2, 3];
    println!("numbers = {:?}", numbers);
}
```

**关键代码说明：**

| 代码 | 含义 | 为什么这样写 |
|------|------|-------------|
| `fn main()` | 程序入口函数 | Rust 规定可执行文件必须有 main 函数 |
| `println!` | 打印并换行的宏 | `!` 表示这是宏而非函数，编译时展开 |
| `"{}"` | 占位符 | 会被后续参数按顺序替换，类型自动推导 |
| `{:?}` | 调试占位符 | 用于打印实现了 `Debug` trait 的类型 |

---

## println! 宏详解

### 基本用法

```rust
fn main() {
    // 简单打印
    println!("Hello, World!");

    // 打印变量
    let name = "Rust";
    println!("Hello, {}!", name);

    // 打印多个变量
    let x = 10;
    let y = 20;
    println!("x = {}, y = {}", x, y);
}
```

### 格式化占位符

```rust
fn main() {
    // {} - 默认格式
    println!("{}", 42);           // 42
    println!("{}", "hello");      // hello

    // {:?} - 调试格式
    println!("{:?}", vec![1, 2, 3]);  // [1, 2, 3]

    // 数字格式化
    let x = 42;
    println!("{:05}", x);     // 00042（5 位，前导零）
    println!("{:x}", x);      // 2a（十六进制）

    // 浮点数格式化
    let pi = std::f64::consts::PI;
    println!("{:.2}", pi);    // 3.14（保留 2 位小数）

    // 命名参数
    println!("{name} is {age} years old",
             name = "Alice", age = 25);

    // 位置参数
    println!("{0} {1} {0}", "hello", "world");
    // 输出：hello world hello
}
```

**关键代码说明：**

| 代码 | 含义 | 为什么这样写 |
|------|------|-------------|
| `{}` | 默认占位符 | 自动推导类型，适用于基本类型 |
| `{:?}` | 调试占位符 | 需要类型实现 `Debug` trait |
| `{:.2}` | 精度控制 | 浮点数保留 2 位小数 |
| `{name}` | 命名参数 | 提高可读性，适合多参数场景 |

### print! vs println!

**概念名称：** `print!` 不换行，`println!` 自动换行，`eprintln!` 输出到标准错误。

```
语法结构：
┌──────────────────────────────────────┐
│  println!("格式串", 参数...);  // 换行 │
│  print!("格式串", 参数...);    // 不换行│
│  eprintln!("格式串", 参数...); // 错误 │
└──────────────────────────────────────┘
```

### 最简示例

```rust
fn main() {
    println!("自动换行");
    print!("不");
    print!("换行");
    println!();  // 手动换行
}
```

### 详细示例

```rust
fn main() {
    // println! 自动换行
    println!("第一行");
    println!("第二行");

    // print! 不换行
    print!("Hello, ");
    print!("World!");
    println!();  // 手动换行

    // eprintln! 打印到标准错误
    eprintln!("这是错误信息");
}
```

**为什么用它？**

```rust
// 没有：手动处理换行符，容易遗漏
print!("第一行\n");
print!("第二行\n");

// 有：自动换行，代码更简洁
println!("第一行");
println!("第二行");

// 错误输出到 stderr，便于日志分离
eprintln!("错误：文件不存在");  // 可被重定向分离
```

---
