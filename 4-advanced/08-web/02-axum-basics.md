## Axum 快速入门

### 项目设置

```toml
[package]
name = "axum-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
```

### Hello World

```rust
use axum::{routing::get, Router};

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // 构建路由
    let app = Router::new().route("/", get(root));

    // 绑定地址
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    // 启动服务
    axum::serve(listener, app).await.unwrap();
}
```

### 运行

```bash
cargo run

# 访问 http://127.0.0.1:3000
```






## 路由

### 基本路由

```rust
use axum::{routing::{get, post, put, delete}, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/posts", get(list_posts))
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str { "Home" }
async fn list_users() -> &'static str { "List users" }
async fn create_user() -> &'static str { "Create user" }
async fn get_user() -> &'static str { "Get user" }
async fn update_user() -> &'static str { "Update user" }
async fn delete_user() -> &'static str { "Delete user" }
async fn list_posts() -> &'static str { "List posts" }
async fn health_check() -> &'static str { "OK" }
```

### 路径参数

```rust
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

// 单个路径参数
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        name: format!("User {}", id),
    })
}

// 多个路径参数
async fn get_user_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("User {} Post {}", user_id, post_id)
}

// 字符串参数
async fn show_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
```







