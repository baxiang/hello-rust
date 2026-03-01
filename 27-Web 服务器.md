# 第 27 章：Web 服务器开发

> 使用 Rust 构建高性能、安全的 Web 应用

---

## 27.1 为什么用 Rust 写 Web

```
┌─────────────────────────────────────────────────────┐
│          Rust Web 开发优势                           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  🚀 高性能                                           │
│     • 编译为本地代码，无 GC 暂停                      │
│     • 与 Node.js/Python 相比有数量级性能提升          │
│                                                     │
│  🛡️ 内存安全                                         │
│     • 无数据竞争                                     │
│     • 编译时消除大部分并发 bug                        │
│                                                     │
│  🔧 优秀的类型系统                                   │
│     • 编译时捕获错误                                 │
│     • 重构更安全                                     │
│                                                     │
│  📦 丰富的生态系统                                   │
│     • Axum, Actix-web, Rocket 等成熟框架             │
│     • 完善的中间件和工具链                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 主流框架对比

| 框架 | 特点 | 适用场景 |
|------|------|----------|
| Axum | ergonomic, tokio-based, 模块化 | 新项目首选 |
| Actix-web | 高性能，成熟稳定 | 高并发场景 |
| Rocket | 易用性，宏驱动 | 快速原型 |
| Warp | 轻量，组合式 | 微服务 |

---

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

## 27.3 路由

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

---

## 27.4 请求处理（Extractors）

### 提取器类型

```rust
use axum::{
    extract::{Path, Query, State, Json, Extension},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use std::collections::HashMap;

// 查询参数
#[derive(Deserialize)]
struct SearchParams {
    q: String,
    page: Option<u32>,
    limit: Option<u32>,
}

async fn search(Query(params): Query<SearchParams>) -> String {
    format!("Search: {} (page: {:?}, limit: {:?})",
            params.q, params.page, params.limit)
}

// 原始查询字符串
async fn raw_query(query: Query<HashMap<String, String>>) -> String {
    format!("{:?}", query)
}

// 请求头
async fn show_headers(headers: HeaderMap) -> String {
    let mut result = String::new();
    for (name, value) in &headers {
        result.push_str(&format!("{}: {:?}\n", name, value));
    }
    result
}

// 应用状态
struct AppState {
    name: String,
}

async fn with_state(State(state): State<AppState>) -> String {
    format!("App: {}", state.name)
}
```

### JSON 请求体

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
    age: Option<u32>,
}

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<UserResponse> {
    // 处理创建逻辑
    Json(UserResponse {
        id: 1,
        name: payload.name,
        email: payload.email,
    })
}
```

---

## 27.5 响应处理

### 返回类型

```rust
use axum::{
    response::{IntoResponse, Response, Html, Redirect, PlainText},
    http::{StatusCode, header, HeaderMap},
    Json,
};
use serde_json::json;

// 字符串
async fn text() -> &'static str {
    "Hello"
}

// HTML
async fn html() -> Html<&'static str> {
    Html("<h1>Hello</h1>")
}

// JSON
async fn json() -> Json<serde_json::Value> {
    Json(json!({"message": "Hello"}))
}

// 状态码
async fn created() -> StatusCode {
    StatusCode::CREATED
}

// 组合响应
async fn full_response() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Success")
}

// 带响应头
async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", "value".parse().unwrap());
    (headers, "OK")
}

// 重定向
async fn redirect() -> Redirect {
    Redirect::temporary("/new-location")
}

// 字节
async fn bytes() -> Vec<u8> {
    vec![1, 2, 3, 4]
}
```

### 错误响应

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// 简单错误
async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

// 带消息的错误
async fn bad_request() -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, "Invalid input".to_string())
}

// JSON 错误
async fn json_error() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": "Something went wrong"})),
    )
}

// Result 返回
async fn result_handler(id: u32) -> Result<Json<&'static str>, StatusCode> {
    if id == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json("Found"))
    }
}
```

---

## 27.6 完整示例：REST API

### 项目结构

```
src/
├── main.rs
├── handlers.rs
├── models.rs
├── state.rs
└── error.rs
```

### main.rs

```rust
use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

