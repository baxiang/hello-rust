# 第 03 章：变量与可变性

> 理解 Rust 的核心特性：变量绑定、可变性控制与常量定义。

## 学习目标

完成本章学习后，你将掌握：
- 理解变量绑定与可变性的概念
- 掌握 `let`、`mut`、`const`、`static` 的用法区别
- 理解变量遮蔽（Shadowing）机制
- 了解变量命名最佳实践

## 章节内容

### 变量基础
- [变量基础](./01-变量基础.md) - let 绑定、可变性、类型推断

### 常量与静态
- [常量与静态](./02-常量与静态.md) - const、static 的区别与使用场景

### 高级特性
- [高级特性](./03-高级特性.md) - 变量遮蔽、解构赋值、模式匹配入门

### 实战总结
- [实战总结](./04-实战总结.md) - 综合练习与最佳实践

## 预计学习时间

- 基础学习：1.5 小时
- 练习巩固：1 小时
- 总计：2.5 小时

## 实战项目

本章将实现一个简单的计算器变量管理模块。

## 练习题答案

参见各小节内的练习部分。

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
# 运行单个示例
cargo run --example 01-let-binding
cargo run --example 02-mut-variable
cargo run --example 03-shadowing
cargo run --example 04-const-static
cargo run --example 05-chapter-review

# 编译检查所有示例
cargo check --examples

# Clippy 检查
cargo clippy --examples
```

## 常见问题

- 为什么 Rust 默认变量不可变？
- 变量遮蔽和可变变量有什么区别？