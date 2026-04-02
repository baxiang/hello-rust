## 1.3 Rust 工具链详解

### rustup - 工具链管理器

`rustup` 是 Rust 的官方工具链管理器，可以：

```bash
# 查看已安装的工具链
rustup show

# 更新 Rust 到最新版本
rustup update

# 安装特定版本的 Rust
rustup install 1.70.0

# 切换到特定版本
rustup default 1.70.0

# 查看可用的组件
rustup component list

# 添加 rustfmt（代码格式化）
rustup component add rustfmt

# 添加 clippy（代码检查）
rustup component add clippy

# 添加 rust-docs（本地文档）
rustup component add rust-docs
```

### cargo - 包管理器

Cargo 是 Rust 的构建工具和包管理器：

```bash
# 创建新项目
cargo new my_project

# 构建项目
cargo build

# 运行项目
cargo run

# 运行测试
cargo test

# 检查代码（快速）
cargo check

# 格式化代码
cargo fmt

# 代码检查（linter）
cargo clippy

# 生成文档
cargo doc --open

# 清理构建文件
cargo clean
```

### 项目结构

创建一个项目看看结构：

```bash
cargo new hello_rust
cd hello_rust
ls -la
```

你会看到：

```
hello_rust/
├── .git/              # Git 仓库
├── .gitignore         # Git 忽略文件
├── Cargo.toml         # 项目配置文件
└── src/
    └── main.rs        # 源代码
```

#### Cargo.toml 详解

```toml
[package]
name = "hello_rust"      # 项目名称
version = "0.1.0"        # 版本号（语义化版本）
edition = "2021"         # Rust 版本（2015/2018/2021）
authors = ["你的名字"]    # 作者信息（可选）

[dependencies]           # 依赖项
# 这里添加第三方库
# 例如：serde = "1.0"
```

#### main.rs 详解

```rust
// 这是 main.rs 文件
// fn: 定义函数
// main: 程序入口函数
// (): 无参数
// {}: 函数体

fn main() {
    // println!: 打印宏（注意感叹号）
    // ! 表示这是宏，不是函数
    // 宏在编译时展开，可以生成代码
    println!("Hello, Rust!");
}
```

---

---

## 下一步

[运行第一个程序](../4-运行第一个程序.md)