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

---

## 下一步

[响应处理](../5-响应处理.md)