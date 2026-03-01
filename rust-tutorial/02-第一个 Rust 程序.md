# 第 02 章：深入理解 Rust 程序

> 从 Hello World 开始，理解 Rust 的核心概念

---

## 2.1 再探 Hello World

### 逐行解析

```rust
fn main() {
    println!("Hello, World!");
}
```

让我们深入理解每一部分：

#### `fn` - 函数定义

```rust
// fn 是 function 的缩写
fn 函数名 (参数) -> 返回值 {
    // 函数体
}
```

#### `main` 函数

```rust
// main 是程序的入口点
// 每个可执行程序必须有且只有一个 main 函数
fn main() {
    // 程序从这里开始执行
}
```

#### `println!` - 打印宏

```rust
// 注意感叹号！这表示这是一个宏（macro）
// 宏在编译时展开，可以生成代码
// 函数在运行时调用

println!("格式字符串，{}", 参数);
```

**宏 vs 函数**：

| 特性 | 宏 | 函数 |
|------|-----|------|
| 调用 | `macro!()` | `function()` |
| 展开时机 | 编译时 | 运行时 |
| 参数类型 | 灵活 | 固定 |
| 性能 | 无调用开销 | 有调用开销 |

---

## 2.2 println! 宏详解

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

    // 换行
    println!("第一行");
    println!("第二行");

    // 或者用 \n
    println!("第一行\n第二行");
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

    // {:#?} - 美化调试格式
    println!("{:#?}", vec![1, 2, 3]);
    // 输出：
    // [
    //     1,
    //     2,
    //     3,
    // ]

    // 数字格式化
    let x = 42;
    println!("{}", x);        // 42
    println!("{:05}", x);     // 00042（5 位，前导零）
    println!("{:x}", x);      // 2a（十六进制）
    println!("{:X}", x);      // 2A（大写十六进制）
    println!("{:b}", x);      // 101010（二进制）
    println!("{:o}", x);      // 52（八进制）

    // 浮点数格式化
    let pi = 3.1415926;
    println!("{}", pi);       // 3.1415926
    println!("{:.2}", pi);    // 3.14（保留 2 位小数）
    println!("{:8.2}", pi);   //     3.14（总宽 8）

    // 对齐
    println!("{:<10}", "左对齐");  // 左对齐
    println!("{:>10}", "右对齐");  //     右对齐
    println!("{:^10}", "居中");    //    居中

    // 命名参数
    println!("{name} is {age} years old",
             name = "Alice", age = 25);

    // 位置参数
    println!("{0} {1} {0}", "hello", "world");
    // 输出：hello world hello
}
```

### print! vs println!

```rust
fn main() {
    // println! 自动换行
    println!("第一行");
    println!("第二行");

    // print! 不换行
    print!("Hello, ");
    print!("World!");
    print!("\n");  // 手动换行

    // eprintln! 打印到标准错误
    eprintln!("这是错误信息");

    // eprint! 不换行的错误打印
    eprint!("错误：");
    eprintln!("something went wrong");
}
```

---

## 2.3 变量与赋值

### let 绑定

```rust
fn main() {
    // 声明并初始化变量
    let x = 5;

    // 变量名必须是蛇形命名（snake_case）
    let my_variable = 10;
    let another_one = 20;

    // 变量一旦赋值，默认不可变
    let y = 100;
    // y = 200;  // ❌ 错误！不能重新赋值

    // 使用 mut 声明可变变量
    let mut z = 300;
    z = 400;  // ✅ 正确
    println!("z = {}", z);
}
```

### 变量类型推断

```rust
fn main() {
    // Rust 可以推断类型
    let x = 42;           // 推断为 i32
    let y = 3.14;         // 推断为 f64
    let s = "hello";      // 推断为 &str
    let b = true;         // 推断为 bool

    // 显式标注类型
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;

    // 何时需要显式标注？
    // 1. 编译器无法推断时
    // 2. 需要特定类型时
    let num = 42;         // i32（默认）
    let big: i64 = 42;    // 显式指定 i64
}
```

### 常量与静态变量

```rust
// 常量（必须标注类型）
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

// 静态变量（整个程序生命周期）
static LANGUAGE: &str = "Rust";

