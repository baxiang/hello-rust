## 交互式 CLI

### 基本输入

```rust
use std::io::{self, Write};

fn main() {
    // 读取单行输入
    print!("请输入您的名字：");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("你好，{}!", name.trim());

    // 菜单选择
    loop {
        println!("\n请选择：");
        println!("1. 选项 A");
        println!("2. 选项 B");
        println!("3. 退出");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => println!("选择了 A"),
            "2" => println!("选择了 B"),
            "3" => {
                println!("再见！");
                break;
            }
            _ => println!("无效选择，请重新输入"),
        }
    }
}
```

### 使用 Dialoguer

```toml
[dependencies]
dialoguer = "0.11"
indicatif = "0.17"
```

```rust
use dialoguer::{Input, Confirm, Select, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 文本输入
    let name: String = Input::new()
        .with_prompt("您的名字")
        .default("Guest".to_string())
        .interact_text()?;

    println!("你好，{}!", name);

    // 确认
    let confirmed = Confirm::new()
        .with_prompt("确认继续？")
        .default(true)
        .interact()?;

    if !confirmed {
        println!("已取消");
        return Ok(());
    }

    // 单选菜单
    let items = vec!["选项 A", "选项 B", "选项 C"];
    let selection = Select::new()
        .with_prompt("选择一项")
        .default(0)
        .items(&items)
        .interact()?;

    println!("选择了：{}", items[selection]);

    // 多选
    let items = vec!["功能 1", "功能 2", "功能 3", "功能 4"];
    let selections = MultiSelect::new()
        .with_prompt("选择多项")
        .items(&items)
        .interact()?;

    println!("已选：{:?}", selections);

    // 进度条
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")?
            .progress_chars("#>-"),
    );

    for i in 0..100 {
        thread::sleep(Duration::from_millis(10));
        pb.set_position(i);
    }
    pb.finish_with_message("完成!");

    Ok(())
}
```






## 彩色输出

### 使用 Colored

```rust
use colored::*;

fn main() {
    // 基本颜色
    println!("{}", "红色".red());
    println!("{}", "绿色".green());
    println!("{}", "蓝色".blue());
    println!("{}", "黄色".yellow());

    // 组合样式
    println!("{}", "粗体红色".red().bold());
    println!("{}", "下划线".underline());
    println!("{}", "斜体".italic());
    println!("{}", "闪烁".blink());

    // 背景色
    println!("{}", "白底黑字".black().on_white());
    println!("{}", "黄底黑字".black().on_yellow());

    // 条件颜色
    let success = true;
    println!(
        "{}",
        if success { "成功".green().bold() } else { "失败".red().bold() }
    );

    // 错误样式
    eprintln!("{}", "错误：文件未找到".red().bold());
    eprintln!("{}", "警告：配置已过时".yellow());

    // 信息样式
    println!("{}", "ℹ 信息：处理中...".blue());
    println!("{}", "✓ 完成：任务成功".green());
    println!("{}", "✗ 失败：操作错误".red());
}
```

### 禁用颜色

```rust
// 环境变量控制
// NO_COLOR=1 cargo run

// 代码中控制
colored::control::set_override(false);  // 禁用
colored::control::set_override(true);   // 启用
```






## 错误处理最佳实践

### 使用 Anyhow

```rust
use anyhow::{Context, Result, bail, ensure};
use std::fs;
use std::path::Path;

fn read_file(path: &str) -> Result<String> {
    // 检查文件存在
    if !Path::new(path).exists() {
        bail!("文件不存在：{}", path);
    }

    // 读取文件，添加上下文
    let content = fs::read_to_string(path)
        .with_context(|| format!("无法读取文件：{}", path))?;

    Ok(content)
}

fn process_data(value: i32) -> Result<()> {
    // 条件检查
    ensure!(value > 0, "值必须是正数，得到：{}", value);

    // 处理逻辑...
    Ok(())
}

fn main() -> Result<()> {
    let content = read_file("test.txt")?;
    println!("{}", content);
    Ok(())
}
```

### 自定义错误类型

```rust
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    NotFound(String),
    InvalidFormat(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO 错误：{}", e),
            AppError::NotFound(path) => write!(f, "未找到：{}", path),
            AppError::InvalidFormat(msg) => write!(f, "格式错误：{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

fn read_config(path: &str) -> Result<String, AppError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if !content.starts_with("{") {
        return Err(AppError::InvalidFormat("配置必须是 JSON".to_string()));
    }

    Ok(content)
}

fn main() {
    match read_config("config.json") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("错误：{}", e),
    }
}
```






## 表格输出

### 使用 Tabled

```toml
[dependencies]
tabled = "0.14"
```

```rust
use tabled::{Table, Tabled, settings::{Style, Color}};

#[derive(Tabled)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let users = vec![
        User { id: 1, name: String::from("Alice"), email: String::from("alice@example.com") },
        User { id: 2, name: String::from("Bob"), email: String::from("bob@example.com") },
        User { id: 3, name: String::from("Charlie"), email: String::from("charlie@example.com") },
    ];

    let mut table = Table::new(&users);
    table.with(Style::modern());

    println!("{}", table);
}
```

输出：
```
┌────┬─────────┬─────────────────────────┐
│ id │  name   │          email          │
├────┼─────────┼─────────────────────────┤
│ 1  │  Alice  │ alice@example.com       │
│ 2  │  Bob    │ bob@example.com         │
│ 3  │ Charlie │ charlie@example.com     │
└────┴─────────┴─────────────────────────┘
```







