# 第 16 章：Trait

> 深入理解 Rust 的 Trait 系统和抽象能力。

## 学习目标

完成本章学习后，你将掌握：
- 定义和实现 Trait
- 使用默认实现和 Trait 继承
- 将 Trait 作为参数和返回值
- 理解 Trait 对象和动态分发
- 掌握常用标准库 Trait

## 章节内容

### Trait 基础
- [Trait 基础](./01-Trait基础.md) - Trait 定义、实现、孤岛规则

### 默认实现
- [默认实现](./02-默认实现.md) - 默认方法、Trait 继承

### Trait 作为参数
- [Trait 作为参数](./03-Trait作为参数.md) - impl Trait、Trait bound、条件实现

### 高级特性
- [高级特性](./04-高级特性.md) - 关联类型、关联常量、标记 Trait

### 标准库 Trait
- [标准库 Trait](./05-标准库Trait.md) - Display、Debug、Clone、Iterator 等

### Trait 对象
- [Trait 对象](./06-Trait对象.md) - dyn Trait、动态分发、对象安全

### 实战总结
- [实战总结](./07-实战总结.md) - Trait 设计原则

## 预计学习时间

- 基础学习：3 小时
- 练习巩固：2 小时
- 总计：5 小时

## 前置要求

- 已完成章节：第 01-15 章
- 知识点：泛型基础

## 实战项目

本章将为自定义类型实现多个标准库 Trait。

## 练习题答案

参见各小节内的练习部分。

## 常见问题

- Trait 对象和泛型如何选择？
- 什么是对象安全？