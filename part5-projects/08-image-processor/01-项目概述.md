## 项目概述

### 功能需求

- 图片格式转换
- 调整大小
- 压缩优化
- 批量处理
- 并行加速

### 技术栈

- **Image**：图片处理库
- **Tokio**：异步运行时
- **Rayon**：并行处理
- **Clap**：命令行参数






## 项目结构

```
image-processor/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库入口
│   ├── processor/
│   │   ├── mod.rs
│   │   ├── resize.rs     # 调整大小
│   │   ├── convert.rs    # 格式转换
│   │   ├── compress.rs   # 压缩
│   │   └── filter.rs     # 滤镜
│   ├── batch/
│   │   ├── mod.rs
│   │   └── worker.rs     # 批量处理
│   └── utils/
│       ├── mod.rs
│       └── format.rs     # 格式工具
└── tests/
    └── processor_test.rs
```







