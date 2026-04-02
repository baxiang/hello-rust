# 现代 Crate 推荐

> 掌握 Rust 生态中的现代 crate 和最佳实践。

## 错误处理

### thiserror - 库错误定义

```rust
// ✅ 推荐：库代码使用 thiserror
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    
    #[error("Query failed: {message}")]
    QueryError {
        message: String,
        #[source]
        source: sqlx::Error,
    },
    
    #[error("Record not found: id={id}")]
    NotFound { id: u64 },
}

// 使用
fn find_user(id: u64) -> Result<User, DatabaseError> {
    Err(DatabaseError::NotFound { id })
}
```

**优势：**
- 零运行时开销
- 自动实现 `std::error::Error`
- 支持错误链（`#[source]`）
- 清晰的错误消息模板

### anyhow - 应用错误处理

```rust
// ✅ 推荐：应用代码使用 anyhow
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read config from {}", path))?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}

fn main() -> Result<()> {
    let config = read_config("config.toml")?;
    Ok(())
}
```

**优势：**
- 简化错误类型（`Result<T, anyhow::Error>`）
- 自动错误转换（`?` 操作符）
- 上下文信息（`.context()`）
- 错误链追踪

### 错误处理对比

```rust
// ❌ 避免在库中使用 anyhow
pub fn library_function() -> anyhow::Result<Data> {
    // 库应该提供具体的错误类型
}

// ✅ 库使用 thiserror
#[derive(Debug, Error)]
pub enum LibraryError { /* ... */ }
pub fn library_function() -> Result<Data, LibraryError> {
    // 调用者可以精确处理错误
}

// ✅ 应用使用 anyhow
fn main() -> anyhow::Result<()> {
    // 应用不需要区分具体错误
    let data = library_function().context("Library call failed")?;
    Ok(())
}
```

## 异步运行时

### Tokio - 生产级运行时

```rust
// ✅ 推荐：使用 Tokio
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");
    sleep(Duration::from_secs(1)).await;
    println!("End");
}

// 多线程运行时
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 使用 4 个工作线程
}

// 单线程运行时（适合简单任务）
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 在当前线程运行
}
```

**核心组件：**

```rust
use tokio::{
    fs::File,           // 异步文件 I/O
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},  // 异步网络
    sync::{Mutex, RwLock, mpsc},    // 异步同步原语
    time::{sleep, timeout},         // 异步时间
    task::{spawn, JoinHandle},      // 任务管理
};
```

### async-std - 标准库风格的异步

```rust
// 替代方案：async-std
use async_std::task;

#[async_std::main]
async fn main() {
    println!("Hello async-std");
}
```

**选择建议：**
- **Tokio**: 生产环境、Web 服务、网络密集型应用
- **async-std**: 简单应用、熟悉标准库风格的开发者

## Web 框架

### Axum - 现代异步 Web 框架

```rust
// ✅ 推荐：使用 Axum
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", post(create_user));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

**优势：**
- 基于 Tower 生态系统
- 类型安全的提取器
- 简洁的路由语法
- 与 Tokio 完美集成

### Actix-web - 高性能 Web 框架

```rust
// 替代方案：Actix-web
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**选择建议：**
- **Axum**: 新项目、追求简洁、Tower 生态
- **Actix-web**: 高性能需求、成熟生态

## 序列化

### serde - 通用序列化框架

```rust
// ✅ 必需：serde 是序列化的基础
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    username: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<String>,
}

// JSON 序列化
let user = User {
    id: 1,
    username: "alice".to_string(),
    email: "alice@example.com".to_string(),
    bio: None,
};

let json = serde_json::to_string(&user)?;
let parsed: User = serde_json::from_str(&json)?;
```

### serde_json - JSON 支持

```rust
use serde_json::{json, Value};

// 创建 JSON 值
let data = json!({
    "name": "Alice",
    "age": 30,
    "hobbies": ["reading", "coding"],
});

// 解析动态 JSON
let value: Value = serde_json::from_str(json_str)?;
let name = value["name"].as_str().unwrap();
```

### toml - 配置文件

```rust
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

let config: Config = toml::from_str(config_str)?;
```

## CLI 开发

### clap - 命令行参数解析

