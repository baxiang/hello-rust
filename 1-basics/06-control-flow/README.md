# 第 06 章：控制流

> 掌握 Rust 的条件表达式、循环结构和模式匹配。

## 学习目标

完成本章学习后，你将掌握：
- 使用 if 表达式进行条件判断
- 掌握 loop、while、for 循环
- 理解 Rust 控制流作为表达式的特性
- 掌握基本的模式匹配语法

## 章节内容

### 条件表达式
- [条件表达式](./01-条件表达式.md) - if 表达式、else if、条件赋值

### 循环
- [循环](./02-循环.md) - loop、while、for、循环控制

### 模式匹配
- [模式匹配](./03-模式匹配.md) - match 表达式基础、模式绑定

### 实战总结
- [实战总结](./04-实战总结.md) - 控制流最佳实践

## 预计学习时间

- 基础学习：1.5 小时
- 练习巩固：1 小时
- 总计：2.5 小时

## 实战项目

本章将实现一个简单的猜数字游戏。

## 练习题答案

参见各小节内的练习部分。

## 常见问题

- if 为什么是表达式而不是语句？
- loop 和 while 应该选择哪个？
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
# 运行单个示例


# 编译检查所有示例
cargo check --examples

# Clippy 检查
cargo clippy --examples
```

---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-conditionals
cargo run --example 02-loops
cargo run --example 03-pattern-matching
cargo run --example 04-control-flow-review
cargo check --examples
cargo clippy --examples
```
