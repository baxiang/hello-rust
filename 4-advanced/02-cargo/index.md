# 第 21 章：Cargo 与 Crates

> 掌握 Cargo 构建系统和 crate 管理。

## 学习目标

完成本章学习后，你将掌握：
- 理解 Cargo 的项目结构
- 掌握 Cargo.toml 配置方法
- 使用常用 Cargo 命令
- 管理外部依赖
- 发布 crate 到 crates.io

## 章节内容

### Cargo 基础
- [Cargo 基础](./01-cargo-basics) - 项目创建、项目结构、基本命令

### Cargo 配置
- [Cargo 配置](./02-cargo-config) - Cargo.toml 详解、依赖管理、特性配置

### Cargo 命令
- [Cargo 命令](./03-cargo-commands) - build、run、test、doc、publish 等

### 高级特性
- [高级特性](./04-advanced-features) - 工作空间、构建脚本、条件编译

### 发布与优化
- [发布与优化](./05-publish-and-optimize) - 发布配置、性能优化、CI/CD

### 实战总结
- [实战总结](./06-practical-summary) - Cargo 最佳实践

## 预计学习时间

- 基础学习：2 小时
- 练习巩固：1.5 小时
- 总计：3.5 小时

## 实战项目

本章将发布一个简单的 crate 到 crates.io。

## 常见问题

- 如何选择合适的 crate 版本？
- 如何处理版本冲突？
---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
cargo run --example 01-cargo-basics
cargo check --examples
```

