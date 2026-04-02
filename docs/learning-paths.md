# Rust 学习路径指南

> 为不同背景和目标的学习者提供清晰的学习路线规划

## 目录

- [初学者路径（0-2 周）](#初学者路径0-2-周)
- [进阶路径（3-6 周）](#进阶路径3-6-周)
- [实战路径（7-12 周）](#实战路径7-12-周)
- [现代实践路径（13-15 周）](#现代实践路径13-15-周)
- [快速路径](#快速路径有编程经验)
- [特定目标路径](#特定目标路径)

---

## 初学者路径（0-2 周）

**适合人群**：零编程基础或编程初学者  
**学习目标**：掌握 Rust 基础语法，理解核心概念

### 第一周：基础入门

#### Day 1-2：环境搭建与第一个程序
- **[第 01 章](../part1-basics/01-intro/README.md)**：简介与环境搭建（1.5 小时）
  - 安装 Rust 工具链
  - 配置开发环境（VSCode + rust-analyzer）
  - 运行第一个程序
- **[第 02 章](../part1-basics/02-first-program/README.md)**：第一个 Rust 程序（2 小时）
  - 理解程序结构
  - 学习 Cargo 基础命令
  - 练习：创建并运行多个小程序

#### Day 3-4：变量与数据类型
- **[第 03 章](../part1-basics/03-variables/README.md)**：变量与可变性（2 小时）
  - 理解不可变性的重要性
  - 掌握变量遮蔽（shadowing）
  - 区分变量、常量、静态变量
- **[第 04 章](../part1-basics/04-types/README.md)**：数据类型（3 小时）
  - 标量类型：整数、浮点、布尔、字符
  - 复合类型：元组、数组
  - 类型推断与显式标注

#### Day 5-6：函数与控制流
- **[第 05 章](../part1-basics/05-functions/README.md)**：函数（2 小时）
  - 函数定义与调用
  - 参数与返回值
  - 表达式与语句
- **[第 06 章](../part1-basics/06-control-flow/README.md)**：控制流（2.5 小时）
  - if 表达式
  - 循环：loop、while、for
  - 模式匹配初探

#### Day 7：复习与练习
- 复习第 01-06 章内容
- 完成所有章节练习题
- 在 Rust Playground 练习基础语法
- 阅读各章节"常见问题"部分

### 第二周：核心概念

#### Day 8-9：所有权系统（重点）
- **[第 07 章](../part2-core/01-ownership/README.md)**：所有权与借用（3.5 小时）
  - **这是 Rust 最核心的概念**
  - 所有权三大规则
  - 移动语义 vs 克隆
  - 练习：手动追踪所有权转移
- **[第 08 章](../part2-core/02-references/README.md)**：引用与借用（3 小时）
  - 不可变引用与可变引用
  - 借用规则（同时只能有一个可变引用）
  - 悬垂引用防护

#### Day 10：切片类型
- **[第 09 章](../part2-core/03-slices/README.md)**：切片（2 小时）
  - 字符串切片 `&str`
  - 数组切片 `&[T]`
  - 切片与所有权的关系

#### Day 11-12：自定义类型
- **[第 10 章](../part2-core/04-structs/README.md)**：结构体（2.5 小时）
  - 结构体定义与实例化
  - 方法定义
  - 关联函数与构造器
- **[第 11 章](../part2-core/05-enums/README.md)**：枚举与模式匹配（3 小时）
  - 枚举定义与变体
  - Option 类型
  - match 表达式
  - if let 简化模式

#### Day 13-14：复习与巩固
- 复习第 07-11 章内容
- 重点练习所有权相关题目
- 编写小程序巩固：
  - 计算器（结构体 + 方法）
  - 简单的文本处理工具

---

## 进阶路径（3-6 周）

**适合人群**：已完成初学者路径或有其他语言经验  
**学习目标**：掌握 Rust 高级特性，理解类型系统

### 第三周：数据结构与集合

#### Day 15-16：集合类型
- **[第 12 章](../part3-data/01-collections/README.md)**：数组与 Vec（3 小时）
  - 数组 vs 动态数组
  - Vec 的创建、增删改查
  - 容量与性能
- **[第 13 章](../part3-data/02-hashmap/README.md)**：HashMap（2.5 小时）
  - 哈希表原理
  - 增删改查操作
  - 自定义键类型

#### Day 17：错误处理
- **[第 14 章](../part3-data/03-error-handling/README.md)**：错误处理（3 小时）
  - panic! 与不可恢复错误
  - Result<T, E> 类型
  - ? 运算符
  - 自定义错误类型

#### Day 18-19：复习与练习
- 编写数据处理小程序
- 练习错误处理的最佳实践
- 实现简单的配置文件解析器

### 第四周：泛型与 Trait

#### Day 20-22：泛型与 Trait（重点）
- **[第 15 章](../part3-data/04-generics/README.md)**：泛型（3 小时）
  - 泛型函数、结构体、枚举
  - Trait bound
  - 泛型性能（单态化）
- **[第 16 章](../part3-data/05-traits/README.md)**：Trait（5 小时）
  - **这是 Rust 的核心抽象机制**
  - Trait 定义与实现
  - 默认实现
  - Trait 对象与动态分发
  - 标准库常用 Trait

#### Day 23-24：生命周期（难点）
- **[第 17 章](../part3-data/06-lifetimes/README.md)**：生命周期（4 小时）
  - **这是 Rust 最难的概念之一**
  - 生命周期标注语法
  - 省略规则
  - 'static 生命周期
  - 练习：理解编译器错误提示

#### Day 25-28：闭包与迭代器
- **[第 18 章](../part3-data/07-closures/README.md)**：闭包（3 小时）
  - 闭包语法
  - 捕获环境变量
  - Fn、FnMut、FnOnce Trait
- **[第 19 章](../part3-data/08-iterators/README.md)**：迭代器（3 小时）
  - Iterator trait
  - 迭代器适配器（map、filter、collect 等）
  - 消费器
  - 零成本抽象

### 第五-六周：项目实践

选择一个小型项目应用所学知识：

#### 项目选择
- **推荐**：待办事项 CLI（项目一）
  - 综合运用结构体、枚举、错误处理
  - 学习命令行参数解析
- **备选**：文件搜索工具（项目二）
  - 练习文件 I/O 和字符串处理
  - 使用迭代器和闭包

#### 学习方式
1. 阅读项目 README 了解需求
2. 尝试自己实现核心功能
3. 遇到问题回顾相关章节
4. 完成后对比参考实现

---

## 实战路径（7-12 周）

**适合人群**：已掌握进阶知识  
**学习目标**：掌握高级主题，完成实战项目

### 第七周：模块系统与 Cargo

#### Day 29-31：模块系统
- **[第 20 章](../part4-advanced/01-modules/README.md)**：包和模块（3 小时）
  - Crate、Package、Module 概念
  - 模块树与路径
  - 可见性控制（pub）
  - use 关键字

#### Day 32-34：Cargo 进阶
- **[第 21 章](../part4-advanced/02-cargo/README.md)**：Cargo 与 Crates（4 小时）
  - Cargo.toml 配置详解
  - 依赖管理
  - 工作空间
  - 发布到 crates.io

### 第八周：智能指针与并发

#### Day 35-37：智能指针
- **[第 22 章](../part4-advanced/03-smart-pointers/README.md)**：智能指针（4 小时）
  - Box<T> 堆分配
  - Rc<T> 引用计数
  - Arc<T> 原子引用计数
  - RefCell<T> 内部可变性

#### Day 38-42：并发编程（重点）
- **[第 23 章](../part4-advanced/04-concurrency/README.md)**：并发编程（6 小时）
  - 线程创建与管理
  - 消息传递（channel）
  - 共享状态并发
  - Sync 和 Send Trait
  - 异步编程基础（async/await）

### 第九周：高级特性

#### Day 43-45：Unsafe Rust
- **[第 24 章](../part4-advanced/05-unsafe/README.md)**：Unsafe Rust（3 小时）
  - unsafe 块的使用场景
  - 裸指针
  - FFI（外部函数接口）
  - 何时使用 unsafe

#### Day 46-49：宏
- **[第 25 章](../part4-advanced/06-macros/README.md)**：宏（4 小时）
  - macro_rules! 声明宏
  - 过程宏（派生宏、属性宏、函数宏）
  - 常用内置宏

### 第十-十二周：应用开发与项目实战

#### Week 10：应用开发
- **[第 26 章](../part4-advanced/07-cli/README.md)**：命令行工具（4 小时）
  - Clap 库使用
  - 彩色输出
  - 进度条
- **[第 27 章](../part4-advanced/08-web/README.md)**：Web 服务器（6 小时）
  - Axum 框架入门
  - 路由与处理器
  - 中间件
  - RESTful API 设计

#### Week 11：测试与文档
- **[第 28 章](../part4-advanced/09-testing/README.md)**：测试与文档（4 小时）
  - 单元测试与集成测试
  - 文档测试
  - 基准测试
  - CI/CD 集成

#### Week 12：综合项目
选择 2-3 个项目实战：

**Web 开发方向**：
- [项目三：REST API 服务](../part5-projects/03-rest-api/README.md)
- [项目五：聊天室应用](../part5-projects/05-chat-room/README.md)

**系统工具方向**：
- [项目四：日志分析工具](../part5-projects/04-log-analyzer/README.md)
- [项目六：简易键值存储](../part5-projects/06-kv-store/README.md)

**数据处理方向**：
- [项目七：网络爬虫](../part5-projects/07-web-crawler/README.md)
- [项目八：图片处理工具](../part5-projects/08-image-processor/README.md)

**编程语言方向**：
- [项目九：简易解释器](../part5-projects/09-interpreter/README.md)

---

## 快速路径（有编程经验）

**适合人群**：有 C/C++、Java、Python 等语言经验  
**学习时间**：2-3 周快速上手

### 核心概念重点（Week 1）

**必读章节**（理解 Rust 独特性）：
- **[第 07-09 章](../part2-core/01-ownership/README.md)**：所有权系统（3 天）
  - 重点关注移动语义
  - 理解借用规则
- **[第 17 章](../part3-data/06-lifetimes/README.md)**：生命周期（1 天）
  - 这是其他语言没有的概念
- **[第 16 章](../part3-data/05-traits/README.md)**：Trait（1 天）
  - 对比其他语言的接口/抽象类

### 类型系统与抽象（Week 2）

- **[第 15 章](../part3-data/04-generics/README.md)**：泛型（半天）
- **[第 18-19 章](../part3-data/07-closures/README.md)**：闭包与迭代器（1 天）
- **[第 22 章](../part4-advanced/03-smart-pointers/README.md)**：智能指针（1 天）
- **[第 23 章](../part4-advanced/04-concurrency/README.md)**：并发编程（2 天）

### 实践应用（Week 3）

- **[第 20-21 章](../part4-advanced/01-modules/README.md)**：模块与 Cargo（半天）
- **[第 26-27 章](../part4-advanced/07-cli/README.md)**：CLI 或 Web 开发（2 天）
- **选择 1-2 个项目实战**（剩余时间）

### 跳过章节
- 可快速浏览或跳过：
  - 第 01-06 章（基础语法，快速浏览）
  - 第 24 章（Unsafe，初期不常用）
  - 第 25 章（宏，初期不常用）

---

## 特定目标路径

### Web 开发路径

**学习顺序**：
1. **基础**：第 01-11 章（必学）
2. **数据处理**：第 12-19 章（必学）
3. **Web 开发**：[第 27 章](../part4-advanced/08-web/README.md)（重点）
4. **项目实战**：[项目三](../part5-projects/03-rest-api/README.md)（REST API）、[项目五](../part5-projects/05-chat-room/README.md)（聊天室）

**推荐框架**：
- Axum（推荐，现代异步框架）
- Actix-web（功能完整）

**关键知识点**：
- 异步编程（async/await）
- 序列化（serde）
- 错误处理
- 状态管理

### 系统编程路径

**学习顺序**：
1. **基础**：第 01-11 章
2. **底层特性**：[第 22 章](../part4-advanced/03-smart-pointers/README.md)（智能指针）、[第 24 章](../part4-advanced/05-unsafe/README.md)（Unsafe）
3. **并发**：[第 23 章](../part4-advanced/04-concurrency/README.md)（并发编程）
4. **项目实战**：[项目二](../part5-projects/02-file-search/README.md)（文件搜索）、[项目六](../part5-projects/06-kv-store/README.md)（键值存储）

**关键知识点**：
- 内存布局与对齐
- FFI 与 C 交互
- 无锁数据结构
- 性能优化

### 嵌入式开发路径

**学习顺序**：
1. **核心**：第 01-11 章
2. **底层**：第 24 章（Unsafe）
3. **内存管理**：第 22 章（智能指针）
4. **并发**：第 23 章（部分内容）

**扩展学习**：
- `embedded-hal` 生态系统
- `no_std` 开发
- 硬件抽象层

**项目建议**：
- 先完成基础项目熟悉语法
- 然后使用嵌入式开发板实践

### CLI 工具开发路径

**学习顺序**：
1. **基础**：第 01-19 章
2. **CLI 专用**：[第 26 章](../part4-advanced/07-cli/README.md)（命令行工具）
3. **项目实战**：[项目一](../part5-projects/01-todo-cli/README.md)（待办事项）、[项目二](../part5-projects/02-file-search/README.md)（文件搜索）、[项目四](../part5-projects/04-log-analyzer/README.md)（日志分析）

**推荐库**：
- `clap`：参数解析
- `indicatif`：进度条
- `colored`：彩色输出
- `serde`：配置文件

### 数据处理路径

**学习顺序**：
1. **基础**：第 01-19 章
2. **集合与迭代器**：[第 12 章](../part3-data/01-collections/README.md)、[第 19 章](../part3-data/08-iterators/README.md)（重点）
3. **并发处理**：[第 23 章](../part4-advanced/04-concurrency/README.md)
4. **项目实战**：[项目四](../part5-projects/04-log-analyzer/README.md)（日志分析）、[项目七](../part5-projects/07-web-crawler/README.md)（网络爬虫）、[项目八](../part5-projects/08-image-processor/README.md)（图片处理）

**关键技能**：
- 高效使用迭代器
- 并发数据处理
- 文件 I/O 操作
- 性能优化

---

## 学习建议

### 时间分配建议

| 阶段 | 时间 | 每日学习 | 建议 |
|------|------|----------|------|
| 初学者路径 | 2 周 | 2-3 小时 | 注重理解，多练习 |
| 进阶路径 | 4 周 | 1-2 小时 | 完成项目巩固 |
| 实战路径 | 6 周 | 1-2 小时 | 选择感兴趣的方向 |

### 学习方法

1. **理论 + 实践**
   - 每章先阅读理解概念
   - 在 Rust Playground 练习
   - 完成本地项目实践

2. **遇到困难时**
   - 查看章节"常见问题"部分
   - 查阅 [FAQ.md](./FAQ.md)
   - 使用 `cargo explain` 查看错误
   - 在 Rust 社区提问

3. **巩固知识**
   - 教别人是最好的学习方式
   - 参与 Rust 社区讨论
   - 阅读 Rust 标准库源码

### 学习资源

**官方资源**：
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [标准库文档](https://doc.rust-lang.org/std/)

**社区资源**：
- [Rust 中文社区](https://rustcc.cn/)
- [Rust Forum](https://users.rust-lang.org/)
- [/r/rust](https://www.reddit.com/r/rust/)

**实践平台**：
- [Rust Playground](https://play.rust-lang.org/)
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Advent of Code](https://adventofcode.com/)

---

## 学习检查清单

### 基础阶段检查点
- [ ] 能够解释所有权、借用、生命周期
- [ ] 熟练使用 match 和 if let
- [ ] 理解 String 和 &str 的区别
- [ ] 能够定义结构体和枚举
- [ ] 掌握方法定义语法

### 进阶阶段检查点
- [ ] 能够使用 Result 进行错误处理
- [ ] 理解泛型和 Trait bound
- [ ] 能够标注生命周期参数
- [ ] 熟练使用迭代器适配器
- [ ] 理解闭包的捕获规则

### 实战阶段检查点
- [ ] 能够组织多模块项目
- [ ] 熟练使用 Cargo 管理依赖
- [ ] 理解智能指针的使用场景
- [ ] 能够编写并发程序
- [ ] 完成至少一个实战项目

---

## 现代实践路径（13-15 周）

**适合人群**：已完成实战路径，想要掌握 Rust 最新技术  
**学习目标**：掌握 Rust 2024 Edition、异步编程、WebAssembly

### 第十三周：Rust 2024 Edition

#### Day 85-86：Edition 机制与新特性
- **[第 29 章](../part6-modern/01-rust-2024/README.md)**：Rust 2024 Edition（4 小时）
  - 理解 Edition 机制和演进策略
  - 学习异步闭包、impl Trait 增强、模式匹配改进
  - 了解迁移策略和最佳实践

#### Day 87-89：现代 Crate 与工具链
- **[第 29 章](../part6-modern/01-rust-2024/README.md)**：现代 Crate 推荐（3 小时）
  - 错误处理：thiserror + anyhow
  - 异步运行时：Tokio
  - Web 框架：Axum
  - CLI：clap
- **[第 29 章](../part6-modern/01-rust-2024/README.md)**：工具链使用（2 小时）
  - rustup、cargo、rust-analyzer
  - CI/CD 配置
  - 开发工作流优化

#### Day 90-91：迁移实践
- 将一个 Rust 2021 项目迁移到 2024 Edition
- 使用新特性改进代码
- 完成迁移验证和测试

### 第十四周：异步编程

#### Day 92-94：异步基础与 Future
- **[第 30 章](../part6-modern/02-async/README.md)**：异步基础（3 小时）
  - 理解异步编程的必要性
  - 同步 vs 异步 vs 多线程对比
  - 学习第一个异步程序
- **[第 30 章](../part6-modern/02-async/README.md)**：Future 与 async/await（3 小时）
  - Future trait 机制
  - async/await 语法
  - Pin 与 Unpin 概念

#### Day 95-97：Tokio 运行时
- **[第 30 章](../part6-modern/02-async/README.md)**：Tokio 运行时（4 小时）
  - 运行时配置（多线程 vs 单线程）
  - 任务调度和 I/O 驱动
  - 异步网络和文件 I/O
  - 定时器和同步原语

#### Day 98-100：异步模式
- **[第 30 章](../part6-modern/02-async/README.md)**：异步模式（3 小时）
  - Stream 和异步迭代器
  - 异步通道（mpsc、oneshot、broadcast）
  - 异步锁（Mutex、RwLock、Semaphore）
  - select! 宏和任务取消

#### Day 101：异步实战
- 实现异步 HTTP 客户端
- 编写并发文件处理器
- 使用异步数据库操作

### 第十五周：WebAssembly

#### Day 102-104：WASM 基础
- **[第 31 章](../part6-modern/03-webassembly/README.md)**：WASM 基础（3 小时）
  - 理解 WebAssembly 概念和优势
  - Rust 编译到 WASM
  - wasm-pack 工具使用
  - 创建第一个 WASM 应用

#### Day 105-107：wasm-bindgen
- **[第 31 章](../part6-modern/03-webassembly/README.md)**：wasm-bindgen（4 小时）
  - JavaScript 互操作
  - 类型转换和结构体导出
  - DOM 操作
  - 异步操作

#### Day 108-110：实战项目
- **[第 31 章](../part6-modern/03-webassembly/README.md)**：实战项目（5 小时）
  - 实现图像处理 Web 应用
  - WASM vs JavaScript 性能对比
  - 性能优化技巧

### 现代实践阶段检查点
- [ ] 理解 Rust 2024 Edition 机制
- [ ] 掌握现代 crate 生态
- [ ] 熟练使用 Tokio 运行时
- [ ] 能够编写异步程序
- [ ] 理解 WebAssembly 基础
- [ ] 完成 WASM 实战项目

---

## 下一步

完成学习路径后：
1. 阅读 [前置知识图谱](./prerequisites.md) 了解知识依赖
2. 查看 [常见问题](./FAQ.md) 解决疑惑
3. 选择一个 [项目实战](../part5-projects/) 开始实践
4. 学习 [现代实践](../part6-modern/) 掌握最新技术
5. 参与 Rust 社区，持续学习