# 第 30 章：异步编程

> 掌握 Rust 异步编程的核心概念和实践。

## 学习目标

完成本章学习后，你将掌握：
- 理解异步编程的基本概念和优势
- 掌握 Future trait 和 async/await 语法
- 熟练使用 Tokio 运行时构建异步应用
- 掌握异步模式和最佳实践
- 实现完整的异步应用程序

## 章节内容

### 异步基础
- [异步基础](./01-异步基础.md) - 为什么需要异步、同步vs异步vs多线程

### Future 与 async/await
- [Future与async](./02-Future与async.md) - Future trait、async函数、await关键字

### Tokio 运行时
- [Tokio运行时](./03-Tokio运行时.md) - 运行时概念、任务调度、I/O驱动

### 异步模式
- [异步模式](./04-异步模式.md) - 异步迭代器、通道、锁、取消和超时

### 实战案例
- [实战案例](./05-实战案例.md) - HTTP客户端、文件处理、数据库操作

## 预计学习时间

- 基础学习：3 小时
- 实践练习：4 小时
- 总计：7 小时

## 实战项目

本章将实现一个异步 Web 爬虫和并发文件处理器。

## 练习题答案

参见各小节内的练习部分。

## 常见问题

- 异步和多线程如何选择？
- 如何避免异步代码中的性能陷阱？
- Tokio 和 async-std 如何选择？

## 参考资源

- [The Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Futures API](https://docs.rs/futures/)
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-async-basics
cargo check --examples
```
