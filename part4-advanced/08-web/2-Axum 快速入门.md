## 27.2 Axum 快速入门

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

---

---

## 下一步

[路由](../3-路由.md)