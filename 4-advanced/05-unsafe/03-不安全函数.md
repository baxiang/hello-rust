## 外部函数接口（FFI）

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




