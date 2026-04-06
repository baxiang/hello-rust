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

### Trait 设计模式
- [Trait 设计模式](./08-Trait设计模式.md) - 组合vs继承、标准库 Trait 深入、高级模式

### 实战总结
- [实战总结](./07-实战总结.md) - Trait 设计原则、完整案例

### 练习题
- [Trait 练习题](../traits-exercises.md) - 15 题实战练习

## 预计学习时间

- 基础学习：4 小时
- 高级模式学习：3 小时
- 练习巩固：3 小时
- 总计：10 小时

## 前置要求

- 已完成章节：第 01-15 章
- 知识点：泛型基础、引用

## 实战项目

本章包含以下实战案例：
- 形状绘制系统（Canvas）
- 插件系统设计
- ORM 系统模拟
- 货币转换系统

## 练习题答案

参见 [Trait 练习题](../traits-exercises.md)。

## 常见问题

- Trait 对象和泛型如何选择？
- 什么是对象安全？
- 关联类型 vs 泛型参数？
- 如何设计组合式 Trait？