## 20.8 Cargo.toml 配置

### 基本配置

```toml
[package]
name = "my_package"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A brief description"
license = "MIT"
repository = "https://github.com/user/repo"
keywords = ["keyword1", "keyword2"]
categories = ["development-tools"]
```

### 定义多个二进制

```toml
[[bin]]
name = "my_tool"
path = "src/bin/my_tool.rs"

[[bin]]
name = "my_server"
path = "src/bin/my_server.rs"
```

### 库配置

```toml
[lib]
name = "my_lib"
path = "src/lib.rs"
crate-type = ["lib", "rlib", "cdylib"]
```

---

---

## 下一步

[常见错误](../9-常见错误.md)