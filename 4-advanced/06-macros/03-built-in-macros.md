## 内置宏

### stringify!

```rust
macro_rules! debug_print {
    ($($arg:tt)*) => {
        println!("{} = {:?}", stringify!($($arg)*), $($arg)*);
    };
}

fn main() {
    let x = 42;
    debug_print!(x);  // 输出：x = 42
    debug_print!(2 + 2);  // 输出：2 + 2 = 4
}
```

### concat!

```rust
fn main() {
    let s = concat!("Hello", " ", "World");
    println!("{}", s);  // Hello World

    // 常用于生成标识符
    let field_name = concat!("field_", line!());
    println!("{}", field_name);  // field_5
}
```

### compile_error!

```rust
// 条件编译错误
#[cfg(not(target_os = "linux"))]
compile_error!("This code only works on Linux!");

// 验证宏参数
macro_rules! require_feature {
    () => {
        compile_error!("Feature 'xxx' is required");
    };
}
```

### env! 和 option_env!

```rust
fn main() {
    // 环境变量（不存在则编译错误）
    let home = env!("HOME");
    println!("Home: {}", home);

    // 环境变量（返回 Option）
    let opt = option_env!("MY_VAR");
    println!("Optional: {:?}", opt);
}
```

### include! 系列

```rust
// 包含文件内容
mod generated {
    include!("generated.rs");
}

// 包含字节
const LOGO: &[u8] = include_bytes!("logo.png");

// 包含字符串
const README: &str = include_str!("../README.md");
```






## 递归宏

### 简单递归

```rust
macro_rules! factorial {
    (0) => { 1 };
    ($n:expr) => {
        $n * factorial!($n - 1)
    };
}

fn main() {
    println!("5! = {}", factorial!(5));  // 120
}
```

### 列表处理

```rust
// 计算参数个数
macro_rules! count {
    () => (0);
    ($x:tt $($rest:tt)*) => (1 + count!($($rest)*));
}

fn main() {
    println!("{}", count!(a b c d e));  // 5
}
```

### 复杂递归

```rust
// 实现类似 Python 的 print
macro_rules! pyprint {
    ($val:expr) => {
        print!("{:?}", $val);
    };
    ($val:expr, $($rest:tt)*) => {
        print!("{:?}, ", $val);
        pyprint!($($rest)*);
    };
}

fn main() {
    pyprint!(1, "hello", 3.14);  // 1, "hello", 3.14
    println!();
}
```







