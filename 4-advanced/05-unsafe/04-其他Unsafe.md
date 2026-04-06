## 可变静态变量

### 定义和访问

```rust
// 可变静态变量
static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);  // 3
    }
}
```

### 线程安全替代方案

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

// ✅ 更安全的做法：原子静态变量
static SAFE_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Safe counter: {}", SAFE_COUNTER.load(Ordering::SeqCst));
}
```

### 使用 lazy_static

```rust
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_MUTEX: Mutex<i32> = Mutex::new(0);
}

fn main() {
    let mut data = GLOBAL_MUTEX.lock().unwrap();
    *data += 1;
    println!("value: {}", *data);
}
```






## 不安全 Trait

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






## Union

### 定义和使用

```rust
#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
    bytes: [u8; 4],
}

fn main() {
    // 创建 union
    let u = MyUnion { i: 42 };

    // 访问 union 字段需要 unsafe
    unsafe {
        println!("i: {}", u.i);
        println!("f: {}", u.f);  // 同样的位，解释为浮点数
        println!("bytes: {:?}", u.bytes);
    }
}
```

### Union 应用

```rust
#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process(data: IntOrFloat, is_int: bool) {
    unsafe {
        if is_int {
            println!("整数：{}", data.i);
        } else {
            println!("浮点数：{}", data.f);
        }
    }
}
```







