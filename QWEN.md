# QWEN.md - Rust 教程项目上下文

## 项目概述

这是一个 **Rust 编程语言中文教程**，共 28 章 + 9 个实战项目，系统性地从零开始讲解 Rust 编程语言。教程采用 Markdown 格式编写，包含丰富的代码示例、可视化图表、常见错误解析和练习题。

### 项目定位

- **类型**：技术文档/教程项目（非代码项目）
- **语言**：中文
- **主题**：Rust 编程语言学习
- **目标读者**：Rust 初学者到进阶开发者


## 目录结构

```
hello-rust/
├── README.md              # 教程总目录和导航
├── QWEN.md                # 项目上下文文件
├── .gitignore             # Git 忽略配置
│
├── part1-basics/          # 第一部分：基础入门
│   ├── 01-intro/          # 简介与环境搭建
│   │   └── README.md
│   ├── 02-first-program/  # 第一个 Rust 程序
│   │   └── README.md
│   ├── 03-variables/      # 变量与可变性
│   │   └── README.md
│   ├── 04-types/          # 数据类型
│   │   └── README.md
│   ├── 05-functions/      # 函数
│   │   └── README.md
│   └── 06-control-flow/   # 控制流
│       └── README.md
│
├── part2-core/            # 第二部分：核心概念
│   ├── 01-ownership/      # 所有权与借用
│   │   └── README.md
│   ├── 02-references/     # 引用与借用
│   │   └── README.md
│   ├── 03-slices/         # 切片
│   │   └── README.md
│   ├── 04-structs/        # 结构体
│   │   └── README.md
│   └── 05-enums/          # 枚举与模式匹配
│       └── README.md
│
├── part3-data/            # 第三部分：数据结构与特性
│   ├── 01-collections/    # 数组与 Vec
│   │   └── README.md
│   ├── 02-hashmap/        # HashMap
│   │   └── README.md
│   ├── 03-error-handling/ # 错误处理
│   │   └── README.md
│   ├── 04-generics/       # 泛型
│   │   └── README.md
│   ├── 05-traits/         # Trait
│   │   └── README.md
│   ├── 06-lifetimes/      # 生命周期
│   │   └── README.md
│   ├── 07-closures/       # 闭包
│   │   └── README.md
│   └── 08-iterators/      # 迭代器
│       └── README.md
│
├── part4-advanced/        # 第四部分：高级主题
│   ├── 01-modules/        # 包和模块
│   │   └── README.md
│   ├── 02-cargo/          # Cargo 与 Crates
│   │   └── README.md
│   ├── 03-smart-pointers/ # 智能指针
│   │   └── README.md
│   ├── 04-concurrency/    # 并发编程
│   │   └── README.md
│   ├── 05-unsafe/         # Unsafe Rust
│   │   └── README.md
│   ├── 06-macros/         # 宏
│   │   └── README.md
│   ├── 07-cli/            # 命令行工具
│   │   └── README.md
│   ├── 08-web/            # Web 服务器
│   │   └── README.md
│   └── 09-testing/        # 测试与文档
│       └── README.md
│
├── part5-projects/        # 第五部分：项目实战
│   ├── 01-todo-cli/       # 待办事项 CLI
│   │   └── README.md
│   ├── 02-file-search/    # 文件搜索工具
│   │   └── README.md
│   ├── 03-rest-api/       # REST API 服务
│   │   └── README.md
│   ├── 04-log-analyzer/   # 日志分析工具
│   │   └── README.md
│   ├── 05-chat-room/      # 聊天室应用
│   │   └── README.md
│   ├── 06-kv-store/       # 简易键值存储
│   │   └── README.md
│   ├── 07-web-crawler/    # 网络爬虫
│   │   └── README.md
│   ├── 08-image-processor/# 图片处理工具
│   │   └── README.md
│   └── 09-interpreter/    # 简易解释器
│       └── README.md
│
└── .claude/               # Claude 配置目录
```


## 教程章节组织

### 第一部分：基础入门（part1-basics/）

适合 Rust 初学者，涵盖基础语法和环境配置：

| 子目录 | 内容 |
|--------|------|
| `01-intro/` | Rust 特点、安装、工具链 |
| `02-first-program/` | 程序结构、Cargo 入门 |
| `03-variables/` | 变量声明、可变性、常量 |
| `04-types/` | 标量类型、复合类型 |
| `05-functions/` | 函数定义、参数、返回值 |
| `06-control-flow/` | 条件表达式、循环 |

### 第二部分：核心概念（part2-core/）

Rust 最独特的特性，内存安全的核心保障：

| 子目录 | 内容 |
|--------|------|
| `01-ownership/` | 所有权三定律、移动语义 |
| `02-references/` | 引用类型、借用规则 |
| `03-slices/` | 字符串切片、数组切片 |
| `04-structs/` | 结构体定义、方法 |
| `05-enums/` | Option、match、if let |

### 第三部分：数据结构与特性（part3-data/）

Rust 类型系统和泛型编程：

