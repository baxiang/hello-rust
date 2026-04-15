use axum::{
    routing::get,
    Router,
};

async fn hello() -> &'static str {
    "Hello, Axum!"
}

#[tokio::main]
async fn main() {
    println!("=== Axum Web 服务器 ===\n");
    
    let _app: Router = Router::new()
        .route("/", get(hello));
    
    println!("服务器将运行在 http://127.0.0.1:3000");
    println!("（示例仅演示，不实际启动服务器）");
}
