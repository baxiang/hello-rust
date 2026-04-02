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

---

---

## 下一步

[依赖管理](../4-依赖管理.md)