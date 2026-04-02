## 14.6 在 main 函数中使用 Result

### main 返回 Result

```rust
use std::error::Error;
use std::fs::File;
use std::io;

// main 可以返回 Result
fn main() -> Result<(), io::Error> {
    let file = File::open("hello.txt")?;
    println!("文件打开成功！");
    Ok(())
}
```

### 使用 Box<dyn Error>

```rust
use std::error::Error;
use std::fs::File;
use std::io;

// 使用 Box<dyn Error>可以返回任何错误类型
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("hello.txt")?;
    let content = std::fs::read_to_string("config.json")?;
    let number: i32 = "not a number".parse()?;
    Ok(())
}
```

### 使用 anyhow 简化错误处理

```toml
# Cargo.toml
[dependencies]
anyhow = "1.0"
```

```rust
use anyhow::{Result, Context, anyhow, bail};

// 简单的 Result 类型别名
fn main() -> Result<()> {
    // 基本使用
    let content = std::fs::read_to_string("hello.txt")?;

    // 添加上下文
    let content = std::fs::read_to_string("hello.txt")
        .context("Failed to read hello.txt")?;

    // 创建错误
    if content.is_empty() {
        bail!("文件为空");
    }

    // 条件错误
    let config: Option<String> = None;
    let config = config.ok_or_else(|| anyhow!("配置不存在"))?;

    Ok(())
}
```

### anyhow 的高级用法

```rust
use anyhow::{Result, Context, anyhow};

fn main() -> Result<()> {
    // 链式上下文
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    let value: i32 = config.trim().parse()
        .context("Failed to parse config as integer")?;

    // 包装错误
    let result: Result<i32> = (|| {
        let content = std::fs::read_to_string("data.txt")?;
        Ok(content.parse::<i32>()?)
    })()
    .context("Failed to process data file");

    // 使用 with_context 添加动态上下文
    let path = "hello.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;

    Ok(())
}
```

---

---

## 下一步

[自定义错误类型](../7-自定义错误类型.md)

---

## 返回

[返回目录](../../README.md)