## 26.5 完整示例：Grep 工具

### Cargo.toml

```toml
[package]
name = "minigrep"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
colored = "2"
```

### main.rs

```rust
use clap::Parser;
use colored::*;
use std::fs;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "minigrep")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "一个简单的 grep 工具", long_about = None)]
struct Args {
    /// 搜索模式
    #[arg(short, long)]
    pattern: String,

    /// 文件路径
    #[arg(short, long)]
    file: String,

    /// 忽略大小写
    #[arg(short = 'i', long, default_value_t = false)]
    ignore_case: bool,

    /// 显示行号
    #[arg(short = 'n', long, default_value_t = false)]
    line_number: bool,

    /// 显示上下文行数
    #[arg(short = 'C', long, default_value_t = 0)]
    context: usize,
}

struct Config {
    pattern: String,
    file: String,
    ignore_case: bool,
    line_number: bool,
    context: usize,
}

impl Config {
    fn new(args: Args) -> Result<Self, &'static str> {
        if args.pattern.is_empty() {
            return Err("模式不能为空");
        }

        Ok(Config {
            pattern: args.pattern,
            file: args.file,
            ignore_case: args.ignore_case,
            line_number: args.line_number,
            context: args.context,
        })
    }
}

fn search<'a>(pattern: &str, content: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    if ignore_case {
        let pattern_lower = pattern.to_lowercase();
        content
            .lines()
            .enumerate()
            .filter(|(_, line)| line.to_lowercase().contains(&pattern_lower))
            .map(|(i, line)| (i + 1, line))
            .collect()
    } else {
        content
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(pattern))
            .map(|(i, line)| (i + 1, line))
            .collect()
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&config.file)?;
    let results = search(&config.pattern, &content, config.ignore_case);

    if results.is_empty() {
        return Ok(());
    }

    for (line_num, line) in results {
        let highlighted = line.replace(
            &config.pattern,
            &config.pattern.red().bold().to_string(),
        );

        if config.line_number {
            println!("{}:{}", line_num.to_string().red(), highlighted);
        } else {
            println!("{}", highlighted);
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let config = match Config::new(args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("配置错误：{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("程序错误：{}", e);
        process::exit(1);
    }
}
```

### 运行

```bash
# 基本搜索
cargo run -- -p "hello" -f test.txt

# 忽略大小写
cargo run -- -p "Hello" -f test.txt -i

# 显示行号和高亮
cargo run -- -p "error" -f log.txt -n

# 显示上下文
cargo run -- -p "error" -f log.txt -C 2
```

---

---

## 下一步

[交互式 CLI](../6-交互式 CLI.md)