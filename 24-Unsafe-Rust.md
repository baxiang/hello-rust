# 第 24 章：Unsafe Rust 详解

> 绕过编译器检查，直接操作底层

---

## 24.1 什么是 Unsafe Rust

```
┌─────────────────────────────────────────────────────┐
│          Unsafe Rust 能做什么                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Safe Rust 无法做到的事：                            │
│  1. 解引用裸指针                                    │
│  2. 调用不安全函数或方法                            │
│  3. 访问或修改可变静态变量                          │
│  4. 实现不安全 Trait                                │
│  5. 访问 union 的字段                               │
│                                                     │
│  何时使用 Unsafe？                                   │
│  • 与 C 代码交互（FFI）                              │
│  • 操作系统系统调用                                  │
│  • 性能关键代码                                      │
│  • 实现底层抽象（如智能指针）                        │
│                                                     │
│  重要原则：                                          │
│  • 最小化 unsafe 块                                  │
│  • 在安全接口内封装 unsafe                           │
│  • 文档说明为什么 unsafe 是安全的                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Unsafe 块

```rust
fn main() {
    // unsafe 块内可以执行不安全操作
    unsafe {
        // 危险操作...
    }
}

// unsafe 不意味着"一定危险"
// 而是"编译器无法保证安全，程序员需要自己保证"
```

---

## 24.2 裸指针

### 创建裸指针

```rust
fn main() {
    let mut num = 5;

    // 创建裸指针
    let r1 = &num as *const i32;  // 不可变裸指针
    let r2 = &mut num as *mut i32;  // 可变裸指针

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);

    // 裸指针可以是 null
    let null_ptr: *mut i32 = std::ptr::null_mut();
    println!("null: {:?}", null_ptr);
}
```

### 裸指针 vs 引用

```
┌─────────────────────────────────────────────────────┐
│          裸指针 vs 引用                              │
├─────────────────────────────────────────────────────┤
│                                                     │
│  引用 (&T, &mut T)：                                │
│  • 始终有效（不能是 null）                          │
│  • 遵循借用规则                                     │
│  • 自动清理                                         │
│  • 安全                                             │
│                                                     │
│  裸指针 (*const T, *mut T)：                        │
│  • 可以是 null                                      │
│  • 不遵循借用规则                                   │
│  • 没有自动清理                                     │
│  • 解引用需要 unsafe                                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 解引用裸指针

```rust
fn main() {
    let mut num = 5;
    let ptr = &mut num as *mut i32;

    // 解引用需要 unsafe
    unsafe {
        println!("解引用：{}", *ptr);
        *ptr = 10;
        println!("修改后：{}", *ptr);
    }

    println!("num = {}", num);  // 10
}
```

### 裸指针操作

```rust
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_mut_ptr();

    unsafe {
        // 读取
        println!("{}", ptr.read());

        // 写入
        ptr.write(10);

        // 偏移
        let second = ptr.add(1);
        println!("{}", second.read());

        // 复制
        std::ptr::copy_nonoverlapping(ptr, ptr.add(1), 4);
    }

    println!("{:?}", arr);
}
```

---

## 24.3 不安全函数

### 定义和调用

```rust
// 不安全函数
unsafe fn dangerous() {
    println!("这是不安全函数");
}

fn main() {
    // 调用需要 unsafe 块
    unsafe {
        dangerous();
    }
}
```

### 创建安全抽象

```rust
use std::slice;

/// 安全地分割可变切片
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len, "mid 超出范围");

    // 内部使用 unsafe，但对外是安全的
    unsafe {
        let ptr = slice.as_mut_ptr();
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("left: {:?}, right: {:?}", left, right);
}
```

### 标准库中的 unsafe

```rust
// 常见的 unsafe 函数
fn main() {
    let mut v = vec![1, 2, 3];

    // 不安全：不检查边界
    unsafe {
        println!("{}", v.get_unchecked(0));  // 1
    }

    // 不安全：需要保证有效性
    let ptr = v.as_mut_ptr();
    unsafe {
        ptr.write(10);
    }

    println!("{:?}", v);
}
```

---

## 24.4 外部函数接口（FFI）

### 调用 C 函数

```rust
extern "C" {
    fn abs(input: i32) -> i32;
    fn printf(format: *const i8, ...) -> i32;
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
}

fn main() {
    unsafe {
        println!("绝对值：{}", abs(-3));
    }
}
```

### 导出给 C 使用

```rust
// 禁用名称修饰
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("从 C 调用！");
}

// 防止被链接器移除
#[no_mangle]
pub static mut STATIC_DATA: i32 = 42;
```

### C 字符串交互

```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn puts(s: *const c_char) -> i32;
}

fn main() {
    // CString 确保没有内部 null 字节
    let s = CString::new("Hello from Rust!").unwrap();

    unsafe {
        puts(s.as_ptr());
    }

    // s 离开作用域，内存自动释放
}
```

