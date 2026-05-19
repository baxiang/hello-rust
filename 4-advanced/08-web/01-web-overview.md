## 为什么用 Rust 写 Web

**概念名称：** Rust 提供高性能、内存安全的 Web 开发体验，无 GC 暂停。

```
语法结构：
┌──────────────────────────────────────┐
│  async fn handler() -> impl IntoResponse│
│                                       │
│  Router::new()                       │
│      .route("/", get(handler))       │
│      .route("/api", post(api_handler))│
│                                       │
│  axum::serve(listener, app).await    │
└──────────────────────────────────────┘
```

### 最简示例

```rust
use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));
    
    println!("服务器运行在 http://127.0.0.1:3000");
}
```

### Rust Web 开发优势

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

### 为什么用它？

```rust
// 没有 Rust：Node.js 有 GC 暂停，Python 有 GIL 限制
// async def handler(request):
//     return JSONResponse({"message": "Hello"})

// 有 Rust：零成本抽象，类型安全
async fn hello() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Hello"}))
}
```

**关键代码说明：**

| 代码 | 含义 | 为什么这样写 |
|------|------|-------------|
| `async fn` | 异步函数 | Web 服务器需要异步处理并发请求 |
| `Router::new()` | 创建路由器 | 定义 URL 到处理函数的映射 |
| `.route("/", get(hello))` | 注册路由 | GET 请求 `/` 调用 `hello` 函数 |
| `#[tokio::main]` | 异步运行时 | 提供事件循环和任务调度 |
