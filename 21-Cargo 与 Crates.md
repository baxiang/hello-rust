# 第 21 章：Cargo 与 Crates.io 详解

> Rust 的包管理器和生态系统

---

## 21.1 Cargo 是什么

```
┌─────────────────────────────────────────────────────┐
│              Cargo 能做什么                          │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📦 依赖管理     - 自动下载和编译依赖               │
│  🔨 构建项目     - 编译、链接、生成二进制           │
│  🧪 运行测试     - 执行单元测试和集成测试           │
│  📚 生成文档     - 自动生成 API 文档                 │
│  🚀 发布 crate   - 发布到 crates.io                 │
│  ✅ 代码检查     - 格式化、lint                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Cargo vs 其他包管理器

| 工具 | 语言 | 特点 |
|------|------|------|
| Cargo | Rust | 内置、快速、可靠 |
| npm | JavaScript | 生态最大、依赖多 |
| pip | Python | 简单易用 |
| Maven | Java | 配置复杂 |

---

## 21.2 创建项目

### 新建二进制项目

```bash
# 创建可执行程序
cargo new my_app
cd my_app

# 项目结构
my_app/
├── Cargo.toml      # 项目清单
├── src/
│   └── main.rs     # 程序入口
└── .git/
```

### 新建库项目

```bash
# 创建库
cargo new my_lib --lib
cd my_lib

# 项目结构
my_lib/
├── Cargo.toml      # 项目清单
├── src/
│   └── lib.rs      # 库入口
└── .git/
```

### 在现有目录初始化

```bash
# 已有代码，初始化 Cargo
cd existing_project
cargo init

# 指定类型
cargo init --lib     # 初始化为库
cargo init --bin     # 初始化为二进制
```

---

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

---

## 21.5 常用 Cargo 命令

### 构建命令

```bash
# 调试构建（默认）
cargo build
# 输出：target/debug/

# 发布构建（优化）
cargo build --release
# 输出：target/release/
# 优化级别：-O3

# 快速检查（不生成二进制）
cargo check
# 用于快速验证代码

# 清理构建文件
cargo clean

# 更新依赖
cargo update
cargo update -p serde  # 更新特定包
```

### 运行命令

```bash
# 运行程序
cargo run
cargo run --release

# 传递参数
cargo run -- arg1 arg2

# 运行特定二进制
cargo run --bin my_tool
```

### 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行测试（显示输出）
cargo test -- --nocapture

# 单线程运行
cargo test -- --test-threads=1

# 只运行文档测试
cargo test --doc

# 只运行单元测试
cargo test --lib
```

### 代码质量

```bash
# 格式化代码
cargo fmt
cargo fmt -- --check  # 检查格式

# 代码 lint
cargo clippy
cargo clippy -- -D warnings  # 警告当错误

# 生成文档
cargo doc
cargo doc --open      # 生成并打开
cargo doc --no-deps   # 不包含依赖
```

### 依赖管理

```bash
# 查看依赖树
cargo tree
cargo tree -d         # 查看重复依赖
cargo tree -i serde   # 查看谁依赖 serde

# 验证
cargo verify-project

# 查看元数据
cargo metadata
```

---

## 21.6 常用 Crates 推荐

### 序列化

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"      # JSON
serde_yaml = "0.9"      # YAML
toml = "0.8"            # TOML
bincode = "1.3"         # 二进制格式
```

### Web 开发

```toml
[dependencies]
# 框架
axum = "0.7"            # 推荐新项目
actix-web = "4"         # 高性能
rocket = "0.5"          # 易用

# HTTP 客户端
reqwest = { version = "0.11", features = ["json"] }
ureq = "2.9"            # 轻量级

# WebSocket
tokio-tungstenite = "0.21"
```

### 异步编程

```toml
[dependencies]
# 运行时
tokio = { version = "1", features = ["full"] }
async-std = "1.12"

# 工具
futures = "0.3"
async-trait = "0.1"
```

### 命令行

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
colored = "2"
indicatif = "0.17"      # 进度条
dialoguer = "0.11"      # 交互对话框
tabled = "0.14"         # 表格输出
```

### 日志与错误

```toml
[dependencies]
# 日志
log = "0.4"
env_logger = "0.10"
tracing = "0.1"         # 结构化日志
tracing-subscriber = "0.3"

# 错误处理
anyhow = "1.0"          # 应用层
thiserror = "1.0"       # 库
```

### 工具库

```toml
[dependencies]
# 日期时间
chrono = { version = "0.4", features = ["serde"] }
time = "0.3"

# 随机数
rand = "0.8"

# 正则
regex = "1"

# UUID
uuid = { version = "1", features = ["v4", "serde"] }

# 环境变量
dotenvy = "0.15"

# 加密
sha2 = "0.10"
hmac = "0.12"
aes-gcm = "0.10"
```

### 数据库

```toml
[dependencies]
# SQL
sqlx = { version = "0.7", features = ["postgres", "mysql", "sqlite"] }
diesel = { version = "2", features = ["postgres"] }

# NoSQL
redis = "0.23"
mongodb = "2"
```

---

## 21.7 构建配置（Profiles）

### 默认配置

