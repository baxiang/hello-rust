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







