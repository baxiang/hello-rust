# Rust 教程目录

> 从零开始系统学习 Rust 编程语言


## 教程结构

本教程共 31 章 + 9 个实战项目，分为六个部分：

### 第一部分：基础入门

| 章节 | 标题 | 说明 |
|------|------|------|
| [第 01 章](1-basics/01-intro/README.md) | 简介与环境搭建 | Rust 特点、安装、第一个程序 |
| [第 02 章](1-basics/02-first-program/README.md) | 第一个 Rust 程序 | 程序结构、编译运行、Cargo 入门 |
| [第 03 章](1-basics/03-variables/README.md) | 变量与可变性 | 变量声明、可变性、常量、静态变量 |
| [第 04 章](1-basics/04-types/README.md) | 数据类型 | 标量类型、复合类型、类型推断 |
| [第 05 章](1-basics/05-functions/README.md) | 函数 | 函数定义、参数、返回值、闭包简介 |
| [第 06 章](1-basics/06-control-flow/README.md) | 控制流 | 条件表达式、循环、模式匹配 |

### 第二部分：核心概念

| 章节 | 标题 | 说明 |
|------|------|------|
| [第 07 章](2-core/01-ownership/README.md) | 所有权与借用 | 所有权系统、移动语义、借用规则 |
| [第 08 章](2-core/02-references/README.md) | 引用与借用 | 引用类型、可变引用、悬垂引用 |
| [第 09 章](2-core/03-slices/README.md) | 切片 | 切片类型、字符串切片、数组切片 |
| [第 10 章](2-core/04-structs/README.md) | 结构体 | 结构体定义、方法、关联函数 |
| [第 11 章](2-core/05-enums/README.md) | 枚举与模式匹配 | 枚举定义、Option、match、if let |

### 第三部分：数据结构与特性

| 章节 | 标题 | 说明 |
|------|------|------|
| [第 12 章](3-data/01-collections/README.md) | 数组与 Vec | 数组、动态数组 Vec、常见操作 |
| [第 13 章](3-data/02-hashmap/README.md) | HashMap | 哈希表、增删改查、自定义键 |
| [第 14 章](3-data/03-error-handling/README.md) | 错误处理 | Result、Option、panic、错误传播 |
| [第 15 章](3-data/04-generics/README.md) | 泛型 | 泛型函数、泛型结构体、约束 |
| [第 16 章](3-data/05-traits/README.md) | Trait | Trait 定义、实现、派生 Trait |
| [第 17 章](3-data/06-lifetimes/README.md) | 生命周期 | 生命周期标注、省略规则、'static |
| [第 18 章](3-data/07-closures/README.md) | 闭包 | 闭包语法、捕获环境、Fn 系列 Trait |
| [第 19 章](3-data/08-iterators/README.md) | 迭代器 | Iterator trait、适配器、消费器 |

### 第四部分：高级主题

| 章节 | 标题 | 说明 |
|------|------|------|
| [第 20 章](4-advanced/01-modules/README.md) | 包和模块 | Crate、模块系统、可见性 |
| [第 21 章](4-advanced/02-cargo/README.md) | Cargo 与 Crates | 包管理、依赖配置、发布 |
| [第 22 章](4-advanced/03-smart-pointers/README.md) | 智能指针 | Box、Rc、Arc、RefCell |
| [第 23 章](4-advanced/04-concurrency/README.md) | 并发编程 | 线程、消息传递、同步原语 |
| [第 24 章](4-advanced/05-unsafe/README.md) | Unsafe Rust | 裸指针、FFI、不安全代码 |
| [第 25 章](4-advanced/06-macros/README.md) | 宏 | macro_rules!、过程宏、内置宏 |
| [第 26 章](4-advanced/07-cli/README.md) | 命令行工具 | Clap、彩色输出、交互 CLI |
| [第 27 章](4-advanced/08-web/README.md) | Web 服务器 | Axum、Actix-web、REST API |
| [第 28 章](4-advanced/09-testing/README.md) | 测试与文档 | 单元测试、集成测试、文档测试 |

### 第五部分：项目实战

| 项目 | 标题 | 说明 |
|------|------|------|
| [项目一](5-projects/01-todo-cli/README.md) | 待办事项 CLI | 命令行待办事项管理工具 |
| [项目二](5-projects/02-file-search/README.md) | 文件搜索工具 | 类似 grep 的文件搜索工具 |
| [项目三](5-projects/03-rest-api/README.md) | REST API 服务 | 使用 Axum 构建 Web API |
| [项目四](5-projects/04-log-analyzer/README.md) | 日志分析工具 | 解析和分析日志文件 |
| [项目五](5-projects/05-chat-room/README.md) | 聊天室应用 | WebSocket 实时聊天应用 |
| [项目六](5-projects/06-kv-store/README.md) | 简易键值存储 | 类似 Redis 的内存数据库 |
| [项目七](5-projects/07-web-crawler/README.md) | 网络爬虫 | 异步爬取网页内容 |
| [项目八](5-projects/08-image-processor/README.md) | 图片处理工具 | 批量处理图片 |
| [项目九](5-projects/09-interpreter/README.md) | 简易解释器 | 小型编程语言解释器 |

