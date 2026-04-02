## 22.5 组合使用：Rc<RefCell<T>>

### 共享 + 可变

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Student {
    name: String,
}

fn main() {
    // 可修改的共享数据
    let student = Rc::new(RefCell::new(Student {
        name: String::from("Alice"),
    }));

    // 多个所有者
    let s1 = Rc::clone(&student);
    let s2 = Rc::clone(&student);

    // 通过任一引用修改
    s1.borrow_mut().name = String::from("Bob");

    // 所有引用看到相同的值
    println!("s1: {:?}", s1.borrow());  // Bob
    println!("s2: {:?}", s2.borrow());  // Bob
}
```

### 完整示例：图结构

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            children: RefCell::new(vec![]),
        }
    }

    fn add_child(&self, child: Rc<Node>) {
        self.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = Rc::new(Node::new(1));
    let child1 = Rc::new(Node::new(2));
    let child2 = Rc::new(Node::new(3));

    root.add_child(Rc::clone(&child1));
    root.add_child(Rc::clone(&child2));

    println!("root: {:?}", root);
    println!("root children: {:?}", root.children.borrow());
}
```

---

---

## 下一步

[Arc<T>：线程安全引用计数](../6-Arc<T>：线程安全引用计数.md)