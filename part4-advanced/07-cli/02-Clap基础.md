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






## 26.4 子命令

### 实现子命令

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "我的应用", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新项目
    Add {
        /// 项目名称
        #[arg(short, long)]
        name: String,

        /// 项目描述
        #[arg(short, long, default_value = "")]
        description: String,
    },
    /// 列出所有项目
    List {
        /// 只显示已完成的
        #[arg(long, default_value_t = false)]
        done: bool,
    },
    /// 删除项目
    Remove {
        /// 项目 ID
        #[arg(short, long)]
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, description } => {
            println!("添加：{} - {}", name, description);
        }
        Commands::List { done } => {
            if done {
                println!("列出已完成项目");
            } else {
                println!("列出所有项目");
            }
        }
        Commands::Remove { id } => {
            println!("删除项目：{}", id);
        }
    }
}
```

### 子命令帮助

```bash
# 查看子命令帮助
cargo run -- add --help
cargo run -- list --help

# 使用子命令
cargo run -- add --name "Project A" --description "My project"
cargo run -- list
cargo run -- remove --id 1
```