fn main() {
    println!("Max points: {}", MAX_POINTS);
    println!("PI: {}", PI);
    println!("Language: {}", LANGUAGE);
}
```

### 变量遮蔽（Shadowing）

```rust
fn main() {
    // 遮蔽允许重新使用变量名
    let x = 5;
    let x = x + 1;      // 新的 x 遮蔽了旧的 x
    let x = x * 2;

    println!("x = {}", x);  // 12

    // 遮蔽可以改变类型
    let spaces = "   ";     // &str 类型
    let spaces = spaces.len();  // usize 类型

    println!("spaces = {}", spaces);  // 3

    // 遮蔽 vs mut
    // mut: 修改值，类型不变
    let mut count = 0;
    count += 1;

    // shadowing: 新值，可以改类型
    let count = "done";
}
```

---

## 2.4 注释

```rust
fn main() {
    // 单行注释：两个斜杠

    /*
       多行注释
       可以跨越多行
    */

    // 文档注释（生成 API 文档）
    /// 这是函数的文档注释
    /// 支持 Markdown 格式
    ///
    /// # Examples
    ///
    /// ```
    /// let result = add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    //! 模块级文档注释
    // 通常放在文件开头
}
```

---

## 2.5 完整示例程序

### 示例 1：个人信息卡片

```rust
fn main() {
    // 定义变量
    let name = "张三";
    let age = 25;
    let height = 175.5;  // 厘米
    let city = "北京";

    // 打印个人信息
    println!("╔════════════════════════╗");
    println!("║     个人信息卡片       ║");
    println!("╠════════════════════════╣");
    println!("║ 姓名：{: <12} ║", name);
    println!("║ 年龄：{: <12} ║", age);
    println!("║ 身高：{: <12.1} ║", height);
    println!("║ 城市：{: <12} ║", city);
    println!("╚════════════════════════╝");
}
```

运行结果：
```
╔════════════════════════╗
║     个人信息卡片       ║
╠════════════════════════╣
║ 姓名：张三         ║
║ 年龄：25           ║
║ 身高：175.5        ║
║ 城市：北京         ║
╚════════════════════════╝
```

### 示例 2：简单计算器

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("简单计算器");
    println!("a = {}, b = {}", a, b);
    println!("─────────────────");

    // 基本运算
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);      // 整数除法：3
    println!("a % b = {}", a % b);      // 取余：1

    // 浮点数运算
    let x = 10.0;
    let y = 3.0;
    println!("\n浮点数运算");
    println!("x / y = {:.4}", x / y);   // 3.3333...

    // 位运算
    println!("\n位运算");
    println!("a << 1 = {}", a << 1);    // 左移：20
    println!("a >> 1 = {}", a >> 1);    // 右移：5
    println!("a & b = {}", a & b);      // 按位与：2
    println!("a | b = {}", a | b);      // 按位或：11
    println!("a ^ b = {}", a ^ b);      // 按位异或：9
}
```

### 示例 3：温度转换器

```rust
fn main() {
    println!("温度转换器");
    println!("────────────────────────────────");
    println!("{:^10} | {:^10} | {:^10}", "摄氏度", "华氏度", "开尔文");
    println!("{:-^46}", "");

    // 打印温度表
    for celsius in 0..=100 {
        if celsius % 10 == 0 {
            let fahrenheit = celsius as f64 * 9.0 / 5.0 + 32.0;
            let kelvin = celsius as f64 + 273.15;
            println!("{:^10} | {:^10.1} | {:^10.2}",
                     celsius, fahrenheit, kelvin);
        }
    }
}
```

---

## 2.6 动手实践

### 练习 1：格式化输出

创建一个程序，打印以下图案：

```
    *
   ***
  *****
 *******
*********
```

<details>
<summary>点击查看答案</summary>

```rust
fn main() {
    for i in 1..=5 {
        let spaces = " ".repeat(5 - i);
        let stars = "*".repeat(2 * i - 1);
        println!("{}{}", spaces, stars);
    }
}
```

</details>

### 练习 2：变量实验

创建一个程序，尝试以下操作：

```rust
fn main() {
    // 1. 创建一个不可变变量，尝试修改它（会报错）
    let x = 10;
    // x = 20;  // 取消注释看看错误

    // 2. 创建一个可变变量，修改它
    let mut y = 10;
    y = 20;
    println!("y = {}", y);

    // 3. 使用遮蔽改变类型
    let s = "hello";
    let s = s.len();
    println!("s = {}", s);

    // 4. 使用常量
    const MAX: i32 = 100;
    println!("MAX = {}", MAX);
}
```

### 练习 3：个人信息生成器

创建一个程序，输出你的个人信息，包括：
- 姓名
- 年龄
- 爱好
- 学习目标

使用不同的格式化选项。

---

## 2.7 常见错误

### 错误 1：修改变量忘记 mut

```rust
fn main() {
    let x = 10;
    // x = 20;  // ❌ 错误

    let mut y = 10;  // ✅ 正确
    y = 20;
}
```

错误信息：
```
error[E0384]: cannot assign twice to immutable variable `x`
```

### 错误 2：变量未初始化就使用

```rust
fn main() {
    let x: i32;
    // println!("{}", x);  // ❌ 错误：未使用
}
```

### 错误 3：类型推断失败

```rust
fn main() {
    // let x;  // ❌ 无法推断类型
    // x = 10;

    let x: i32;  // ✅ 显式标注
    x = 10;
}
```

---

## 2.8 调试技巧

### 使用 dbg! 宏

```rust
fn main() {
    let x = 10;
    let y = 20;

    // dbg! 打印表达式和结果
    let sum = dbg!(x + y);

    // 输出：
    // [src/main.rs:5] x + y = 30

    // 可以链式使用
    let result = dbg!(dbg!(x * 2) + dbg!(y * 3));
}
```

### 使用 cargo expand

安装：
```bash
cargo install cargo-expand
```

查看宏展开：
```bash
cargo expand
```

---

## 2.9 小结

本章我们学习了：

- ✅ `println!` 宏的详细用法
- ✅ 变量声明和可变性
- ✅ 类型推断和显式标注
- ✅ 常量和静态变量
- ✅ 变量遮蔽
- ✅ 注释的写法

### 关键概念

| 概念 | 说明 |
|------|------|
| `let` | 声明变量 |
| `let mut` | 声明可变变量 |
| `const` | 常量（必须标注类型） |
| `println!` | 打印宏（带换行）
| `print!` | 打印宏（不带换行） |
| 遮蔽 | 重新使用变量名 |

---

## 下一章

[第 03 章：变量与可变性](03-变量与可变性.md)
