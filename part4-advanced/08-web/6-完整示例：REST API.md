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

---

## 下一步

[中间件](../7-中间件.md)