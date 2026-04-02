## 22.6 Arc<T>：线程安全引用计数

### Arc vs Rc

```
┌─────────────────────────────────────────────────────┐
│              Arc vs Rc                               │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Rc<T>：                                            │
│  • 单线程引用计数                                   │
│  • 轻量级，无锁                                     │
│  • 不能跨线程                                       │
│                                                     │
│  Arc<T>：                                           │
│  • 多线程引用计数                                   │
│  • 原子操作，有锁开销                               │
│  • 可以跨线程                                       │
│  • 用法与 Rc 相同                                   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 基本用法

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    // 线程安全的引用计数
    let data = Arc::new(5);

    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("data = {}", data);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("引用计数：{}", Arc::strong_count(&data));
}
```

### Arc<Mutex<T>>：线程安全共享

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
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

---

---

## 下一步

[Weak<T>：弱引用](../7-Weak<T>：弱引用.md)