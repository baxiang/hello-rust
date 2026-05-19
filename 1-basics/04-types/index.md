# 第 04 章：数据类型

> 掌握 Rust 的类型系统，包括基本类型、复合类型和字符串类型。

## 学习目标

完成本章学习后，你将掌握：
- 理解 Rust 类型系统的设计理念
- 掌握整数、浮点数、布尔值、字符等基本类型
- 理解元组、数组等复合类型
- 掌握 String 和 &str 的区别与用法
- 了解类型转换与类型推断

## 章节内容

### 类型系统概述
- [类型系统概述](./01-type-system-overview) - 静态类型、强类型、类型推断

### 基本类型
- [基本类型](./02-basic-types) - 整数、浮点数、布尔值、字符类型

### 复合类型
- [复合类型](./03-compound-types) - 元组、数组的定义与使用

### 字符串类型
- [字符串类型](./04-string-type) - String 与 &str 的区别、转换、操作

### 实战总结
- [实战总结](./05-practical-summary) - 类型选择最佳实践

## 预计学习时间

- 基础学习：2 小时
- 练习巩固：1 小时
- 总计：3 小时

## 实战项目

本章将实现一个简单的数据类型转换工具。

## 常见问题

- String 和 &str 什么时候使用哪个？
- 为什么整数类型有这么多变种？

---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
# 运行单个示例
cargo run --example 01-type-system-overview
cargo run --example 02-basic-types
cargo run --example 03-compound-types
cargo run --example 04-string-types
cargo run --example 05-type-review

# 编译检查所有示例
cargo check --examples

# Clippy 检查
cargo clippy --examples
```

