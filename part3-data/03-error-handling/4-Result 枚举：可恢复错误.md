## 14.4 Result 枚举：可恢复错误

### Result 的定义

```rust
// 标准库中的定义
#[must_use]
pub enum Result<T, E> {
    Ok(T),   // 成功，包含值
    Err(E),  // 失败，包含错误
}
```

```
┌─────────────────────────────────────────────────────┐
│              Result 详解                             │
├─────────────────────────────────────────────────────┤
│                                                     │
│  泛型参数：                                          │
│  ├── T - 成功时返回的类型                           │
│  └── E - 错误的类型                                 │
│                                                     │
│  #[must_use] 属性：                                  │
│  • 强制处理 Result                                   │
│  • 忽略未处理的 Result 会产生警告                   │
│  • 确保错误不会被无意中忽略                         │
│                                                     │
│  示例：                                              │
│  Result<String, io::Error>      // 成功返回 String   │
│  Result<i32, ParseError>        // 成功返回 i32      │
│  Result<(), Box<dyn Error>>     // 成功无返回值      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

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




