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

---

## 下一步

[构建配置（Profiles）](../7-构建配置（Profiles）.md)