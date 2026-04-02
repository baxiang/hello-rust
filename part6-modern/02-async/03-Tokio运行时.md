# Tokio 运行时

> 掌握 Tokio 异步运行时的核心原理和实践。

## 什么是运行时？

异步运行时负责：
1. 调度和执行异步任务
2. 管理 I/O 事件（网络、文件、定时器）
3. 处理任务之间的通信
4. 提供异步同步原语

```
┌─────────────────────────────────────────┐
│           Tokio 运行时架构                │
├─────────────────────────────────────────┤
│                                         │
│  ┌──────────────────────────────────┐ │
│  │       任务调度器（Scheduler）      │ │
│  │  ┌───────────┐  ┌───────────┐   │ │
│  │  │ 工作线程 1 │  │ 工作线程 2 │   │ │
│  │  └───────────┘  └───────────┘   │ │
│  └──────────────────────────────────┘ │
│                                         │
│  ┌──────────────────────────────────┐ │
│  │      I/O 驱动（I/O Driver）       │ │
│  │  ┌────────┐  ┌────────┐         │ │
│  │  │ TCP    │  │ UDP     │         │ │
│  │  └────────┘  └────────┘         │ │
│  │  ┌────────┐  ┌────────┐         │ │
│  │  │ 定时器 │  │ 文件    │         │ │
│  │  └────────┘  └────────┘         │ │
│  └──────────────────────────────────┘ │
│                                         │
│  ┌──────────────────────────────────┐ │
│  │       任务队列（Task Queue）      │ │
│  └──────────────────────────────────┘ │
│                                         │
└─────────────────────────────────────────┘
```

## Tokio 简介

### 核心组件

| 组件 | 功能 | 关键模块 |
|------|------|---------|
| 运行时 | 调度任务 | `tokio::runtime` |
| 任务 | 异步执行单元 | `tokio::task` |
| I/O | 异步网络和文件 | `tokio::net`, `tokio::fs` |
| 同步原语 | 异步锁和通道 | `tokio::sync` |
| 定时器 | 异步时间控制 | `tokio::time` |

### 安装 Tokio

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

**特性选择：**

```toml
# 最小特性（减少编译时间和大小）
tokio = { version = "1.0", features = ["rt", "macros"] }

# 常用特性
tokio = { version = "1.0", features = [
    "rt-multi-thread",  # 多线程运行时
    "macros",           # #[tokio::main] 等
    "net",              # 网络 I/O
    "time",             # 定时器
    "fs",               # 文件 I/O
    "sync",             # 同步原语
    "io-util",          # I/O 工具
] }
```

## 运行时配置

### #[tokio::main]

```rust
// 最简单的配置
#[tokio::main]
async fn main() {
    println!("Hello Tokio!");
}

// 多线程运行时（默认）
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 使用 4 个工作线程
}

// 单线程运行时
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 在当前线程运行，适合简单任务
}
```

### 手动创建运行时

```rust
use tokio::runtime::Runtime;

fn main() {
    // 创建运行时
    let rt = Runtime::new().unwrap();
    
    // 在运行时中执行异步代码
    rt.block_on(async {
        println!("In runtime!");
    });
}

// 多线程运行时
use tokio::runtime::{Runtime, Builder};

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        // 异步代码
    });
}

// 单线程运行时
let rt = Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
```

### 运行时类型选择

```rust
// ✅ 多线程运行时：生产环境、高并发
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // 默认使用所有 CPU 核心
}

// ✅ 单线程运行时：测试、简单任务
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 适合少量任务
}
```

| 运行时类型 | 适用场景 | 特点 |
|-----------|---------|------|
| multi_thread | 生产环境、高并发 | 多工作线程，并行处理 |
| current_thread | 测试、简单应用 | 单线程，减少开销 |

## 任务调度

### tokio::spawn

```rust
// spawn 创建新任务，立即返回 JoinHandle
async fn main() {
    let handle = tokio::spawn(async {
        println!("Task running");
        42
    });
    
    // 可以继续执行其他代码
    println!("After spawn");
    
    // 等待任务完成并获取结果
    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
```

### JoinHandle

```rust
use tokio::task::JoinHandle;

async fn example() {
    let handle: JoinHandle<i32> = tokio::spawn(async {
        42
    });
    
    // JoinHandle 本身是一个 Future
    let result = handle.await;  // 等待任务完成
    
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Task failed: {}", e),
    }
}
```

### 任务隔离

```rust
// spawn_local: 在当前线程创建任务（仅单线程运行时）
tokio::task::spawn_local(async {
    // 在当前线程执行
});

// spawn_blocking: 在独立线程池执行阻塞任务
let result = tokio::task::spawn_blocking(|| {
    // 阻塞操作，如调用阻塞库
    std::thread::sleep(Duration::from_secs(1));
    42
}).await;
```