### 完整 FFI 示例

```rust
// Rust 代码
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

// C 代码 (C 文件)
// extern int rust_add(int a, int b);
// int result = rust_add(2, 3);
```

---

## 24.5 可变静态变量

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

---

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

## 24.7 Union

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

---

## 24.8 完整示例

### 示例 1：自定义 Vec

```rust
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        assert!(std::mem::size_of::<T>() > 0);
        MyVec {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.len == self.cap {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            self.grow(new_cap);
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }

    fn grow(&mut self, new_cap: usize) {
        unsafe {
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            let new_ptr = if self.cap == 0 {
                alloc(new_layout) as *mut T
            } else {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                std::alloc::realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T
            };
            self.ptr = new_ptr;
            self.cap = new_cap;
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }
            if self.cap > 0 {
                let layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut v = MyVec::<i32>::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("v[0] = {:?}", v.get(0));
    println!("v[1] = {:?}", v.get(1));
}
```

### 示例 2：包装 C 库

```rust
use std::os::raw::{c_int, c_void};
use std::ptr;

// 模拟 C 库函数
extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

// 安全包装
pub struct Buffer {
    ptr: *mut u8,
    len: usize,
}

impl Buffer {
    pub fn new(len: usize) -> Self {
        unsafe {
            let ptr = malloc(len) as *mut u8;
            if ptr.is_null() {
                panic!("内存分配失败");
            }
            // 初始化内存
            ptr::write_bytes(ptr, 0, len);
            Buffer { ptr, len }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut c_void);
        }
    }
}

fn main() {
    let mut buffer = Buffer::new(10);
    buffer.as_mut_slice().copy_from_slice(b"hello");
    println!("{:?}", buffer.as_slice());
}
```

---

## 24.9 常见错误

### 错误 1：悬垂指针

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x as *const i32;
    }

    unsafe {
        // ❌ 错误：x 已释放
        // println!("r: {}", *r);  // 未定义行为
    }
}
```

### 错误 2：空指针解引用

```rust
fn main() {
    let ptr: *const i32 = std::ptr::null();

    unsafe {
        // ❌ 错误：空指针解引用
        // println!("{}", *ptr);  // 崩溃
    }
}
```

### 错误 3：数据竞争

```rust
static mut DATA: i32 = 0;

fn main() {
    // ❌ 错误：多线程访问可变静态变量
    // unsafe { DATA += 1; }  // 数据竞争
}
```

### 错误 4：无效内存访问

```rust
fn main() {
    let arr = [1, 2, 3];
    let ptr = arr.as_ptr();

    unsafe {
        // ❌ 错误：越界访问
        // println!("{}", *ptr.add(10));
    }
}
```

---

## 24.10 Unsafe 编程指南

### 最佳实践

```
┌─────────────────────────────────────────────────────┐
│              Unsafe 最佳实践                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. 最小化                                           │
│     • 只在必要时使用 unsafe                          │
│     • 尽可能缩小 unsafe 块的范围                      │
│                                                     │
│  2. 封装                                             │
│     • 在安全接口内封装 unsafe                        │
│     • 对外暴露安全的 API                             │
│                                                     │
│  3. 文档                                             │
│     • 说明为什么 unsafe 是安全的                     │
│     • 列出所有需要维护的不变量                       │
│                                                     │
│  4. 测试                                             │
│     • 充分测试 unsafe 代码                           │
│     • 使用 Miri 检测未定义行为                       │
│                                                     │
│  5. 审计                                             │
│     • 标记 unsafe 代码块                             │
│     • 定期审查 unsafe 代码                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 使用 Miri 检测

```bash
# 安装 Miri
rustup component add miri

# 运行 Miri
cargo miri run
cargo miri test

# Miri 可以检测：
# • 悬垂指针
# • 空指针解引用
# • 越界访问
# • 数据竞争
```

---

## 24.11 练习

### 练习 1：安全包装

创建一个安全的裸指针包装类型，确保不会悬垂。

### 练习 2：FFI 调用

使用 FFI 调用一个系统函数（如 libc 中的函数）。

### 练习 3：简单内存池

实现一个简单的内存池，使用 unsafe 管理内存。

---

## 24.12 小结

本章我们学习了：

- ✅ 裸指针的创建和解引用
- ✅ 不安全函数的定义和调用
- ✅ 外部函数接口（FFI）
- ✅ 可变静态变量
- ✅ 不安全 Trait
- ✅ Union 类型
- ✅ Unsafe 最佳实践

### Unsafe 规则速查

| 规则 | 说明 |
|------|------|
| 最小化 | 只在必要时使用 |
| 封装 | 安全接口封装 unsafe |
| 文档 | 说明安全性保证 |
| 测试 | 充分测试，使用 Miri |

---

## 下一章

[第 25 章：宏](25-宏.md)
