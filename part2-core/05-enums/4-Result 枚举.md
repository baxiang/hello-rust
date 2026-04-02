## 11.4 Result 枚举

### 什么是 Result？

```rust
// Rust 标准库中的定义
enum Result<T, E> {
    Ok(T),   // 成功
    Err(E),  // 错误
}
```

### Result 基本用法

```rust
use std::fs::File;
use std::io::Error;

fn open_file() -> Result<File, Error> {
    File::open("hello.txt")
}

fn main() {
    // 处理结果
    match open_file() {
        Ok(file) => println!("文件打开成功"),
        Err(e) => println!("打开文件失败：{:?}", e),
    }

    // 使用 unwrap（不推荐用于生产代码）
    // let file = open_file().unwrap();

    // 使用 expect（可以自定义错误信息）
    // let file = open_file().expect("无法打开文件");
}
```

### Result 常用方法

```rust
fn main() {
    let ok_result: Result<i32, &str> = Ok(5);
    let err_result: Result<i32, &str> = Err("错误");

    // is_ok / is_err
    println!("{}", ok_result.is_ok());   // true
    println!("{}", err_result.is_err()); // true

    // ok / err - 转换为 Option
    println!("{:?}", ok_result.ok());   // Some(5)
    println!("{:?}", err_result.ok());  // None

    // unwrap / unwrap_err
    println!("{}", ok_result.unwrap());  // 5
    // println!("{}", err_result.unwrap());  // panic!

    // unwrap_or / unwrap_or_else
    println!("{}", ok_result.unwrap_or(0));   // 5
    println!("{}", err_result.unwrap_or(0));  // 0

    // map - 转换成功值
    let doubled = ok_result.map(|x| x * 2);
    println!("{:?}", doubled);  // Ok(10)

    // map_err - 转换错误值
    let mapped = err_result.map_err(|e| format!("错误：{}", e));
    println!("{:?}", mapped);

    // and_then - 链式操作
    let result: Result<i32, &str> = Ok(5);
    let chained = result.and_then(|x| Ok(x * 2));
    println!("{:?}", chained);  // Ok(10)
}
```

### Result 实际模式

```rust
use std::fs::File;
use std::io::{self, Read};

// 模式 1：传播错误
fn read_file_contents() -> io::Result<String> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 模式 2：错误转换
fn parse_number(s: &str) -> Result<i32, String> {
    s.trim().parse::<i32>()
        .map_err(|e| format!("解析失败：{}", e))
}

// 模式 3：提供默认值
fn get_port(config: Option<u16>) -> u16 {
    config.unwrap_or(8080)
}

fn main() {
    match read_file_contents() {
        Ok(contents) => println!("文件内容：{}", contents),
        Err(e) => println!("读取失败：{}", e),
    }
}
```

### ? 操作符

```rust
use std::fs::File;
use std::io::{self, Read};

// 使用?传播错误
fn read_file() -> io::Result<String> {
    let mut file = File::open("hello.txt")?;  // 失败时返回 Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 等同于
fn read_file_verbose() -> io::Result<String> {
    let mut file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn main() {
    // main 函数也可以使用?
    println!("文件内容：{}", read_file().unwrap_or_default());
}
```

---

---

## 下一步

[match 控制流](../5-match 控制流.md)