### 任务取消

```rust
use tokio::task::AbortHandle;

async fn example() {
    let handle = tokio::spawn(async {
        loop {
            println!("Working...");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    
    // 获取 AbortHandle
    let abort_handle = handle.abort_handle();
    
    // 5 秒后取消任务
    tokio::time::sleep(Duration::from_secs(5)).await;
    abort_handle.abort();
    
    // 等待任务（会返回错误）
    match handle.await {
        Ok(_) => println!("Task completed"),
        Err(e) => {
            if e.is_cancelled() {
                println!("Task was cancelled");
            }
        }
    }
}
```

## I/O 驱动

### 网络 I/O

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP 服务器
async fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);
        
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 { break; }  // 连接关闭
        
        stream.write_all(&buf[..n]).await.unwrap();
    }
}

// TCP 客户端
async fn tcp_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    
    stream.write_all(b"Hello").await.unwrap();
    
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
}
```

### UDP

```rust
use tokio::net::UdpSocket;

async fn udp_server() {
    let socket = UdpSocket::bind("127.0.0.1:8080").await.unwrap();
    
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        println!("Received from {}", addr);
        
        socket.send_to(&buf[..len], addr).await.unwrap();
    }
}

async fn udp_client() {
    let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    
    socket.send_to(b"Hello", "127.0.0.1:8080").await.unwrap();
    
    let mut buf = [0; 1024];
    let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..len]));
}
```

### 文件 I/O

```rust
use tokio::fs::{File, read_to_string, write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// 读取文件
async fn read_file(path: &str) -> String {
    read_to_string(path).await.unwrap()
}

// 写入文件
async fn write_file(path: &str, content: &str) {
    write(path, content).await.unwrap();
}

// 流式读写
async fn copy_file(src: &str, dst: &str) {
    let mut src_file = File::open(src).await.unwrap();
    let mut dst_file = File::create(dst).await.unwrap();
    
    tokio::io::copy(&mut src_file, &mut dst_file).await.unwrap();
}
```

## 定时器

### sleep

```rust
use tokio::time::{sleep, Duration};

async fn example() {
    println!("Start");
    sleep(Duration::from_secs(2)).await;
    println!("After 2 seconds");
}
```

### interval

```rust
use tokio::time::{interval, Duration};

async fn periodic_task() {
    let mut timer = interval(Duration::from_secs(1));
    
    loop {
        timer.tick().await;  // 等待下一个周期
        println!("Tick!");
    }
}
```

### timeout

```rust
use tokio::time::{timeout, Duration};

async fn operation() -> String {
    sleep(Duration::from_secs(5)).await;
    "completed".to_string()
}

async fn with_timeout() {
    match timeout(Duration::from_secs(2), operation()).await {
        Ok(result) => println!("Result: {}", result),
        Err(_) => println!("Timeout!"),
    }
}
```

### delay_for（已弃用，使用 sleep）

```rust
// ❌ 旧 API（已弃用）
tokio::time::delay_for(Duration::from_secs(1)).await;

// ✅ 新 API
tokio::time::sleep(Duration::from_secs(1)).await;
```

## 同步原语

### Mutex

```rust
use tokio::sync::Mutex;

struct SharedState {
    counter: Mutex<i32>,
}

async fn increment(state: Arc<SharedState>) {
    let mut lock = state.counter.lock().await;
    *lock += 1;
}

async fn main() {
    let state = Arc::new(SharedState {
        counter: Mutex::new(0),
    });
    
    // 并发增量
    let tasks: Vec<_> = (0..10)
        .map(|_| tokio::spawn(increment(state.clone())))
        .collect();
    
    for task in tasks {
        task.await.unwrap();
    }
    
    let final_value = *state.counter.lock().await;
    println!("Final: {}", final_value);  // 10
}
```

### RwLock

```rust
use tokio::sync::RwLock;

struct Database {
    data: RwLock<HashMap<String, String>>,
}

async fn read(db: Arc<Database>, key: &str) -> Option<String> {
    let lock = db.data.read().await;
    lock.get(key).cloned()
}

async fn write(db: Arc<Database>, key: String, value: String) {
    let mut lock = db.data.write().await;
    lock.insert(key, value);
}
```

### 通道

```rust
use tokio::sync::mpsc;

async fn channel_example() {
    // 创建通道（容量 32）
    let (tx, mut rx) = mpsc::channel(32);
    
    // 发送者
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });
    
    // 接收者
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}

// 多发送者
async fn multi_sender() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // 克隆发送者
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    
    tokio::spawn(async move {
        tx1.send(1).await.unwrap();
    });
    
    tokio::spawn(async move {
        tx2.send(2).await.unwrap();
    });
    
    // 丢弃最后一个发送者以关闭通道
    drop(tx);
    
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}
```

### oneshot

```rust
use tokio::sync::oneshot;

