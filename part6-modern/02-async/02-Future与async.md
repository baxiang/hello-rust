# Future 与 async/await

> 理解 Future trait 的底层机制和 async/await 语法的原理。

## Future Trait

### Future 定义

```rust
// std::future::Future trait
pub trait Future {
    /// 异步操作完成时返回的类型
    type Output;
    
    /// 尝试推进 Future 的执行
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

/// poll 返回的结果
pub enum Poll<T> {
    /// 操作已完成，返回结果
    Ready(T),
    /// 操作未完成，继续等待
    Pending,
}
```

### Poll 机制

Future 通过 `poll` 方法推进执行：

```
┌──────────────────────────────────────────┐
│          Future 执行流程                   │
├──────────────────────────────────────────┤
│                                          │
│  运行时调用 poll()                        │
│      ↓                                   │
│  ┌─────────────────────┐                │
│  │ Ready(result)?      │                │
│  └────────┬────────────┘                │
│           │                              │
│     Yes   │   No                         │
│      ↓    │    ↓                         │
│  返回结果 │  返回 Pending                │
│           │    ↓                         │
│           │  注册唤醒器                  │
│           │    ↓                         │
│           │  等待事件就绪                │
│           │    ↓                         │
│           │  唤醒器唤醒                  │
│           │    ↓                         │
│           │  运行时再次 poll()           │
│           └─────────────────┐           │
│                          │              │
└──────────────────────────┼──────────────┘
                           │
                     循环直到 Ready
```

### 手动实现 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

// 一个简单的延时 Future
struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            // 时间已到，返回结果
            Poll::Ready("done")
        } else {
            // 时间未到，注册唤醒器
            // 在实际实现中，这里会设置定时器
            cx.waker().wake_by_ref();  // 立即唤醒（简化示例）
            Poll::Pending
        }
    }
}

// 使用
async fn example() {
    let delay = Delay {
        when: Instant::now() + Duration::from_secs(1),
    };
    
    let result = delay.await;
    println!("Result: {}", result);
}
```

### Waker 原理

```rust
// Waker 负责通知运行时 Future 可以继续 poll
use std::task::Waker;

struct MyFuture {
    data: Vec<u8>,
    waker: Option<Waker>,
}

impl Future for MyFuture {
    type Output = Vec<u8>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.data.is_ready() {
            Poll::Ready(self.data.clone())
        } else {
            // 保存唤醒器
            self.waker = Some(cx.waker().clone());
            
            // 当数据就绪时，调用 waker.wake()
            // 例如：在网络事件回调中
            Poll::Pending
        }
    }
}
```

**唤醒流程：**

```
1. Future 返回 Pending
   ↓
2. 运行时注册 Waker 到 I/O 事件源
   ↓
3. I/O 事件就绪（如：数据到达）
   ↓
4. 事件源调用 Waker.wake()
   ↓
5. 运行时再次调用 Future.poll()
   ↓
6. Future 返回 Ready
```

## async/await 语法

### async 函数

```rust
// async 函数返回 Future
async fn my_function() -> i32 {
    42
}

// 等价于
fn my_function() -> impl Future<Output = i32> {
    async { 42 }
}

// 调用
async fn caller() {
    let future = my_function();  // 创建 Future，未执行
    let result = future.await;   // 执行并等待结果
}
```

### async 块

```rust
// async 块创建匿名 Future
let future = async {
    let x = 1 + 2;
    let y = x * 3;
    y
};

let result = future.await;  // result = 9
```

### await 操作符

```rust
// await 只能在 async 函数中使用
async fn process() {
    let result1 = async_op1().await;  // 等待 op1 完成
    let result2 = async_op2(result1).await;  // 等待 op2 完成
}

// await 会暂停当前任务，但不阻塞线程
async fn example() {
    println!("Start");
    tokio::time::sleep(Duration::from_secs(1)).await;  // 暂停任务
    println!("End");  // 其他任务可以在这期间执行
}
```

### 编译器转换

async/await 在编译时被转换为状态机：

```rust
// 原始代码
async fn example() {
    let x = op1().await;
    let y = op2(x).await;
    op3(y).await;
}

// 编译器转换为状态机（简化）
enum ExampleFuture {
    State0,              // 初始状态，等待 op1
    State1(Result1),     // op1 完成，等待 op2
    State2(Result2),     // op2 完成，等待 op3
    Done,                // 完成
}

