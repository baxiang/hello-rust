## 26.6 交互式 CLI

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

---

---

## 下一步

[彩色输出](../7-彩色输出.md)