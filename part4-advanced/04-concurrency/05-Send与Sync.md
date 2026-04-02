## 23.8 Send 和 Sync Trait

### 线程安全 Trait

```
┌─────────────────────────────────────────────────────┐
│              Send 和 Sync                            │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Send: 可以在线程间转移所有权                        │
│  "这个类型的所有权可以安全地发送给另一个线程"        │
│                                                     │
│  Sync: 可以在线程间共享引用                          │
│  "&T 是 Send 的，即 T 是 Sync 的"                     │
│                                                     │
│  关系：                                              │
│  • T: Sync => &T: Send                              │
│  • T: Send + Sync => T 可以安全地在线程间共享        │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 示例

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    // 基本类型
    assert_send::<i32>();
    assert_sync::<i32>();

    // String
    assert_send::<String>();  // 可以转移所有权
    // assert_sync::<String>();  // ❌ 不是 Sync

    // Vec
    assert_send::<Vec<i32>>();
    assert_sync::<Vec<i32>>();  // 内容是 Sync 的

    // Rc 不是 Send 或 Sync
    // assert_send::<Rc<i32>>();  // ❌

    // Arc 是 Send 和 Sync
    use std::sync::Arc;
    assert_send::<Arc<i32>>();
    assert_sync::<Arc<i32>>();

    // Mutex
    use std::sync::Mutex;
    assert_send::<Mutex<i32>>();
    assert_sync::<Mutex<i32>>();  // T: Sync => Mutex<T>: Sync
}
```

### 自动实现

```rust
// 编译器自动为以下类型实现 Send 和 Sync：
// • 基本类型（i32, bool, 等）
// • 由 Send/Sync 类型组成的复合类型

// 手动实现（需要 unsafe）
unsafe impl Send for MyType {}
unsafe impl Sync for MyType {}
```

---

---

## 下一步

[完整示例](../9-完整示例.md)

---

## 返回

[返回目录](../../README.md)