| 子目录 | 内容 |
|--------|------|
| `01-collections/` | 数组、动态数组 |
| `02-hashmap/` | 哈希表操作 |
| `03-error-handling/` | Result、Option、panic |
| `04-generics/` | 泛型函数、泛型结构体 |
| `05-traits/` | Trait 定义与实现 |
| `06-lifetimes/` | 生命周期标注 |
| `07-closures/` | 闭包语法、Fn Trait |
| `08-iterators/` | Iterator、适配器 |

### 第四部分：高级主题（part4-advanced/）

实战开发与高级特性：

| 子目录 | 内容 |
|--------|------|
| `01-modules/` | Crate、模块系统 |
| `02-cargo/` | 包管理、依赖配置 |
| `03-smart-pointers/` | Box、Rc、Arc、RefCell |
| `04-concurrency/` | 线程、消息传递 |
| `05-unsafe/` | 裸指针、FFI |
| `06-macros/` | macro_rules!、过程宏 |
| `07-cli/` | Clap、CLI 开发 |
| `08-web/` | Axum、Actix-web |
| `09-testing/` | 单元测试、文档测试 |

### 第五部分：项目实战（part5-projects/）

完整的项目开发实践：

| 子目录 | 内容 | 技术栈 |
|--------|------|--------|
| `01-todo-cli/` | 命令行待办事项管理工具 | Clap, Serde |
| `02-file-search/` | 类似 grep 的文件搜索工具 | Regex, Walkdir |
| `03-rest-api/` | 使用 Axum 构建 Web API | Axum, SQLx, JWT |
| `04-log-analyzer/` | 解析和分析日志文件 | Regex, Chrono |
| `05-chat-room/` | WebSocket 实时聊天应用 | Axum WS, DashMap |
| `06-kv-store/` | 类似 Redis 的内存数据库 | Tokio, DashMap |
| `07-web-crawler/` | 异步爬取网页内容 | Reqwest, Scraper |
| `08-image-processor/` | 批量处理图片 | Image, Rayon |
| `09-interpreter/` | 小型编程语言解释器 | 纯 Rust |


## 文档风格与格式约定

### Markdown 格式特点

1. **标题层级**
   - `#` 章节标题
   - `##` 主要节
   - `###` 子节
   - `>` 引用块用于章节简介

2. **代码块**
   - 使用 `rust` 语言标记
   - 包含详细注释说明
   - 标注 `✅ 正确` 和 `❌ 错误` 示例

3. **可视化图表**
   - 使用 ASCII 艺术绘制内存布局图
   - 使用表格对比概念
   - 使用 `┌─┐` 风格的框图

4. **示例格式**
```rust
// ✅ 正确示例
fn good_example() { ... }

// ❌ 错误示例（附带错误信息）
fn bad_example() { ... }
```

5. **章节结构**
   - 概念介绍
   - 代码示例
   - 内存布局图解
   - 常见错误
   - 练习题
   - 小结


## 关键内容特点

### 所有权系统讲解（part2-core/01-ownership/）

教程对 Rust 核心的所有权系统有详细讲解：
- 三大定律的清晰表述
- 移动语义的内存布局图
- Copy vs Clone 对比
- 函数与所有权传递

### 错误处理风格

每个章节包含：
- 常见编译错误示例
- 错误信息解读
- 多种解决方案对比

### 实战章节（part4-advanced/07-cli/、08-web/、09-testing/）

包含完整的实战项目示例：
- 命令行工具开发（使用 Clap）
- Web 服务器（Axum/Actix-web）
- 测试与文档编写

### 项目实战部分（part5-projects/）

每个项目包含：
- 项目需求分析
- 架构设计
- 分步实现
- 完整代码
- 运行效果


## 编辑与维护指南

### 添加新内容时遵循的原则

1. **保持风格一致**
   - 使用相同的标题层级
   - 代码块附带注释
   - 添加可视化图表

2. **代码示例规范**
   - 提供完整可运行的代码
   - 标注正确/错误示例
   - 解释编译错误信息

3. **中文术语**
   - 使用标准中文技术术语
   - 保留英文原文术语对照
   - 如：所有权 (Ownership)、借用 (Borrowing)

4. **章节末尾**
   - 添加小结 checklist
   - 提供练习题
   - 链接到下一章

### 添加新章节

1. 确定章节所属部分目录
2. 在对应目录创建子目录（如 `09-new-topic/`）
3. 在子目录中创建 `README.md`
4. 更新 README.md 的导航链接
5. 更新此文件的章节列表

### 添加新项目

1. 在 `part5-projects/` 目录创建子目录
2. 在子目录中创建 `README.md`
3. 包含完整的项目实现步骤
4. 更新 README.md 的项目列表
5. 更新此文件的项目列表


## 参考资源（教程中引用）

### 官方资源
- Rust 官网: https://www.rust-lang.org/
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- 标准库文档: https://doc.rust-lang.org/std/

### 社区资源
- Rust 中文社区: https://rustcc.cn/
- Rust Forum: https://users.rust-lang.org/


## 版本信息

- 最低 Rust 版本：1.85+
- 教程版本：2.2
- Markdown 格式：GitHub Flavored Markdown