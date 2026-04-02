## 项目概述

### 功能需求

- WebSocket 实时通信
- 多房间支持
- 用户认证
- 消息历史记录
- 在线用户列表

### 技术栈

- **Axum**：Web 框架 + WebSocket
- **Tokio**：异步运行时
- **Serde**：消息序列化
- **DashMap**：并发 HashMap






## 项目结构

```
chat-room/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库入口
│   ├── models/
│   │   ├── mod.rs
│   │   ├── message.rs    # 消息模型
│   │   ├── room.rs       # 房间模型
│   │   └── user.rs       # 用户模型
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── ws.rs         # WebSocket 处理
│   │   └── room.rs       # 房间管理
│   ├── state.rs          # 应用状态
│   └── broadcast.rs      # 广播模块
└── static/
    └── index.html        # 前端页面
```







