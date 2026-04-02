## 26.3 使用 Clap 解析参数

### Clap derive 方式（推荐）

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "my_cli")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "一个简单的 CLI 工具", long_about = None)]
struct Args {
    /// 要处理的文件
    #[arg(short, long)]
    file: Option<String>,

    /// 是否详细输出
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// 输出格式
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("详细模式启用");
    }

    if let Some(file) = args.file {
        println!("处理文件：{}", file);
    }

    println!("输出格式：{}", args.format);
}
```

### 运行效果

```bash
# 查看帮助
cargo run -- --help

# 使用参数
cargo run -- --file test.txt --verbose
cargo run -- -f test.txt -v
cargo run -- --file test.txt --format json
```

### 帮助输出

```
一个简单的 CLI 工具

Usage: my_cli [OPTIONS]

Options:
  -f, --file <FILE>        要处理的文件
  -v, --verbose            是否详细输出
      --format <FORMAT>    输出格式 [default: text]
  -h, --help               Print help
  -V, --version            Print version
```

---

---

## 下一步

[子命令](../4-子命令.md)