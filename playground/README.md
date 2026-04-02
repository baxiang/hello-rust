# Rust Playground - 交互式示例

> 在线运行 Rust 代码，快速理解核心概念

## 使用说明

### 本地运行

```bash
# 进入示例目录
cd playground/01-ownership

# 运行示例
cargo run

# 运行测试
cargo test

# 查看输出
cargo run | less
```

### 在线运行

每个示例提供 [Rust Playground](https://play.rust-lang.org/) 链接，无需本地环境即可运行。

---

## 核心概念示例

### 第一部分：基础入门

| 示例 | 难度 | 概念 | 在线运行 |
|------|------|------|----------|
| [Hello World](./01-hello-world/) | 🟢 初级 | 基本语法 | [Playground](#) |
| [变量与类型](./02-variables/) | 🟢 初级 | let, mut, 类型推断 | [Playground](#) |
| [函数](./03-functions/) | 🟢 初级 | fn, 参数, 返回值 | [Playground](#) |
| [控制流](./04-control-flow/) | 🟡 中级 | if, loop, match | [Playground](#) |

### 第二部分：核心概念

| 示例 | 难度 | 概念 | 在线运行 |
|------|------|------|----------|
| [所有权基础](./05-ownership-basics/) | 🟡 中级 | 移动, 克隆, 作用域 | [Playground](#) |
| [引用与借用](./06-references/) | 🟡 中级 | &T, &mut T, 借用规则 | [Playground](#) |
| [生命周期](./07-lifetimes/) | 🔴 高级 | 'a, 生命周期标注 | [Playground](#) |
| [切片类型](./08-slices/) | 🟡 中级 | &[T], 字符串切片 | [Playground](#) |

### 第三部分：数据结构

| 示例 | 难度 | 概念 | 在线运行 |
|------|------|------|----------|
| [结构体](./09-structs/) | 🟢 初级 | struct, impl, 方法 | [Playground](#) |
| [枚举与匹配](./10-enums/) | 🟡 中级 | enum, match, Option | [Playground](#) |
| [集合类型](./11-collections/) | 🟡 中级 | Vec, HashMap, HashSet | [Playground](#) |
| [错误处理](./12-error-handling/) | 🟡 中级 | Result, ?, 自定义错误 | [Playground](#) |

### 第四部分：高级特性

| 示例 | 难度 | 概念 | 在线运行 |
|------|------|------|----------|
| [泛型](./13-generics/) | 🟡 中级 | 泛型函数, 结构体, 约束 | [Playground](#) |
| [Trait](./14-traits/) | 🟡 中级 | Trait 定义, 实现, 对象 | [Playground](#) |
| [闭包](./15-closures/) | 🟡 中级 | 闭包语法, 捕获, Fn traits | [Playground](#) |
| [迭代器](./16-iterators/) | 🟡 中级 | Iterator, 适配器, 消费器 | [Playground](#) |

### 第五部分：并发与安全

| 示例 | 难度 | 概念 | 在线运行 |
|------|------|------|----------|
| [智能指针](./17-smart-pointers/) | 🟡 中级 | Box, Rc, Arc, RefCell | [Playground](#) |
| [并发编程](./18-concurrency/) | 🔴 高级 | 线程, 消息传递, 互斥锁 | [Playground](#) |
| [Unsafe Rust](./19-unsafe/) | 🔴 高级 | unsafe 块, 原始指针 | [Playground](#) |
| [宏](./20-macros/) | 🔴 高级 | 声明宏, 过程宏 | [Playground](#) |

---

## 示例结构

每个示例包含：

```
playground/xx-topic/
├── Cargo.toml          # 项目配置
├── src/
│   ├── main.rs         # 主程序
│   ├── lib.rs          # 库代码（可选）
│   └── examples/       # 额外示例（可选）
├── tests/              # 测试代码（可选）
└── README.md           # 说明文档
```

### README 格式

```markdown
# 示例名称

> 简短描述

## 核心概念

- 概念 1
- 概念 2

## 代码示例

```rust
// 示例代码
```

## 预期输出

```
// 输出结果
```

## 练习

1. 尝试修改 X
2. 观察错误 Y
3. 思考为什么 Z
```

---

## 快速开始示例

### 示例 1：所有权移动

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权移动到 s2
    
    // println!("{}", s1);  // ❌ 错误：s1 已失效
    println!("{}", s2);     // ✅ 正确
}
```

**在线运行：** [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=example)

---

### 示例 2：借用检查

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;       // 不可变借用
    let r2 = &s;       // 另一个不可变借用
    println!("{} {}", r1, r2);  // ✅ 可以有多个不可变借用
    
    let r3 = &mut s;   // ❌ 错误：已有不可变借用
    println!("{}", r3);
}
```

**在线运行：** [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=example)

---

### 示例 3：生命周期标注

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result);  // ✅ 正确
    }
    // println!("{}", result);  // ❌ 错误：string2 已失效
}
```

**在线运行：** [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=example)

---

## 调试技巧

### 使用 dbg! 宏

```rust
fn main() {
    let x = 5;
    let y = 10;
    dbg!(x + y);  // 输出：[src/main.rs:4] x + y = 15
}
```

### 查看类型

```rust
fn main() {
    let x = 5;
    let _: () = x;  // 编译错误会显示类型
}
```

### 打印内存布局

```rust
use std::mem;

fn main() {
    println!("Size of String: {}", mem::size_of::<String>());  // 24 bytes
    println!("Size of &str: {}", mem::size_of::<&str>());      // 16 bytes
}
```

---

## 贡献示例

欢迎添加新的交互示例：

1. 在 `playground/` 创建新目录
2. 添加 `Cargo.toml` 和 `src/main.rs`
3. 创建 `README.md` 说明
4. 测试示例可运行
5. 提交 Pull Request

详见 [CONTRIBUTING.md](../CONTRIBUTING.md)

---

## 相关资源

- [Rust Playground](https://play.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings 练习](https://github.com/rust-lang/rustlings)
- [可视化图解](../docs/diagrams/)

---

**更新日志：**
- 2026-04-03: 创建 playground 框架