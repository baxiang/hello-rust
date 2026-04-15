# 第 19 章：迭代器

> 掌握 Rust 迭代器的使用和自定义迭代器。

## 学习目标

完成本章学习后，你将掌握：
- 理解迭代器的概念和 Iterator trait
- 掌握常用的迭代器适配器
- 学会使用消费适配器处理数据
- 实现自定义迭代器
- 理解迭代器的性能优势

## 章节内容

### 迭代器基础
- [迭代器基础](./01-迭代器基础.md) - Iterator trait、创建迭代器、next 方法

### 消费适配器
- [消费适配器](./02-消费适配器.md) - collect、sum、fold、for_each

### 迭代器适配器
- [迭代器适配器](./03-迭代器适配器.md) - map、filter、take、skip、zip 等

### 高级特性
- [高级特性](./04-高级特性.md) - 自定义迭代器、IntoIterator、迭代器组合

### 实战总结
- [实战总结](./05-实战总结.md) - 迭代器性能优化与最佳实践

## 预计学习时间

- 基础学习：2 小时
- 练习巩固：1.5 小时
- 总计：3.5 小时

## 实战项目

本章将实现一个数据处理流水线。

## 练习题答案

参见各小节内的练习部分。

## 常见问题

- 迭代器和 for 循环哪个性能更好？
- 如何组合多个迭代器？
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-iterator-basics 02-consuming-adapters 03-iterator-adapters 05-iterator-review
cargo check --examples
```
