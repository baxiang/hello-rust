## 集成测试

### 测试目录结构

```
project/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
│   ├── common/
│   │   └── mod.rs
│   └── integration_test.rs
└── examples/
```

### 集成测试示例

```rust
// tests/integration_test.rs
use my_crate::*;

#[test]
fn test_public_api() {
    let result = public_function(5);
    assert_eq!(result, 10);
}

#[test]
fn test_multiple_calls() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(5, 5), 10);
    assert_eq!(add(0, 0), 0);
}
```

### 共享测试工具

```rust
// tests/common/mod.rs
use my_crate::User;

pub fn setup() -> TestContext {
    TestContext {
        users: vec![
            User::new("Alice".to_string(), 25),
            User::new("Bob".to_string(), 30),
        ],
    }
}

pub struct TestContext {
    pub users: Vec<User>,
}

// tests/integration_test.rs
mod common;

#[test]
fn test_with_setup() {
    let ctx = common::setup();
    assert_eq!(ctx.users.len(), 2);
    assert_eq!(ctx.users[0].name(), "Alice");
}
```







