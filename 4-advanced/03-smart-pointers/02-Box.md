## Box<T>：堆上分配

### Box 语法

**概念名称：** `Box<T>` 在堆上分配数据，栈上存储指针。

```
语法结构：
┌──────────────────────────────────────┐
│  let 变量 = Box::new(值);             │
│             ↑        ↑                │
│             构造函数  堆上的数据        │
│                                       │
│  let b = Box::new(5);                │
│                                       │
│  解引用：                              │
│  *b      → 获取值                     │
│  b.deref() → 解引用方法               │
│                                       │
│  使用场景：                            │
│  • 递归类型（大小未知）               │
│  • 大对象（避免栈溢出）               │
│  • Trait 对象（dyn Trait）           │
└──────────────────────────────────────┘
```

### 最简示例

```rust
fn main() {
    let b = Box::new(5);
    println!("{}", *b);  // 5
}
```

### 基本用法

```rust
fn main() {
    // 在栈上
    let stack_val = 5;

    // 在堆上
    let heap_val = Box::new(5);

    println!("栈：{}", stack_val);
    println!("堆：{}", heap_val);

    // 自动解引用
    assert_eq!(5, *heap_val);
}  // heap_val 离开作用域，堆上内存自动释放
```

### 使用场景 1：递归类型

```rust
// ❌ 错误：编译时无法确定大小
// enum List {
//     Cons(i32, List),  // List 大小未知
//     Nil,
// }

// ✅ 正确：使用 Box
enum List {
    Cons(i32, Box<List>),  // Box 大小固定
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    println!("{:?}", list);
}
```

### 使用场景 2：大对象

```rust
fn main() {
    // 可能栈溢出
    // let big_array = [0u8; 10 * 1024 * 1024];  // 10MB

    // 堆上分配
    let big_data: Box<[u8]> = vec![0; 10 * 1024 * 1024].into_boxed_slice();
    println!("分配了 {} 字节", big_data.len());
}
```

### 使用场景 3：Trait 对象

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("汪!"); }
}

impl Animal for Cat {
    fn speak(&self) { println!("喵!"); }
}

fn main() {
    // 不同类型但相同 trait，需要 Box
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.speak();
    }
}
```