### 第六部分：现代实践

| 章节 | 标题 | 说明 |
|------|------|------|
| [第 29 章](6-modern/01-rust-2024/README.md) | Rust 2024 Edition | 新特性、现代 crate、工具链 |
| [第 30 章](6-modern/02-async/README.md) | 异步编程 | Future、async/await、Tokio 运行时 |
| [第 31 章](6-modern/03-webassembly/README.md) | WebAssembly | WASM 基础、wasm-bindgen、实战项目 |


## 快速导航

### 基础语法
- [变量与可变性](1-basics/03-variables/README.md)
- [数据类型](1-basics/04-types/README.md)
- [函数](1-basics/05-functions/README.md)
- [控制流](1-basics/06-control-flow/README.md)

### 所有权系统
- [所有权与借用](2-core/01-ownership/README.md)
- [引用与借用](2-core/02-references/README.md)
- [切片](2-core/03-slices/README.md)

### 自定义类型
- [结构体](2-core/04-structs/README.md)
- [枚举与模式匹配](2-core/05-enums/README.md)

### 集合类型
- [数组与 Vec](3-data/01-collections/README.md)
- [HashMap](3-data/02-hashmap/README.md)

### 高级特性
- [泛型](3-data/04-generics/README.md)
- [Trait](3-data/05-traits/README.md)
- [生命周期](3-data/06-lifetimes/README.md)
- [闭包](3-data/07-closures/README.md)
- [迭代器](3-data/08-iterators/README.md)

### 模块与包
- [包和模块](4-advanced/01-modules/README.md)
- [Cargo 与 Crates](4-advanced/02-cargo/README.md)

### 智能指针与并发
- [智能指针](4-advanced/03-smart-pointers/README.md)
- [并发编程](4-advanced/04-concurrency/README.md)

### 元编程与底层
- [宏](4-advanced/06-macros/README.md)
- [Unsafe Rust](4-advanced/05-unsafe/README.md)

### 实战应用
- [命令行工具](4-advanced/07-cli/README.md)
- [Web 服务器](4-advanced/08-web/README.md)
- [测试与文档](4-advanced/09-testing/README.md)

### 项目实战
- [待办事项 CLI](5-projects/01-todo-cli/README.md)
- [文件搜索工具](5-projects/02-file-search/README.md)
- [REST API 服务](5-projects/03-rest-api/README.md)
- [日志分析工具](5-projects/04-log-analyzer/README.md)
- [聊天室应用](5-projects/05-chat-room/README.md)
- [简易键值存储](5-projects/06-kv-store/README.md)
- [网络爬虫](5-projects/07-web-crawler/README.md)
- [图片处理工具](5-projects/08-image-processor/README.md)
- [简易解释器](5-projects/09-interpreter/README.md)

### 现代实践
- [Rust 2024 Edition](6-modern/01-rust-2024/README.md)
- [异步编程](6-modern/02-async/README.md)
- [WebAssembly](6-modern/03-webassembly/README.md)


## 学习建议

### 初学者路线
1. 按顺序阅读第 1-11 章，打好基础
2. 完成每章的练习题
3. 编写小项目巩固知识

### 进阶学习
1. 深入学习第 12-19 章
2. 理解 Rust 独特的类型系统
3. 阅读优秀开源项目代码

### 实战开发
1. 学习第 20-28 章
2. 选择合适的框架和库
3. 完成第五部分的项目实战

### 项目实战路线
1. 从 CLI 工具开始（项目一、二）
2. 学习 Web 开发（项目三、五）
3. 进阶数据处理（项目四、六、七、八）
4. 挑战系统编程（项目九）

### 现代实践路线
1. 学习 Rust 2024 Edition 新特性（第 29 章）
2. 掌握异步编程（第 30 章）
3. 探索 WebAssembly 应用（第 31 章）


## 参考资源

### 官方资源
- [Rust 官网](https://www.rust-lang.org/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [标准库文档](https://doc.rust-lang.org/std/)

### 社区资源
- [Rust 中文社区](https://rustcc.cn/)
- [Rust Forum](https://users.rust-lang.org/)
- [/r/rust](https://www.reddit.com/r/rust/)

### 实践平台
- [Rust Playground](https://play.rust-lang.org/)
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Advent of Code](https://adventofcode.com/)


## 版本信息

- 最低 Rust 版本：1.75+（现代实践部分需要 1.85+）
- 教程版本：2.3
- 最后更新：2025-01


*祝学习愉快！*