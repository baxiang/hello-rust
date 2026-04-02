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

---

## 下一步

[数据库集成](../10-数据库集成.md)