mod handlers;
mod models;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    // 初始化共享状态
    let state = AppState {
        users: Arc::new(RwLock::new(vec![])),
    };

    // 构建路由
    let app = Router::new()
        .route("/users", get(handlers::list_users))
        .route("/users", post(handlers::create_user))
        .route("/users/:id", get(handlers::get_user))
        .route("/users/:id", put(handlers::update_user))
        .route("/users/:id", delete(handlers::delete_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### models.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
}
```

### state.rs

```rust
use crate::models::User;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
}
```

### handlers.rs

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{
    models::{User, CreateUserRequest, ApiResponse},
    state::AppState,
};

pub async fn list_users(
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    let users = state.users.read().await;
    Json(users.clone())
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;

    // 检查 email 是否已存在
    if users.iter().any(|u| u.email == payload.email) {
        return Err(StatusCode::CONFLICT);
    }

    // 生成新 ID
    let new_id = users.iter().map(|u| u.id).max().unwrap_or(0) + 1;

    let user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };

    users.push(user.clone());
    Ok(Json(user))
}

pub async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;
    users.iter()
        .find(|u| u.id == id)
        .map(|u| Json(u.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn update_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.name = payload.name;
        user.email = payload.email;
        return Ok(Json(user.clone()));
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.users.write().await;

    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        return Ok(StatusCode::NO_CONTENT);
    }

    Err(StatusCode::NOT_FOUND)
}
```

### 测试 API

```bash
# 列出用户
curl http://localhost:3000/users

# 创建用户
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'

# 获取用户
curl http://localhost:3000/users/1

# 更新用户
curl -X PUT http://localhost:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice Updated","email":"alice@example.com"}'

# 删除用户
curl -X DELETE http://localhost:3000/users/1
```

---

## 27.7 中间件

### 日志中间件

```rust
use axum::{
    middleware::{self, Next},
    http::{Request, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use std::time::Instant;

async fn log_request<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    println!("--> {} {} ", method, uri);

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();
    println!("<-- {} {} {:?}", uri, status, duration);

    Ok(response)
}

async fn handler() -> &'static str {
    "Hello"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(log_request));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

### CORS 中间件

```rust
use axum::{
    middleware,
    http::{Request, Response, StatusCode},
    Router,
};
use tower_http::cors::{CorsLayer, Any};

async fn cors_middleware<B>(
    req: Request<B>,
    next: middleware::Next<B>,
) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;

    response.headers_mut().insert(
        "Access-Control-Allow-Origin",
        "*".parse().unwrap(),
    );

    Ok(response)
}

#[tokio::main]
async fn main() {
    // 使用 tower-http 的 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .layer(cors);

    // 或者使用自定义中间件
    let app_with_custom = Router::new()
        .route("/", get(|| async { "Hello" }))
        .layer(middleware::from_fn(cors_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

---

## 27.8 静态文件服务

### 使用 tower-http

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["fs", "trace"] }
```

```rust
use axum::{Router, routing::get_service};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        // 服务静态目录
        .nest_service("/static", ServeDir::new("public"))
        // 服务单个文件
        .nest_service(
            "/favicon.ico",
            ServeFile::new("assets/favicon.ico"),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

---

## 27.9 Actix-web 示例

### 项目设置

```toml
[package]
name = "actix-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
```

### Hello World

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn greet(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().json(User {
        id: 1,
        name: info.name.clone(),
    })
}

async fn get_user(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(User {
        id: *path,
        name: format!("User {}", path),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/greet", web::post().to(greet))
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

## 27.10 数据库集成

### 使用 SQLx

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use axum::{
    extract::{Path, State},
    Json, Router,
    routing::{get, post},
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(users))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { db };

    let app = Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

---

## 27.11 常见错误处理

### 自定义错误类型

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
    DatabaseError(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, Json(json!({"error": msg}))).into_response()
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response()
            }
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": msg}))).into_response()
            }
            AppError::DatabaseError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
```

---

## 27.12 练习

### 练习 1：用户认证 API

实现一个用户认证系统：
- 注册（用户名 + 密码）
- 登录（返回 JWT）
- 获取当前用户信息

### 练习 2：文件上传服务

创建一个文件上传 API：
- 支持 multipart/form-data
- 保存上传的文件
- 提供文件下载接口

### 练习 3：分页列表 API

实现带分页的列表接口：
- `?page=1&limit=10`
- 返回总数和分页信息

---

## 27.13 小结

本章我们学习了：

- ✅ Axum 基础用法
- ✅ 路由和提取器
- ✅ 响应处理
- ✅ REST API 实现
- ✅ 中间件
- ✅ 静态文件服务
- ✅ Actix-web 示例
- ✅ 数据库集成

### Web 框架选择

| 需求 | 推荐框架 |
|------|----------|
| 新项目 | Axum |
| 高性能 | Actix-web |
| 易用性 | Rocket |
| 微服务 | Warp |

---

## 下一章

[第 28 章：测试与文档](28-测试与文档.md)
