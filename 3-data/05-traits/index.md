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
- [Trait 基础](./01-trait-basics) - Trait 定义、实现、孤岛规则

### 默认实现
- [默认实现](./02-default-impl) - 默认方法、Trait 继承

### Trait 作为参数
- [Trait 作为参数](./03-traits-as-params) - impl Trait、Trait bound、条件实现

### 高级特性
- [高级特性](./04-advanced-features) - 关联类型、关联常量、标记 Trait

### 标准库 Trait
- [标准库 Trait](./05-stdlib-traits) - Display、Debug、Clone、Iterator 等

### Trait 对象
- [Trait 对象](./06-trait-objects) - dyn Trait、动态分发、对象安全

### Trait 设计模式
- [Trait 设计模式](./08-trait-design-patterns) - 组合vs继承、标准库 Trait 深入、高级模式

### 实战总结
- [实战总结](./07-practical-summary) - Trait 设计原则、完整案例

### 练习题
- [Trait 练习题](../exercises) - 15 题实战练习

## 预计学习时间

- 基础学习：4 小时
- 高级模式学习：3 小时
- 练习巩固：3 小时
- 总计：10 小时

## 实战项目

本章包含以下实战案例：
- 形状绘制系统（Canvas）
- 插件系统设计
- ORM 系统模拟
- 货币转换系统

## 练习题答案

参见 [Trait 练习题](../exercises)。

## 常见问题

- Trait 对象和泛型如何选择？
- 什么是对象安全？
- 关联类型 vs 泛型参数？
- 如何设计组合式 Trait？
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-trait-basics 02-default-impl 03-trait-as-param 04-trait-advanced 05-standard-traits 06-trait-objects 08-trait-patterns
cargo check --examples
```

