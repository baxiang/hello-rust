# 第 14 章：错误处理

> 掌握 Rust 的错误处理机制和最佳实践。

## 学习目标

完成本章学习后，你将掌握：
- 理解 panic 和 Result 两种错误处理方式
- 掌握 Result 类型的操作方法
- 学会使用 ? 运算符传播错误
- 定义和处理自定义错误类型
- 理解错误处理的最佳实践

## 章节内容

### 错误处理概述
- [错误处理概述](./01-错误处理概述.md) - Rust 错误处理哲学、不可恢复错误

### panic
- [panic](./02-panic.md) - panic 触发、栈展开、panic 处理

### Result
- [Result](./03-Result.md) - Result 类型、错误传播、? 运算符

### 自定义错误
- [自定义错误](./04-自定义错误.md) - Error trait、thiserror、anyhow

### 实战总结
- [实战总结](./05-实战总结.md) - 错误处理最佳实践

## 预计学习时间

- 基础学习：2 小时
- 练习巩固：1.5 小时
- 总计：3.5 小时

## 实战项目

本章将实现一个健壮的文件处理工具。

## 练习题答案

参见各小节内的练习部分。

## 常见问题

- 什么时候用 panic，什么时候用 Result？
- 如何统一处理多种错误类型？
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-error-overview 02-panic 03-result 04-custom-errors 05-error-review
cargo check --examples
```
