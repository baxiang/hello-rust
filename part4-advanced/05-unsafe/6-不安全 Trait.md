## 24.6 不安全 Trait

### 定义和实现

```rust
// 不安全 Trait
unsafe trait Foo {
    fn dangerous_method(&self);
}

// 不安全实现
unsafe impl Foo for i32 {
    fn dangerous_method(&self) {
        println!("Calling for {}", self);
    }
}

fn main() {
    // 调用需要 unsafe
    unsafe {
        5.dangerous_method();
    }
}
```

### Send 和 Sync

```rust
// Send 和 Sync 是不安全 Trait
// 编译器不会自动实现，需要手动声明

struct MyType {
    ptr: *mut i32,
}

// 手动实现 Send（需要保证线程安全）
unsafe impl Send for MyType {}

// 手动实现 Sync（需要保证线程安全）
unsafe impl Sync for MyType {}
```

### 自定义分配器

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        println!("分配：size={}", layout.size());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("释放：size={}", layout.size());
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
```

---

---

## 下一步

[Union](../7-Union.md)