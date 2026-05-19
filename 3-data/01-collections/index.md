# 第 12 章：数组与 Vec

> 掌握 Rust 的数组和动态数组集合类型。

## 学习目标

完成本章学习后，你将掌握：
- 理解固定大小数组和动态数组 Vec 的区别
- 掌握数组的创建、访问和操作
- 掌握 Vec 的增删改查操作
- 理解容量和长度的区别
- 学会选择合适的数组类型

## 章节内容

### 数组基础
- [数组基础](./01-array-basics) - 数组语法、初始化、访问、遍历

### Vec 详解
- [Vec 详解](./02-vec-details) - Vec 创建、操作、容量管理

### 数组与 Vec
- [数组与 Vec](./03-array-vs-vec) - 性能对比、转换、选择策略

### 实战总结
- [实战总结](./04-practical-summary) - 集合类型最佳实践

## 预计学习时间

- 基础学习：1.5 小时
- 练习巩固：1 小时
- 总计：2.5 小时

## 实战项目

本章将实现一个简单的任务列表管理器。

## 常见问题

- 什么时候用数组，什么时候用 Vec？
- 如何避免 Vec 的频繁重新分配？
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-array-basics 02-vec-details 03-array-vec 04-collections-review
cargo check --examples
```