async fn oneshot_example() {
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        tx.send(42).unwrap();
    });
    
    match rx.await {
        Ok(value) => println!("Received: {}", value),
        Err(_) => println!("Sender dropped"),
    }
}
```

### broadcast

```rust
use tokio::sync::broadcast;

async fn broadcast_example() {
    let (tx, _) = broadcast::channel(16);
    
    // 创建多个接收者
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    
    tokio::spawn(async move {
        tx.send("message").unwrap();
    });
    
    println!("rx1: {}", rx1.recv().await.unwrap());
    println!("rx2: {}", rx2.recv().await.unwrap());
}
```

## 最佳实践

### 1. 避免阻塞运行时

```rust
// ❌ 错误：阻塞异步运行时
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));  // 阻塞整个运行时
}

// ✅ 正确：使用异步 sleep
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// ✅ 正确：使用 spawn_blocking 处理阻塞操作
async fn blocking_call() {
    let result = tokio::task::spawn_blocking(|| {
        // 阻塞操作在独立线程池执行
        std::thread::sleep(Duration::from_secs(1));
        42
    }).await.unwrap();
}
```

### 2. 合理设置并发度

```rust
// ✅ 控制并发任务数量
use tokio::sync::Semaphore;

async fn limited_concurrency() {
    let semaphore = Arc::new(Semaphore::new(10));  // 最多 10 个并发
    
    let tasks: Vec<_> = (0..100)
        .map(|i| {
            let sem = semaphore.clone();
            tokio::spawn(async move {
                let permit = sem.acquire().await.unwrap();
                process(i).await;
                permit.forget();
            })
        })
        .collect();
    
    for task in tasks {
        task.await.unwrap();
    }
}
```

### 3. 使用 Arc 共享状态

```rust
use std::sync::Arc;

async fn shared_state() {
    let state = Arc::new(Mutex::new(0));
    
    let state1 = state.clone();
    let state2 = state.clone();
    
    tokio::spawn(async move {
        let mut lock = state1.lock().await;
        *lock += 1;
    });
    
    tokio::spawn(async move {
        let mut lock = state2.lock().await;
        *lock += 1;
    });
}
```

### 4. 处理任务错误

```rust
async fn handle_errors() {
    let handle = tokio::spawn(async {
        // 可能失败的任务
        Err("task failed")
    });
    
    match handle.await {
        Ok(result) => match result {
            Ok(value) => println!("Success: {}", value),
            Err(e) => println!("Task error: {}", e),
        },
        Err(e) => {
            if e.is_panic() {
                println!("Task panicked");
            } else if e.is_cancelled() {
                println!("Task cancelled");
            }
        }
    }
}
```

## 性能优化

### 工作线程数量

```rust
// ✅ 根据负载调整线程数
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    // CPU 密集型：线程数 = CPU 核数
    // I/O 密集型：线程数可以更多
}
```

### 任务队列大小

```rust
// ✅ 设置合适的通道容量
let (tx, rx) = mpsc::channel(1000);  // 根据需求调整

// ❌ 无限容量（可能导致内存耗尽）
// let (tx, rx) = mpsc::channel(usize::MAX);
```

### 批量处理

```rust
// ✅ 批量处理减少开销
async fn batch_process() {
    let mut batch = Vec::with_capacity(100);
    
    while let Some(item) = receive_item().await {
        batch.push(item);
        
        if batch.len() == 100 {
            process_batch(&batch).await;
            batch.clear();
        }
    }
}
```

## 小结

**Tokio 运行时核心：**
- 任务调度器：管理和执行异步任务
- I/O 驱动：处理网络、文件、定时器事件
- 同步原语：Mutex、RwLock、通道

**关键组件：**
- `tokio::spawn`: 创建并发任务
- `tokio::net`: 异步网络 I/O
- `tokio::fs`: 异步文件 I/O
- `tokio::sync`: 异步同步原语
- `tokio::time`: 定时器和超时

**最佳实践：**
- 避免阻塞运行时
- 使用 `spawn_blocking` 处理阻塞操作
- 合理控制并发度
- 使用 `Arc` 共享状态

**下一步：**
下一节我们将学习异步编程的常见模式和技巧。

## 练习

### 练习 1：TCP Echo 服务器

实现一个 TCP echo 服务器：

```rust
// TODO: 接收客户端连接
// TODO: 将收到的数据原样返回
// TODO: 支持多客户端并发
```

### 练习 2：并发任务管理

使用通道实现任务分发：

```rust
// TODO: 创建任务通道
// TODO: 多个工作者接收任务
// TODO: 处理任务并发送结果
```

### 练习 3：定时任务

实现一个定时任务系统：

```rust
// TODO: 每秒打印一次时间
// TODO: 使用 timeout 处理超时
// TODO: 使用 interval 实现周期任务
```