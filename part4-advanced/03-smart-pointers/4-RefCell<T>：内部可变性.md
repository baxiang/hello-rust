## 22.4 RefCell<T>：内部可变性

### 什么是内部可变性

```
┌─────────────────────────────────────────────────────┐
│              内部可变性                              │
├─────────────────────────────────────────────────────┤
│                                                     │
│  外部可变性（Rust 默认）：                           │
│  • 需要 mut 才能修改                                 │
│  • 借用检查在编译时                                  │
│                                                     │
│  内部可变性（RefCell）：                             │
│  • 不需要 mut 也能修改                               │
│  • 借用检查在运行时                                  │
│  • 违反规则会 panic                                  │
│                                                     │
│  使用场景：                                          │
│  • 缓存                                              │
│  • 惰性初始化                                        │
│  • 复杂数据结构                                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 基本用法

```rust
use std::cell::RefCell;

fn main() {
    // 不需要 mut
    let data = RefCell::new(5);

    // 可变借用
    *data.borrow_mut() += 1;

    println!("data = {}", data.borrow());  // 6

    // 不可变借用
    {
        let borrowed = data.borrow();
        println!("{:?}", *borrowed);
    }  // 借用结束

    // 可变借用
    {
        let mut borrowed = data.borrow_mut();
        *borrowed += 10;
    }
}
```

### borrow vs borrow_mut

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    // borrow(): 不可变借用（多个同时存在）
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("{:?} {:?}", *r1, *r2);
    drop(r1);
    drop(r2);

    // borrow_mut(): 可变借用（独占）
    let mut w = data.borrow_mut();
    w.push(4);
    drop(w);

    // try_borrow(): 尝试借用（不 panic）
    match data.try_borrow() {
        Ok(borrowed) => println!("{:?}", *borrowed),
        Err(e) => println!("借用失败：{}", e),
    }
}
```

---

---

## 下一步

[组合使用：Rc<RefCell<T>>](../5-组合使用：Rc<RefCell<T>>.md)