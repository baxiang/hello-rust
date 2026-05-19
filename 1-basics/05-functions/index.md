# 第 05 章：函数

> **本章代码基于 Rust 2024 Edition (Rust 1.85+) 编写**

> 掌握 Rust 函数定义、参数传递、返回值和高级特性。

## 本章目标

完成本章学习后，你将掌握：
- 定义和调用函数
- 理解参数传递与所有权的关系
- 掌握返回值与表达式语法
- 了解高阶函数与闭包基础
- 理解递归函数的使用

## 章节内容

### 函数基础
- [函数基础](./01-function-basics) - 函数定义、调用、命名规范

### 参数与返回值
- [参数与返回值](./02-params-and-returns) - 参数模式、返回值、表达式

### 所有权与函数
- [所有权与函数](./03-ownership-and-functions) - 值传递、引用传递、所有权转移

### 高级函数
- [高级函数](./04-advanced-functions) - 函数指针、高阶函数、方法链

### 闭包与递归
- [闭包与递归](./05-closures-and-recursion) - 闭包基础、递归实现、尾递归

### 实战总结
- [实战总结](./06-practical-summary) - 函数设计最佳实践

## 预计学习时间

- 基础学习：2 小时
- 练习巩固：1.5 小时
- 总计：3.5 小时

## 实战项目

本章将实现一个简单的数学计算库。

## 常见问题

- 函数的返回值为什么不需要 return 关键字？
- 如何在函数中返回多个值？

---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
# 运行单个示例
cargo run --example 01-function-basics
cargo run --example 02-params-returns
cargo run --example 03-ownership-functions
cargo run --example 04-advanced-functions
cargo run --example 05-closures-recursion
cargo run --example 06-function-review

# 编译检查所有示例
cargo check --examples

# Clippy 检查
cargo clippy --examples
```

