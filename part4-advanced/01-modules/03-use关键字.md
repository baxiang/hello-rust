## 20.6 模块文件分离

### 传统方式（mod.rs）

```bash
src/
├── lib.rs
├── front_of_house.rs
└── front_of_house/
    ├── mod.rs
    └── hosting.rs
```

```rust
// src/lib.rs
mod front_of_house;

// src/front_of_house.rs
pub mod hosting;

// src/front_of_house/mod.rs  (也可以放在这里)
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
pub fn seat_at_table() {}
```

### Rust 2018+ 简化方式

```bash
src/
├── lib.rs
└── front_of_house/
    ├── mod.rs
    └── hosting.rs
```

```rust
// src/lib.rs
mod front_of_house;  // 自动查找 front_of_house/mod.rs 或 front_of_house.rs

// src/front_of_house/mod.rs
pub mod hosting;  // 自动查找 hosting.rs

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

### 模块文件规则

```
┌─────────────────────────────────────────────────────┐
│              模块文件查找规则                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  声明：mod foo;                                     │
│                                                     │
│  编译器查找：                                        │
│  1. src/foo.rs                                      │
│  2. src/foo/mod.rs                                  │
│                                                     │
│  嵌套模块：mod foo { mod bar; }                     │
│                                                     │
│  编译器查找：                                        │
│  1. src/foo/bar.rs                                  │
│  2. src/foo/bar/mod.rs                              │
│                                                     │
└─────────────────────────────────────────────────────┘
```







