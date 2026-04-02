## 23.6 RwLock：读写锁

### RwLock vs Mutex

```
┌─────────────────────────────────────────────────────┐
│              RwLock vs Mutex                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Mutex：                                            │
│  • 独占锁，同一时间只有一个线程访问                 │
│  • 适合写多读少                                     │
│                                                     │
│  RwLock (Read-Write Lock)：                         │
│  • 多个读锁可以同时存在                            │
│  • 写锁独占                                         │
│  • 适合读多写少                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 基本用法

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(5));

    // 多个读锁
    let reader1 = data.read().unwrap();
    let reader2 = data.read().unwrap();
    println!("读取：{} {}", *reader1, *reader2);
    drop(reader1);
    drop(reader2);

    // 写锁（独占）
    let mut writer = data.write().unwrap();
    *writer += 1;
    println!("写入后：{}", *writer);
}
```

### 读写优先级

```rust
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(RwLock::new(0));

    // 读线程
    for i in 0..5 {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            for _ in 0..3 {
                let val = data.read().unwrap();
                println!("读线程 {}：{}", i, *val);
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    // 写线程
    for i in 0..2 {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            for j in 1..=3 {
                let mut val = data.write().unwrap();
                *val = i * 10 + j;
                println!("写线程：{}", *val);
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    thread::sleep(Duration::from_millis(500));
}
```

---

---

## 下一步

[原子类型](../7-原子类型.md)