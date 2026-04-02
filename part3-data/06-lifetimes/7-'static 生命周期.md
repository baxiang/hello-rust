## 17.7 'static 生命周期

### 什么是 'static

```rust
// 'static 表示生命周期是整个程序运行期间
// 字符串字面量是典型的 &'static str

let s: &'static str = "I have a static lifetime.";

// "I have a static lifetime." 存储在程序的静态区
// 在程序整个运行期间都有效
```

### 'static 使用场景

```rust
// 1. 字符串字面量
fn main() {
    let greeting: &'static str = "Hello, World!";
    println!("{}", greeting);
}

// 2. 常量
const VERSION: &'static str = "1.0.0";
static CONFIG: &'static str = "production";

// 3. 全局数据
use std::collections::HashMap;
use std::sync::Mutex;

static mut CACHE: Option<Mutex<HashMap<&'static str, i32>>> = None;

fn main() {
    println!("版本：{}", VERSION);
    println!("配置：{}", CONFIG);
}
```

### 'static 不等于永久

```
┌─────────────────────────────────────────────────────┐
│           'static 的正确理解                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  'static 的含义：                                    │
│  • 数据存储在程序的静态区                          │
│  • 在程序整个运行期间有效                          │
│  • 不会被释放（直到程序结束）                      │
│                                                     │
│  常见的 'static 引用：                               │
│  • 字符串字面量："hello"                           │
│  • 静态变量：static X: i32 = 5;                    │
│  • 常量：const X: i32 = 5;                         │
│                                                     │
│  注意：                                              │
│  • 'static 引用的数据本身不能被修改（除非用 Mutex） │
│  • 不是所有引用都能转换为'static                   │
│  • 不要滥用'static，优先使用更短的生命周期         │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

---

## 下一步

[生命周期约束](../8-生命周期约束.md)