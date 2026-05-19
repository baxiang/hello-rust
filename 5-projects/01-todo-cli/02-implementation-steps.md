## 第一步：创建项目

```bash
cargo new todo-cli
cd todo-cli
```

### Cargo.toml

```toml
[package]
name = "todo-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
colored = "2"
```






## 第二步：定义数据结构

### src/todo.rs

```rust
use serde::{Deserialize, Serialize};

/// 待办事项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// 唯一 ID
    pub id: u32,
    /// 任务内容
    pub content: String,
    /// 是否完成
    pub done: bool,
    /// 创建时间
    pub created_at: String,
}

impl Todo {
    /// 创建新的待办事项
    pub fn new(id: u32, content: String) -> Self {
        let created_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            id,
            content,
            done: false,
            created_at,
        }
    }

    /// 标记完成
    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
```






## 第三步：实现存储模块

### src/storage.rs

```rust
use anyhow::Result;
use serde_json;
use std::fs;
use std::path::PathBuf;

use crate::todo::Todo;

/// 存储管理器
pub struct Storage {
    /// 数据文件路径
    path: PathBuf,
}

impl Storage {
    /// 创建存储管理器
    pub fn new() -> Self {
        // 数据存储在用户目录下
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let data_dir = home.join(".todo-cli");
        
        // 确保目录存在
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).ok();
        }
        
        Self {
            path: data_dir.join("todos.json"),
        }
    }

    /// 加载所有待办事项
    pub fn load(&self) -> Result<Vec<Todo>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&self.path)?;
        let todos: Vec<Todo> = serde_json::from_str(&content)?;
        Ok(todos)
    }

    /// 保存所有待办事项
    pub fn save(&self, todos: &[Todo]) -> Result<()> {
        let content = serde_json::to_string_pretty(todos)?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    /// 获取下一个 ID
    pub fn next_id(&self) -> Result<u32> {
        let todos = self.load()?;
        let max_id = todos.iter().map(|t| t.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }
}
```






## 第四步：实现命令处理

### src/commands.rs

```rust
use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::todo::Todo;

/// 添加待办事项
pub fn add(storage: &Storage, content: String) -> Result<()> {
    let id = storage.next_id()?;
    let todo = Todo::new(id, content);
    
    let mut todos = storage.load()?;
    todos.push(todo);
    storage.save(&todos)?;
    
    println!("{} 待办事项 #{}: {}", "✓ 添加成功".green(), id, todo.content);
    Ok(())
}

/// 列出所有待办事项
pub fn list(storage: &Storage, show_done: bool) -> Result<()> {
    let todos = storage.load()?;
    
    if todos.is_empty() {
        println!("暂无待办事项");
        return Ok(());
    }
    
    println!("\n{}", "待办事项列表".bold());
    println!("{}", "─".repeat(50));
    
    for todo in todos {
        if !show_done && todo.done {
            continue;
        }
        
        let status = if todo.done {
            "✓".green()
        } else {
            "○".yellow()
        };
        
        let content = if todo.done {
            todo.content.dimmed()
        } else {
            todo.content.normal()
        };
        
        println!("{} [#{}] {} - {}", status, todo.id, content, todo.created_at);
    }
    
    println!("{}", "─".repeat(50));
    let done_count = todos.iter().filter(|t| t.done).count();
    println!("总计: {} 项, 已完成: {} 项", todos.len(), done_count);
    
    Ok(())
}

/// 标记完成
pub fn done(storage: &Storage, id: u32) -> Result<()> {
    let mut todos = storage.load()?;
    
    let todo = todos.iter_mut().find(|t| t.id == id);
    
    match todo {
        Some(t) => {
            t.mark_done();
            storage.save(&todos)?;
            println!("{} 待办事项 #{} 已完成", "✓".green(), id);
        }
        None => {
            println!("{} 未找到待办事项 #{}", "✗".red(), id);
        }
    }
    
    Ok(())
}

/// 删除待办事项
pub fn remove(storage: &Storage, id: u32) -> Result<()> {
    let mut todos = storage.load()?;
    
    let index = todos.iter().position(|t| t.id == id);
    
    match index {
        Some(i) => {
            let removed = todos.remove(i);
            storage.save(&todos)?;
            println!("{} 已删除待办事项 #{}: {}", "✓".green(), id, removed.content);
        }
        None => {
            println!("{} 未找到待办事项 #{}", "✗".red(), id);
        }
    }
    
    Ok(())
}

/// 清空已完成的事项
pub fn clear_done(storage: &Storage) -> Result<()> {
    let todos = storage.load()?;
    let remaining: Vec<Todo> = todos.into_iter().filter(|t| !t.done).collect();
    
    let removed_count = storage.load()?.len() - remaining.len();
    storage.save(&remaining)?;
    
    println!("{} 已清理 {} 个完成的事项", "✓".green(), removed_count);
    Ok(())
}
```






## 第五步：CLI 入口

### src/main.rs

```rust
use clap::{Parser, Subcommand};
use anyhow::Result;

mod todo;
mod storage;
mod commands;

use storage::Storage;

#[derive(Parser)]
#[command(name = "todo")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "待办事项管理工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加待办事项
    Add {
        /// 待办事项内容
        content: String,
    },
    /// 列出待办事项
    List {
        /// 显示已完成的事项
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },
    /// 标记完成
    Done {
        /// 待办事项 ID
        id: u32,
    },
    /// 删除待办事项
    Remove {
        /// 待办事项 ID
        id: u32,
    },
    /// 清空已完成的事项
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let storage = Storage::new();
    
    match cli.command {
        Commands::Add { content } => {
            commands::add(&storage, content)?;
        }
        Commands::List { all } => {
            commands::list(&storage, all)?;
        }
        Commands::Done { id } => {
            commands::done(&storage, id)?;
        }
        Commands::Remove { id } => {
            commands::remove(&storage, id)?;
        }
        Commands::Clear => {
            commands::clear_done(&storage)?;
        }
    }
    
    Ok(())
}
```







