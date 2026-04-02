## 23.5 互斥锁（Mutex）

### 基本用法

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    // 获取锁
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // 锁释放（Guard 离开作用域）

    println!("m = {:?}", m.lock().unwrap());  // 6
}
```

### MutexGuard

```rust
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(vec![1, 2, 3]);

    // lock() 返回 MutexGuard
    // MutexGuard 实现了 Deref 和 Drop
    {
        let mut guard = data.lock().unwrap();
        guard.push(4);
        // guard 离开作用域时自动释放锁
    }

    println!("{:?}", data.lock().unwrap());
}
```

### 多线程共享

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc + Mutex 实现线程安全共享
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("结果：{}", *counter.lock().unwrap());  // 10
}
```




