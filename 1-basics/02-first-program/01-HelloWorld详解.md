## 再探 Hello World

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







