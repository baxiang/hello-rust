## 项目概述

### 功能需求

- 添加待办事项
- 列出所有待办事项
- 标记完成
- 删除待办事项
- 数据持久化存储

### 技术栈

- **Clap**：命令行参数解析
- **Serde**：数据序列化
- **Anyhow**：错误处理






## 项目结构

```
todo-cli/
├── Cargo.toml
├── src/
│   ├── main.rs        # 入口和 CLI 定义
│   ├── lib.rs         # 库入口
│   ├── todo.rs        # Todo 数据结构
│   ├── storage.rs     # 存储模块
│   └── commands.rs    # 命令处理
└── data/
    └── todos.json     # 数据文件
```