```toml
[profile.dev]
opt-level = 0           # 无优化
debug = true            # 调试信息
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'

[profile.release]
opt-level = 3           # 最大优化
debug = false
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'

[profile.test]
opt-level = 0
debug = true

[profile.bench]
opt-level = 3
```

### 自定义配置

```toml
# 快速发布构建（平衡编译速度和运行速度）
[profile.release]
opt-level = 2
lto = "thin"

# 最小二进制大小
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

# 开发环境启用部分优化
[profile.dev]
opt-level = 1

[profile.dev.package."*"]  # 依赖的优化
opt-level = 3
```

---

## 21.8 Workspaces

### 创建 Workspace

```toml
# 根目录 Cargo.toml
[workspace]
members = [
    "crate1",
    "crate2",
    "crate3",
]
resolver = "2"  # 使用 Rust 2021 依赖解析器
```

### 项目结构

```
my_workspace/
├── Cargo.toml          # Workspace 配置
├── Cargo.lock          # 共享锁文件
├── crate1/
│   ├── Cargo.toml
│   └── src/
├── crate2/
│   ├── Cargo.toml
│   └── src/
└── target/             # 共享构建目录
```

### 共享依赖

```toml
# 根目录 Cargo.toml
[workspace]
members = ["crate1", "crate2"]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = "1.0"
anyhow = "1.0"

# crate1/Cargo.toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }

# crate2/Cargo.toml
[dependencies]
serde = { workspace = true }
anyhow = { workspace = true }
```

---

## 21.9 发布 Crate

### 准备工作

```bash
# 1. 注册 crates.io 账号
# 访问 https://crates.io/login

# 2. 获取 API token
# 登录后访问 Account Settings

# 3. 本地登录
cargo login <your_api_token>
```

### 发布前检查

```bash
# 检查包内容
cargo package --list

# 本地测试打包
cargo package

# 测试发布（不实际上传）
cargo publish --dry-run
```

### 发布

```bash
# 发布到 crates.io
cargo publish

# 发布到私有 registry
cargo publish --registry my-registry
```

### 版本管理

```bash
# 1. 更新 Cargo.toml 中的 version
# 2. 提交更改
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# 3. 打标签
git tag v0.2.0

# 4. 推送
git push
git push --tags

# 5. 发布
cargo publish
```

---

## 21.10 配置优化

### 配置镜像（国内用户）

```toml
# ~/.cargo/config.toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

# 或者使用 rsproxy
[source.rsproxy]
registry = "sparse+https://rsproxy.cn/crates.io-index/"
```

### 编译加速

```toml
# ~/.cargo/config.toml
# 使用 sccache 缓存
[build]
rustc-wrapper = "sccache"

# 或者使用 ccache
rustc-wrapper = "ccache"
```

```bash
# 安装 sccache
cargo install sccache

# 启用
export RUSTC_WRAPPER=sccache
```

### 增加并行编译

```toml
# ~/.cargo/config.toml
[build]
jobs = 4  # 并行任务数
```

---

## 21.11 常见问题

### 问题 1：依赖冲突

```
error: failed to select a version for `crate_name`
```

**解决**：
```bash
# 更新依赖
cargo update

# 查看冲突
cargo tree -d

# 指定兼容版本
cargo add crate_name@0.9
```

### 问题 2：编译慢

**解决**：
```bash
# 1. 使用 sccache 缓存
cargo install sccache

# 2. 减少优化级别
# Cargo.toml
[profile.dev]
opt-level = 0

# 3. 使用 mimalloc 链接器
cargo install mimalloc
```

### 问题 3：Cargo.lock 冲突

**解决**：
```bash
# 重新生成锁文件
rm Cargo.lock
cargo update
```

### 问题 4：下载依赖慢

**解决**：配置国内镜像（见 21.10 节）

---

## 21.12 练习

### 练习 1：依赖管理

创建一个新项目，添加以下依赖：
- serde（带 derive 功能）
- serde_json
- tokio（带 full 功能）

### 练习 2：Profile 配置

配置发布模式：
- 启用 LTO
- 优化为最小二进制大小
- 移除调试符号

### 练习 3：Workspace

创建一个 workspace，包含两个 crate：
- `common`：共享代码库
- `app`：使用 common 的应用

---

## 21.13 小结

本章我们学习了：

- ✅ Cargo 基础概念
- ✅ Cargo.toml 配置
- ✅ 依赖管理
- ✅ 常用 Cargo 命令
- ✅ 常用 Crates 推荐
- ✅ 构建配置（Profiles）
- ✅ Workspaces
- ✅ 发布 Crate

### 常用命令速查

| 命令 | 说明 |
|------|------|
| `cargo build` | 构建项目 |
| `cargo build --release` | 发布构建 |
| `cargo run` | 运行项目 |
| `cargo test` | 运行测试 |
| `cargo check` | 快速检查 |
| `cargo fmt` | 格式化代码 |
| `cargo clippy` | 代码 lint |
| `cargo doc --open` | 生成文档 |
| `cargo add` | 添加依赖 |
| `cargo update` | 更新依赖 |
| `cargo tree` | 查看依赖树 |

---

## 下一章

[第 22 章：智能指针](22-智能指针.md)
