//! # 示例：异步基础
//! 
//! 对应章节：01-异步基础
//! 运行：cargo run --example 01-async-basics

use std::time::Duration;

async fn hello_async() {
    println!("Hello from async!");
}

async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(Duration::from_millis(100)).await;
    "数据获取完成".to_string()
}

#[tokio::main]
async fn main() {
    println!("=== 异步编程基础 ===\n");
    
    hello_async().await;
    
    let data = fetch_data().await;
    println!("{}", data);
    
    // ✅ 并发执行
    let (a, b) = tokio::join!(
        async { 1 + 2 },
        async { 3 + 4 },
    );
    println!("并发结果：{} {}", a, b);
    
    println!("\n异步章节完成！");
}