impl Future for ExampleFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        loop {
            match *self {
                State0 => {
                    match op1().poll(cx) {
                        Poll::Ready(x) => *self = State1(x),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                State1(x) => {
                    match op2(x).poll(cx) {
                        Poll::Ready(y) => *self = State2(y),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                State2(y) => {
                    match op3(y).poll(cx) {
                        Poll::Ready(_) => return Poll::Ready(()),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                Done => return Poll::Ready(()),
            }
        }
    }
}
```

## Pin 与 Unpin

### Pin 的作用

Pin 防止 Future 在内存中被移动：

```rust
// Future 可能包含自引用
struct SelfReferentialFuture {
    data: String,
    pointer: *const String,  // 指向 data
}

// 如果 Future 被移动，pointer 会失效
// Pin 防止这种情况
pub struct Pin<P> {
    pointer: P,
}
```

### 为什么需要 Pin？

```rust
// 自引用 Future 示例
async fn self_ref() {
    let data = vec![1, 2, 3];
    let ref_to_data = &data;  // 引用局部变量
    
    some_async_op().await;  // 异步操作可能移动 Future
    
    // 如果 Future 被移动，ref_to_data 会失效
    // Pin 确保 Future 在 await 期间不被移动
}
```

### Unpin Trait

```rust
// Unpin 表示类型可以安全移动
pub trait Unpin {}

// 大多数类型自动实现 Unpin
// 自引用类型需要手动处理

// ✅ 可以移动的类型
let mut v = vec![1, 2, 3];
let pin: Pin<&mut Vec<i32>> = Pin::new(&mut v);

// ❌ 不能移动的类型需要特殊处理
// 如：包含自引用的 Future
```

## 实际应用

### 异步网络 I/O

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    
    // 异步读取
    let n = stream.read(&mut buf).await.unwrap();
    
    // 异步写入
    stream.write_all(&buf[..n]).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("New client: {}", addr);
        
        tokio::spawn(handle_client(stream));
    }
}
```

### 异步文件 I/O

```rust
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn read_file(path: &str) -> String {
    let mut file = File::open(path).await.unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).await.unwrap();
    content
}

async fn write_file(path: &str, content: &str) {
    let mut file = File::create(path).await.unwrap();
    file.write_all(content.as_bytes()).await.unwrap();
}
```

### 异步 HTTP 客户端

```rust
use reqwest;
use tokio;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    match fetch_url("https://example.com").await {
        Ok(body) => println!("Body length: {}", body.len()),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Future 组合器

### map

```rust
use futures::future::{FutureExt, map};

async fn double_value() -> i32 {
    42
}

// 使用 map 转换结果
let doubled = double_value().map(|x| x * 2);
let result = doubled.await;  // result = 84
```

### then

```rust
use futures::future::FutureExt;

async fn first() -> i32 {
    1
}

async fn second(x: i32) -> i32 {
    x + 1
}

// 链式异步操作
let chained = first().then(|x| second(x));
let result = chained.await;  // result = 2
```

### join

```rust
use futures::future::join;

async fn op1() -> i32 { 1 }
async fn op2() -> i32 { 2 }

// 并发执行，等待所有完成
let (r1, r2) = join(op1(), op2()).await;
// r1 = 1, r2 = 2
```

### select

```rust
use futures::future::select;

async fn op1() -> i32 { 
    tokio::time::sleep(Duration::from_secs(1)).await;
    1
}

async fn op2() -> i32 {
    tokio::time::sleep(Duration::from_secs(2)).await;
    2
}

// 执行最先完成的
let result = select(op1(), op2()).await;
// result = Left((1, op2_future))  ← op1 先完成
```

## 异步错误处理

### Result Future

```rust
use std::result::Result;

async fn might_fail() -> Result<i32, String> {
    // 返回 Result 的异步函数
    Ok(42)
}

async fn handle() {
    match might_fail().await {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

### 使用 ? 操作符

```rust
async fn operation1() -> Result<i32, MyError> {
    Ok(1)
}

async fn operation2(x: i32) -> Result<i32, MyError> {
    Ok(x + 1)
}

async fn chained() -> Result<i32, MyError> {
    let r1 = operation1().await?;  // 失败时提前返回
    let r2 = operation2(r1).await?;
    Ok(r2)
}
```

### 错误传播

```rust
use anyhow::{Context, Result};

async fn read_config(path: &str) -> Result<Config> {
    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read config")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}
```

## 性能优化

### 避免不必要的 await

```rust
// ❌ 低效：频繁 await
async fn inefficient() -> i32 {
    let sum = 0;
    for i in 0..1000 {
        sum += async_op(i).await;  // 1000 次 await
    }
    sum
}

// ✅ 高效：批量处理
async fn efficient() -> i32 {
    let futures: Vec<_> = (0..1000)
        .map(async_op)
        .collect();
    
    let results = futures::future::join_all(futures).await;
    results.iter().sum()
}
```

### 使用 spawn

```rust
// ❌ 阻塞当前任务
async fn blocking_example() {
    for task in tasks {
        task.await;  // 顺序执行
    }
}

// ✅ 并发执行
async fn concurrent_example() {
    let handles: Vec<_> = tasks
        .map(|task| tokio::spawn(task))
        .collect();
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

## 小结

**Future 的核心机制：**
- `poll` 方法驱动 Future 执行
- `Pending` 表示未完成，`Ready` 表示完成
- Waker 负责唤醒运行时重新 poll

**async/await 的本质：**
- async 函数返回 Future
- await 暂停任务等待 Future 完成
- 编译器将 async/await 转换为状态机

**关键概念：**
- Pin: 防止 Future 被移动
- Waker: 通知运行时 Future 可继续执行
- 状态机: async/await 的底层实现

**下一步：**
下一节我们将学习 Tokio 运行时的原理和使用。

## 练习

### 练习 1：手动实现 Future

实现一个计数器 Future，每次 poll 计数一次，达到 5 时完成：

```rust
// TODO: 实现 CounterFuture
// poll 5 次后返回 Ready(5)
```

### 练习 2：异步链式操作

使用 Future 组合器实现链式异步操作：

```rust
// TODO: 使用 then 和 map 组合
// async_op1().then(async_op2).map(transform)
```

### 练习 3：异步错误处理

实现带错误处理的异步链：

```rust
// TODO: 使用 Result 和 ? 操作符
// 实现多步异步操作，失败时提前返回
```