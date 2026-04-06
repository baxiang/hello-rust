## 项目概述

### 功能需求

- 基本操作：SET、GET、DEL
- 数据类型：String、List、Hash
- 过期时间支持
- 持久化存储
- 简单命令协议

### 技术栈

- **Tokio**：异步运行时
- **DashMap**：并发 HashMap
- **Serde**：序列化
- **Chrono**：时间处理






## 项目结构

```
kv-store/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库入口
│   ├── store/
│   │   ├── mod.rs
│   │   ├── engine.rs     # 存储引擎
│   │   ├── types.rs      # 数据类型
│   │   └── expire.rs     # 过期管理
│   ├── command/
│   │   ├── mod.rs
│   │   ├── parser.rs     # 命令解析
│   │   ├── executor.rs   # 命令执行
│   │   └── protocol.rs   # 协议定义
│   ├── persistence/
│   │   ├── mod.rs
│   │   ├── file.rs       # 文件持久化
│   │   └── snapshot.rs   # 快照
│   └── server.rs         # TCP 服务器
└── tests/
    └── store_test.rs
```







