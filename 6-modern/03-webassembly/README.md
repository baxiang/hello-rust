# 第 31 章：WebAssembly

> 使用 Rust 构建 WebAssembly 应用，实现高性能 Web 开发。

## 学习目标

完成本章学习后，你将掌握：
- 理解 WebAssembly 的基本概念和优势
- 掌握 Rust 编译到 WebAssembly 的方法
- 熟练使用 wasm-bindgen 实现 JavaScript 互操作
- 实现完整的 WebAssembly Web 应用

## 章节内容

### WASM 基础
- [WASM基础](./01-WASM基础.md) - 什么是 WebAssembly、编译到 WASM、wasm-pack

### wasm-bindgen
- [wasm-bindgen](./02-wasm-bindgen.md) - JavaScript 互操作、类型转换、DOM 操作

### 实战项目
- [实战项目](./03-实战项目.md) - 图像处理 Web 应用、性能对比

## 预计学习时间

- 基础学习：2 小时
- 实践练习：3 小时
- 总计：5 小时

## 实战项目

本章将实现一个图像处理 Web 应用，对比 WASM 和 JavaScript 的性能差异。

## 练习题答案

参见各小节内的练习部分。

## 常见问题

- WebAssembly 和 JavaScript 如何选择？
- 如何调试 WebAssembly 代码？
- WASM 模块如何打包和部署？

## 参考资源

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)