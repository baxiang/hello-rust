## 第七步：TCP 服务器

### src/server.rs

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use crate::store::engine::Store;
use crate::command::{parser::CommandParser, executor::CommandExecutor, protocol::Response};

/// 启动服务器
pub async fn run(addr: &str) -> anyhow::Result<()> {
    let store = Arc::new(Store::new());
    let executor = Arc::new(CommandExecutor::new((*store).clone()));
    
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("服务器启动: {}", addr);
    
    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::debug!("新连接: {}", addr);
        
        let executor = executor.clone();
        tokio::spawn(async move {
            handle_connection(socket, executor).await;
        });
    }
}

/// 处理连接
async fn handle_connection(
    mut socket: TcpStream,
    executor: Arc<CommandExecutor>,
) {
    let mut buffer = vec![0u8; 1024];
    
    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(0) => break,  // 连接关闭
            Ok(n) => n,
            Err(_) => break,
        };
        
        // 解析命令
        let command = CommandParser::parse_resp(&buffer[..n]);
        
        if let Some(cmd) = command {
            // 执行命令
            let response = executor.execute(cmd);
            
            // 发送响应
            let response_str = response.to_string();
            if socket.write_all(response_str.as_bytes()).await.is_err() {
                break;
            }
        }
    }
}
```

---

---

## 下一步

[第八步：主入口](../第八步：主入口.md)