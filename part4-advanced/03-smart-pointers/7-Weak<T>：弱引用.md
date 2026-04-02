## 22.7 Weak<T>：弱引用

### 为什么需要 Weak

```
问题：循环引用导致内存泄漏

Rc<Node A> ──parent──> Rc<Node B>
     ▲                      │
     │                      │
     └─────child────────────┘

两个 Rc 互相引用，引用计数永远不会归零！

解决：使用 Weak 打破循环
```

### 弱引用用法

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,   // 弱引用父节点
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),  // 空弱引用
        children: RefCell::new(vec![]),
    });

    // 升级弱引用（返回 Option）
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());  // None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 设置父节点
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 升级并访问
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("Parent value: {}", parent.value);  // 5
    }
}
```

### Weak 方法

```rust
use std::rc::Weak;

let weak: Weak<i32> = Weak::new();

// 方法
weak.upgrade();           // Option<Rc<T>>，升级弱引用
weak.strong_count();      // 强引用计数
weak.weak_count();        // 弱引用计数
```