```rust
// ✅ 推荐：使用 clap derive API
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A CLI application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new item
    Add {
        #[arg(short, long)]
        name: String,
        
        #[arg(short, long, default_value = "default")]
        category: String,
    },
    /// List all items
    List {
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { name, category } => {
            println!("Adding {} to {}", name, category);
        }
        Commands::List { all } => {
            println!("Listing items, all={}", all);
        }
    }
}
```

**生成的帮助信息：**

```
$ myapp --help
A CLI application

Usage: myapp [OPTIONS] <COMMAND>

Commands:
  add   Add a new item
  list  List all items

Options:
  -v, --verbose  
  -h, --help     Print help
```

## 日志与追踪

### tracing - 结构化日志

```rust
// ✅ 推荐：使用 tracing
use tracing::{info, warn, error, debug, instrument};

#[instrument]
async fn process_order(order_id: u64) -> Result<(), Error> {
    info!(order_id, "Processing order");
    
    match validate_order(order_id).await {
        Ok(_) => {
            info!(order_id, "Order validated");
            Ok(())
        }
        Err(e) => {
            error!(order_id, "Validation failed: {}", e);
            Err(e)
        }
    }
}

// 初始化
tracing_subscriber::fmt::init();
```

**优势：**
- 结构化日志（JSON 输出）
- 异步友好
- 性能追踪（span）
- 与 Tokio 集成

## 测试

### 测试工具

```rust
// ✅ 使用标准测试框架
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[should_panic(expected = "overflow")]
    fn test_panic() {
        panic!("overflow");
    }
    
    #[tokio::test]
    async fn test_async() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}

// 使用测试属性
#[test]
fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_test_data()?;
    assert!(!data.is_empty());
    Ok(())
}
```

## Crate 选择指南

| 类别 | 推荐 Crate | 替代方案 | 使用场景 |
|------|-----------|---------|---------|
| 错误处理 | `thiserror` + `anyhow` | `eyre`, `color-eyre` | 所有项目 |
| 异步运行时 | `tokio` | `async-std` | 异步应用 |
| Web 框架 | `axum` | `actix-web`, `rocket` | Web 服务 |
| 序列化 | `serde` + `serde_json` | - | 所有项目 |
| 配置 | `toml`, `serde` | `config` | 配置管理 |
| CLI | `clap` | `structopt` | 命令行工具 |
| 日志 | `tracing` | `log`, `slog` | 所有项目 |
| HTTP 客户端 | `reqwest` | `surf`, `ureq` | HTTP 请求 |
| 数据库 | `sqlx`, `sea-orm` | `diesel` | 数据库操作 |
| 测试 | 内置测试框架 | `proptest`, `quickcheck` | 测试 |

## 最佳实践

### 1. Cargo.toml 配置

```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2024"

[dependencies]
# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# Web 框架
axum = "0.7"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"
```

### 2. 版本锁定

```toml
# ✅ 使用语义化版本范围
serde = "1.0"

# ✅ 锁定特性以减少编译时间
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }

# ❌ 避免过度精确的版本
serde = "=1.0.193"  # 不必要地限制灵活性
```

### 3. 使用 workspace

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "app",
    "lib1",
    "lib2",
]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

## 小结

**现代 Rust 开发的核心 crate：**

1. **错误处理**: `thiserror` (库) + `anyhow` (应用)
2. **异步运行时**: `tokio` (生产级)
3. **Web 框架**: `axum` (现代简洁)
4. **序列化**: `serde` + `serde_json`
5. **CLI**: `clap` (derive API)
6. **日志**: `tracing`

**选择原则：**
- 优先选择活跃维护的 crate
- 考虑生态系统兼容性
- 关注编译时间和二进制大小
- 使用 `cargo tree` 检查依赖树

**下一步：**
下一节我们将学习现代 Rust 工具链的使用。

## 练习

### 练习 1：配置错误处理

为一个简单的文件处理程序配置错误处理：

```rust
// TODO: 使用 thiserror 定义错误类型
// TODO: 使用 anyhow 处理应用错误
```

### 练习 2：创建 CLI 工具

创建一个支持子命令的 CLI 工具：

```rust
// TODO: 使用 clap 定义 CLI
// myapp add <name>
// myapp list
// myapp remove <id>
```

### 练习 3：Web 服务

使用 Axum 创建一个简单的 REST API：

```rust
// TODO: GET /users
// TODO: POST /users
// TODO: GET /users/:id
```