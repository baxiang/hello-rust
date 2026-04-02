## 20.5 use 关键字：引入模块

### 基本用法

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
}

// 方式 1：引入模块
use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}

// 方式 2：引入具体项
use front_of_house::hosting::add_to_waitlist;

fn serve() {
    add_to_waitlist();  // 直接使用
}
```

### 使用 as 重命名

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// 解决命名冲突
use std::fmt::Debug as FmtDebug;
use std::fmt::Display as FmtDisplay;
```

### 重新导出（pub use）

```rust
// lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 重新导出，让外部使用更简洁的路径
pub use crate::front_of_house::hosting;

// 外部代码可以这样用：
// use my_crate::hosting;  // 而不是 my_crate::front_of_house::hosting
```

### 使用 * 导入所有项

```rust
// 导入模块所有公共项
use std::collections::*;

fn main() {
    let v: Vec<i32> = Vec::new();      // 来自 std::collections
    let h: HashMap<i32, i32> = HashMap::new();
}

// 注意：不推荐在库代码中使用 * 导入
// 原因：
// • 难以追踪名称来源
// • 可能导致命名冲突
// • 降低代码可读性
```

---

---

## 下一步

[模块文件分离](../6-模块文件分离.md)