# Result 枚举：可恢复错误

> 掌握 Result<T, E> 的基本用法，学会使用 match、unwrap_or 和 ? 操作符处理可恢复错误。

## Result 枚举：可恢复错误

**概念名称：** `Result<T, E>` 表示可能成功（Ok）或失败（Err）的操作。

```
语法结构：
┌──────────────────────────────────────┐
│  enum Result<T, E> {                 │
│      Ok(T),      // 成功，包含值      │
│      Err(E),     // 失败，包含错误    │
│  }                                   │
│                                       │
│  处理模式：                            │
│  match result {                      │
│      Ok(v) => ...                    │
│      Err(e) => ...                   │
│  }                                   │
│                                       │
│  简化：? 操作符自动传播错误            │
│  fn foo() -> Result<T, E> {          │
│      let x = risky_op()?;            │
│      Ok(x)                           │
│  }                                   │
└──────────────────────────────────────┘
```

### 最简示例

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(r) => println!("结果：{}", r),
        Err(e) => println!("错误：{}", e),
    }
}
```

### 详细示例

```rust
use std::fs::File;
use std::io::{self, Read};

// 使用 ? 传播错误
fn read_file_contents() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // unwrap_or - 提供默认值
    let content = std::fs::read_to_string("hello.txt")
        .unwrap_or_default();  // 失败返回空字符串

    println!("内容：{}", content);
    
    // 链式使用 ?
    match read_file_contents() {
        Ok(c) => println!("读取成功：{} 字符", c.len()),
        Err(e) => println!("读取失败：{}", e),
    }
}
```

**关键代码说明：**

| 代码 | 含义 | 为什么这样写 |
|------|------|-------------|
| `Result<T, E>` | 成功/失败的结果类型 | 将错误作为值，强制调用者处理 |
| `Ok(a / b)` | 成功分支 | 包含有效计算结果 |
| `Err(...)` | 失败分支 | 包含错误描述信息 |
| `?` 操作符 | 错误传播语法糖 | 自动返回 Err，大幅简化代码 |
| `.unwrap_or_default()` | 失败时使用默认值 | 适合可接受空结果的场景 |

### 处理 Result 的五种方法

```rust
use std::fs::File;
use std::io::Error;

fn main() {
    // 方法 1：match 表达式（最灵活）
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("打开文件失败：{}", error);
            return;
        }
    };

    // 方法 2：unwrap()（最简单，但可能 panic）
    let file = File::open("hello.txt").unwrap();

    // 方法 3：expect()（推荐，可以自定义错误信息）
    let file = File::open("hello.txt")
        .expect("Failed to open hello.txt");

    // 方法 4：if let（只关心成功情况）
    if let Ok(file) = File::open("hello.txt") {
        // 使用 file
    } else {
        // 处理错误
    }

    // 方法 5：? 操作符（传播错误）
    let result = open_file()?;
}

fn open_file() -> Result<File, Error> {
    File::open("hello.txt")
}
```

### 优雅地处理特定错误

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = match File::open("hello.txt") {
        Ok(file) => file,

        Err(error) => match error.kind() {
            // 文件不存在：创建它
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("文件已创建");
                        fc
                    }
                    Err(e) => {
                        panic!("创建文件失败：{:?}", e);
                    }
                }
            }

            // 其他错误：直接 panic
            other_error => {
                panic!("打开文件失败：{:?}", other_error);
            }
        },
    };
}
```

### 使用 unwrap_or 系列方法

```rust
use std::fs::File;
use std::io;

fn main() {
    // unwrap_or - 提供默认值
    let file = File::open("hello.txt")
        .unwrap_or_else(|_| File::create("hello.txt").unwrap());

    // unwrap_or_default - 使用默认值
    let content = std::fs::read_to_string("hello.txt")
        .unwrap_or_default();  // 失败返回空字符串

    // unwrap_or_else - 使用闭包提供默认值
    let config = load_config()
        .unwrap_or_else(|_| Config::default());
}

fn load_config() -> Result<Config, io::Error> {
    // ...
    unimplemented!()
}

struct Config {
    // ...
}

impl Config {
    fn default() -> Self {
        Config { /* 默认配置 */ }
        unimplemented!()
    }
}
```






---

## ? 操作符：错误传播

### ? 的基本用法

```rust
use std::fs::File;
use std::io::{self, Read};

// 使用?传播错误
fn read_file_contents() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;  // 失败时返回 Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 等价于嵌套 match
fn read_file_verbose() -> Result<String, io::Error> {
    let mut file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut contents = match {
        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Ok(_) => Ok(string),
            Err(e) => Err(e),
        }
    } {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    Ok(contents)
}
```

### ? 的工作原理

```
? 操作符的展开：

表达式                展开后
file.open()?    →     match file.open() {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                      }

some_opt?       →     match some_opt {
                        Some(val) => val,
                        None => return None,
                      }

关键点：
• ? 只能在返回 Result 或 Option 的函数中使用
• ? 会自动进行错误类型转换（通过 From trait）
• ? 可以链式调用
```

### ? 的链式调用

```rust
use std::fs;
use std::io;

// 链式使用 ?
fn read_and_parse() -> Result<i32, io::Error> {
    let content = fs::read_to_string("number.txt")?;
    let number = content.trim().parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(number)
}

// 或者一行搞定
fn read_number() -> Result<i32, io::Error> {
    fs::read_to_string("number.txt")?
        .trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
```

### ? 与 Option 的配合

```rust
fn main() {
    // ? 也可以用于 Option
    fn get_first_char(s: Option<&str>) -> Option<char> {
        let first = s?.chars().next()?;
        Some(first)
    }

    println!("{:?}", get_first_char(Some("hello")));  // Some('h')
    println!("{:?}", get_first_char(Some("")));       // None
    println!("{:?}", get_first_char(None));           // None
}




---

## 小结

- `Result<T, E>` 用于处理可恢复的错误，强制调用者处理
- 处理 Result 的五种方法：match、unwrap、expect、if let、?
- `?` 操作符自动传播错误，是 Rust 错误处理的精髓
- `unwrap_or` 系列方法提供默认值，适合可接受空结果的场景
- 可以同时用于 Result 和 Option 类型

## 练习题

详见：[练习题](../../exercises/03-error-handling.md)
```




