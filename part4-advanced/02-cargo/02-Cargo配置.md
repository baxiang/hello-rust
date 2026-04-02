## 21.3 Cargo.toml 详解

### 完整配置示例

```toml
[package]
# 基本信息
name = "my_crate"           # 包名（发布到 crates.io 的名称）
version = "0.1.0"           # 语义化版本号
edition = "2021"            # Rust 版本

# 可选信息
authors = ["Your Name <you@example.com>"]
description = "A brief description of my crate"
license = "MIT"             # 许可证
repository = "https://github.com/user/repo"
homepage = "https://my-crate.com"
documentation = "https://docs.rs/my_crate"
keywords = ["keyword1", "keyword2"]  # 最多 5 个
categories = ["development-tools"]
readme = "README.md"

# 排除文件
exclude = ["tests/", "benches/"]
include = ["src/", "LICENSE"]

[dependencies]
# 见下一节

[dev-dependencies]
# 开发依赖

[build-dependencies]
# 构建依赖
```

### 版本号规则（语义化版本）

```toml
version = "1.2.3"
#       │ │ │
#       │ │ └─ PATCH: 向后兼容的问题修复
#       │ └─── MINOR: 向后兼容的新功能
#       └───── MAJOR: 不兼容的变更

# 版本要求
serde = "1.0"      # >=1.0.0, <2.0.0
serde = "1.0.0"    # >=1.0.0, <1.1.0
serde = "^1.0.0"   # >=1.0.0, <1.1.0
serde = "~1.0.0"   # >=1.0.0, <1.0.99
serde = "=1.0.0"   # 精确版本
serde = "*"        # 任意版本（不推荐）
```






## 21.4 依赖管理

### 添加依赖

```toml
[dependencies]
# 1. 从 crates.io 获取
serde = "1.0"
serde_json = "1.0.100"

# 2. 指定功能
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

# 3. 可选依赖
serde = { version = "1.0", optional = true }

# 4. 重命名依赖（解决冲突）
rustix_37 = { package = "rustix", version = "0.37" }

# 5. 从 Git 获取
serde = { git = "https://github.com/serde-rs/serde" }
serde = { git = "https://github.com/user/repo", branch = "main" }
serde = { git = "https://github.com/user/repo", tag = "v1.0" }

# 6. 从本地路径获取
my_lib = { path = "../my_lib" }

# 7. 指定版本范围
regex = "^1.5"      # >=1.5.0, <2.0.0
tokio = "~1.28"     # >=1.28.0, <1.29.0
```

### 开发依赖

```toml
[dev-dependencies]
# 仅测试和示例需要，不会包含在最终产物中
pretty_assertions = "1.4"
criterion = "0.5"
mockito = "1.0"
```

### 构建依赖

```toml
[build-dependencies]
# 编译时运行（如生成代码）
cc = "1.0"
bindgen = "0.68"
```

### 目标特定依赖

```toml
# 仅在特定平台使用
[target.'cfg(unix)'.dependencies]
libc = "0.2"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winsock2"] }

# 特定架构
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
```

### 使用 cargo add（推荐）

```bash
# 添加基本依赖
cargo add serde

# 添加带功能的依赖
cargo add serde --features derive

# 添加开发依赖
cargo add --dev pretty_assertions

# 添加本地依赖
cargo add ../my_lib

# 添加 Git 依赖
cargo add serde --git https://github.com/serde-rs/serde

# 添加可选依赖
cargo add serde --optional
```







