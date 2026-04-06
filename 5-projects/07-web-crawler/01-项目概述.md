## 项目概述

### 功能需求

- 异步 HTTP 请求
- HTML 解析
- 链接提取与跟踪
- 并发控制
- 数据存储

### 技术栈

- **Tokio**：异步运行时
- **Reqwest**：HTTP 客户端
- **Scraper**：HTML 解析
- **Selector**：CSS 选择器






## 项目结构

```
web-crawler/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库入口
│   ├── crawler/
│   │   ├── mod.rs
│   │   ├── fetcher.rs    # 网页获取
│   │   ├── parser.rs     # HTML 解析
│   │   ├── scheduler.rs  # 任务调度
│   │   └── queue.rs      # URL 队列
│   ├── extractor/
│   │   ├── mod.rs
│   │   ├── links.rs      # 链接提取
│   │   ├── text.rs       # 文本提取
│   │   ├── images.rs     # 图片提取
│   └── storage/
│       ├── mod.rs
│       └── file.rs       # 文件存储
└── config.toml           # 配置文件
```







