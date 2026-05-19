## 项目概述

### 功能需求

- 用户管理 CRUD
- JWT 认证授权
- 数据持久化
- 错误处理
- API 文档

### 技术栈

- **Axum**：Web 框架
- **Tokio**：异步运行时
- **SQLx**：数据库操作
- **JWT**：认证
- **Serde**：序列化






## 项目结构

```
rest-api/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库入口
│   ├── config.rs         # 配置
│   ├── db.rs             # 数据库
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs       # 用户模型
│   │   └── auth.rs       # 认证模型
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── user.rs       # 用户处理
│   │   └── auth.rs       # 认证处理
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs       # 认证中间件
│   └── error.rs          # 错误处理
├── migrations/
│   └── 001_users.sql
└── .env
```







