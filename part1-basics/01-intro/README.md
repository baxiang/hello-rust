# 第 01 章：简介与环境搭建

> **本章代码基于 Rust 2024 Edition (Rust 1.85+) 编写**

> 开启 Rust 学习之旅，搭建开发环境，了解 Rust 的优势与应用场景。

---

## 第 1 步：实际场景引入

### 为什么需要编程？

想象这样一个场景：你需要处理 1000 个日志文件，每个文件需要提取特定格式的错误信息并汇总。

**手动操作的方式：**

```
┌─────────────────────────────────────────────────────────────┐
│                    手动处理日志                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. 打开文件 1 → 搜索关键词 → 复制结果 → 粘贴到汇总文件      │
│  2. 打开文件 2 → 搜索关键词 → 复制结果 → 粘贴到汇总文件      │
│  3. 打开文件 3 → 搜索关键词 → 复制结果 → 粘贴到汇总文件      │
│  ...                                                        │
│  1000. 打开文件 1000 → ...                                  │
│                                                             │
│  ⏱️ 预计耗时：8 小时+                                        │
│  ❌ 容易出错：漏掉文件、复制错误                              │
│  😫 枯燥乏味：重复劳动                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**程序自动化的方式：**

```
┌─────────────────────────────────────────────────────────────┐
│                    程序自动处理                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  $ cargo run --release                                      │
│  正在处理日志文件...                                         │
│  已处理: 1000/1000                                          │
│  找到错误: 342 条                                           │
│  结果已保存到: errors_report.txt                            │
│                                                             │
│  ⏱️ 耗时：3 秒                                              │
│  ✅ 准确无误：程序不会疲劳、不会遗漏                          │
│  🎯 可重复使用：明天再来 1000 个文件，还是 3 秒              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 为什么选择 Rust？

市面上有很多编程语言，为什么偏偏选择 Rust？

```
┌─────────────────────────────────────────────────────────────┐
│                    编程语言对比                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Python  ──✅ 易学  ──❌ 慢（解释执行）                       │
│  Java    ──✅ 安全 ──❌ 内存占用大（垃圾回收）                │
│  C/C++   ──✅ 快   ──❌ 内存不安全（段错误、内存泄漏）         │
│                                                             │
│  Rust    ──✅ 易学（逐步学习曲线）                           │
│          ──✅ 安全（编译时内存检查）                          │
│          ──✅ 快（零成本抽象，媲美 C/C++）                    │
│          ──✅ 低内存占用（无垃圾回收）                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Rust 的独特价值：**

| 场景 | 传统方案的问题 | Rust 的解决方案 |
|------|---------------|----------------|
| 系统编程 | C/C++ 内存泄漏、段错误 | 所有权系统在编译时防止 |
| Web 服务 | Java/Go 内存占用高 | 零成本抽象，内存可控 |
| 嵌入式 | C 不安全，调试困难 | 安全 + 高性能 + 工具链 |
| CLI 工具 | Python 依赖环境 | 单二进制文件，无依赖 |

---

## 第 2 步：概念动机

### Rust 解决了什么问题？

让我们看一个真实的内存错误案例：

```rust
// ❌ C/C++ 中常见的内存错误（伪代码示意）
// 
// char* buffer = new char[100];
// // ... 使用 buffer
// // 忘记 delete 了！内存泄漏！
// 
// // 或者更糟：
// delete buffer;
// buffer[0] = 'a';  // 使用已释放的内存！程序崩溃！
```

这种**内存错误**是 C/C++ 程序中最常见的 bug 来源，可能导致：
- 程序崩溃
- 数据损坏
- 安全漏洞

**Rust 的解决方案：**

```rust
// ✅ Rust 代码 - 编译器帮你管理内存
fn main() {
    let buffer = String::with_capacity(100);
    // 使用 buffer
    // 离开作用域时，Rust 自动释放内存！
    // 不可能忘记释放，也不可能使用已释放的内存
}
```

### Rust 的核心优势

```
┌─────────────────────────────────────────────────────────────┐
│                    Rust 的核心优势                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  🛡️ 内存安全                                                │
│     └─ 无需垃圾回收，编译时检查所有内存操作                   │
│     └─ 消除空指针、悬垂指针、数据竞争                        │
│                                                             │
│  ⚡ 高性能                                                   │
│     └─ 零成本抽象：高级特性不牺牲性能                        │
│     └─ 媲美 C/C++ 的运行速度                                │
│                                                             │
│  🧵 并发安全                                                 │
│     └─ 编译器防止数据竞争                                   │
│     └─ "无畏并发"：放心写多线程代码                          │
│                                                             │
│  📦 现代工具链                                               │
│     └─ Cargo：包管理、构建、测试、文档一体化                │
│     └─ rustup：轻松管理多版本 Rust                          │
│                                                             │
│  📚 优秀文档                                                 │
│     └─ 内置文档系统 rustdoc                                 │
│     └─ 丰富的标准库文档                                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 谁在使用 Rust？

| 公司/项目 | 用途 |
|-----------|------|
| **Microsoft** | Windows 内核、Azure 服务 |
| **Google** | Android 系统、Fuchsia OS |
| **Amazon AWS** | Firecracker、Bottlerocket |
| **Meta** | 后端服务、Source Control |
| **Cloudflare** | 网络服务、边缘计算 |
| **Discord** | 实时消息服务（从 Go 迁移到 Rust） |
| **Firefox** | 渲染引擎组件 |

---

## 第 3 步：最简示例

### 5 分钟上手 Rust

**目标：** 创建并运行你的第一个 Rust 程序。

```bash
# 1. 创建新项目（约 10 秒）
cargo new hello_rust

# 2. 进入项目目录
cd hello_rust

# 3. 运行程序（首次约 30 秒，后续约 1 秒）
cargo run
```

**输出：**

```
   Compiling hello_rust v0.1.0 (/path/to/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/hello_rust`
Hello, world!
```

**恭喜！** 你已经成功运行了第一个 Rust 程序！

### 项目结构

```
hello_rust/
├── Cargo.toml        # 项目配置文件
└── src/
    └── main.rs       # 源代码（入口文件）
```

**main.rs 的内容：**

```rust
fn main() {
    println!("Hello, world!");
}
```

**代码解析：**

```
┌─────────────────────────────────────────────────────────────┐
│                    Hello World 代码解析                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  fn main() {                                                │
│  │  │   │                                                   │
│  │  │   └─ () 无参数                                        │
│  │  └─ main 程序入口函数名                                  │
│  └─ fn 定义函数的关键字                                     │
│                                                             │
│      println!("Hello, world!");                            │
│      │       │                                              │
│      │       └─ 字符串内容                                  │
│      └─ println! 打印宏（! 表示这是宏，不是函数）           │
│  }                                                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 第 4 步：详细讲解

### 4.1 安装 Rust

#### macOS / Linux 安装

打开终端，执行以下命令：

```bash
# 下载并运行安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**命令解析：**

```
┌─────────────────────────────────────────────────────────────┐
│                    安装命令解析                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs │ sh
│       │              │         │    │                      │ │
│       │              │         │    │                      └─ 执行脚本
│       │              │         │    └─ 安装脚本 URL
│       │              │         └─ 安静模式 + 显示错误
│       │              └─ 最低 TLS 版本 1.2
│       └─ 只允许 HTTPS 协议（安全）
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

安装完成后，你会看到：

```
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

执行以下命令加载环境变量：

```bash
source "$HOME/.cargo/env"
```

#### Windows 安装

1. 访问 [https://rustup.rs](https://rustup.rs)
2. 下载 `rustup-init.exe`
3. 双击运行，按照提示安装（默认选项即可）

> **注意：** Windows 用户需要先安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，选择 "C++ build tools" 工作负载。

#### 验证安装

```bash
# 检查 Rust 编译器版本
rustc --version

# 检查 Cargo（包管理器）版本
cargo --version

# 检查 rustup（工具链管理器）版本
rustup --version
```

**预期输出：**

```
rustc 1.85.0 (4d91de4e4 2025-01-15)
cargo 1.85.0 (d73d2cab9 2025-01-14)
rustup 1.27.1 (54dd3d00f 2024-04-24)
```

> **提示：** 版本号可能不同，这没关系。建议保持最新版本。

### 4.2 工具链详解

Rust 工具链由三个核心组件构成：

```
┌─────────────────────────────────────────────────────────────┐
│                    Rust 工具链架构                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                    ┌─────────────┐                          │
│                    │   rustup    │  工具链管理器             │
│                    │  (管理版本)  │                          │
│                    └──────┬──────┘                          │
│                           │                                 │
│              ┌────────────┼────────────┐                    │
│              │            │            │                    │
│              ▼            ▼            ▼                    │
│        ┌──────────┐ ┌──────────┐ ┌──────────┐               │
│        │  rustc   │ │  cargo   │ │ rust-docs │               │
│        │ (编译器)  │ │(包管理器)│ │ (文档)    │               │
│        └──────────┘ └──────────┘ └──────────┘               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### rustup - 工具链管理器

```bash
# 查看已安装的工具链
rustup show

# 更新 Rust 到最新版本
rustup update

# 安装特定版本的 Rust
rustup install 1.85.0

# 切换到特定版本
rustup default 1.85.0

# 查看可用的组件
rustup component list

# 添加常用组件
rustup component add rustfmt   # 代码格式化
rustup component add clippy    # 代码检查
rustup component add rust-docs # 本地文档
```

#### cargo - 包管理器与构建工具

Cargo 是 Rust 的核心工具，集成了包管理、构建、测试等功能：

```bash
# 创建新项目
cargo new my_project        # 可执行项目
cargo new my_lib --lib      # 库项目

# 构建与运行
cargo build                 # 构建（调试模式）
cargo build --release       # 构建（发布模式）
cargo run                   # 构建并运行
cargo check                 # 快速检查（不生成二进制）

# 测试与质量
cargo test                  # 运行测试
cargo fmt                   # 格式化代码
cargo clippy                # 代码检查（linter）

# 文档
cargo doc --open            # 生成并打开文档

# 清理
cargo clean                 # 清理构建文件
```

### 4.3 项目结构详解

创建项目后，目录结构如下：

```
hello_rust/
├── .git/              # Git 仓库（cargo new 自动初始化）
├── .gitignore         # Git 忽略文件
├── Cargo.toml         # 项目配置文件
└── src/
    └── main.rs        # 源代码（入口文件）
```

#### Cargo.toml 配置文件

```toml
[package]
name = "hello_rust"      # 项目名称
version = "0.1.0"        # 版本号（语义化版本）
edition = "2024"         # Rust Edition（2015/2018/2021/2024）
authors = ["Your Name"]  # 作者信息（可选）

[dependencies]           # 依赖项
# 在这里添加第三方库
# 例如：serde = "1.0"
```

**Edition 说明：**

| Edition | 发布年份 | 主要特性 |
|---------|---------|---------|
| 2015 | 2015 | 初始版本 |
| 2018 | 2018 | 模块系统改进 |
| 2021 | 2021 | 闭包特性、数组 IntoIterator |
| 2024 | 2024 | RPITIT 稳定、async 闭包 |

#### main.rs 源文件

```rust
// src/main.rs - 程序入口文件

fn main() {
    println!("Hello, world!");
}
```

**关键概念：**

- `fn` - 定义函数的关键字
- `main` - 程序入口函数名（固定名称）
- `()` - 无参数
- `println!` - 打印宏（`!` 表示这是宏，不是函数）

### 4.4 调试模式 vs 发布模式

```
┌─────────────────────────────────────────────────────────────┐
│                    构建模式对比                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  调试模式 (dev)                    发布模式 (release)        │
│  ─────────────────                 ─────────────────        │
│  cargo build                       cargo build --release    │
│  输出: target/debug/               输出: target/release/    │
│                                                             │
│  ✅ 编译速度快                     ⏱️ 编译速度慢             │
│  ✅ 包含调试信息                   ❌ 无调试信息             │
│  ❌ 运行速度慢                     ✅ 运行速度快             │
│  ❌ 二进制文件大                   ✅ 二进制文件小           │
│                                                             │
│  适用场景：                        适用场景：                 │
│  - 开发调试                        - 生产部署               │
│  - 单元测试                        - 性能测试               │
│  - 学习阶段                        - 发布分发               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**性能对比示例：**

```bash
# 构建两个版本
cargo build
cargo build --release

# 查看文件大小
ls -lh target/debug/hello_rust
ls -lh target/release/hello_rust
```

通常 release 版本运行速度比 debug 版本快 **10-100 倍**。

### 4.5 配置开发环境

#### VS Code 配置（推荐）

1. **安装 VS Code**：[https://code.visualstudio.com](https://code.visualstudio.com)

2. **安装扩展**：
   - 打开 VS Code
   - 按 `Cmd+Shift+X`（macOS）或 `Ctrl+Shift+X`（Windows/Linux）
   - 搜索并安装：

   | 扩展名 | 用途 | 必要性 |
   |--------|------|--------|
   | `rust-analyzer` | 代码补全、类型提示、错误检查 | 必须 |
   | `CodeLLDB` | 调试支持 | 推荐 |
   | `Better TOML` | Cargo.toml 语法高亮 | 推荐 |
   | `crates` | 依赖版本提示 | 可选 |

3. **rust-analyzer 功能**：
   - 实时代码补全
   - 类型提示（内联显示）
   - 错误检查（红色波浪线）
   - 跳转到定义
   - 重命名符号

#### 其他编辑器

| 编辑器 | 插件/配置 |
|--------|----------|
| **IntelliJ IDEA** | Rust 插件（官方支持） |
| **Vim/Neovim** | rust.vim + rust-analyzer (LSP) |
| **Emacs** | rust-mode + lsp-mode |
| **Sublime Text** | RustEnhanced |
| **Helix** | 内置 LSP 支持 |

### 4.6 常见问题

#### Q1: 下载慢怎么办？

配置国内镜像（以中科大为例）：

```bash
# 创建配置目录
mkdir -p ~/.cargo

# 写入配置文件
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

[net]
git-fetch-with-cli = true
EOF
```

#### Q2: 编译错误：找不到 rustc

确保已加载环境变量：

```bash
# 方法一：加载环境变量
source "$HOME/.cargo/env"

# 方法二：重启终端

# 方法三：添加到 shell 配置文件
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc  # bash
echo 'source "$HOME/.cargo/env"' >> ~/.zshrc   # zsh
```

#### Q3: 如何卸载 Rust？

```bash
rustup self uninstall
```

#### Q4: 如何更新 Rust？

```bash
rustup update
```

#### Q5: 如何查看本地文档？

```bash
# 打开标准库文档
rustup doc

# 打开 Rust Book
rustup doc --book
```

---

## 第 5 步：渐进复杂化

### 层级 1：创建并运行第一个项目

**目标：** 熟悉 `cargo new` 和 `cargo run` 命令。

```bash
# 创建项目
cargo new hello_rust

# 进入目录
cd hello_rust

# 运行项目
cargo run
```

**输出：**

```
   Compiling hello_rust v0.1.0 (/path/to/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/hello_rust`
Hello, world!
```

**输出解读：**

```
┌─────────────────────────────────────────────────────────────┐
│                    cargo run 输出解读                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Compiling hello_rust v0.1.0                                │
│  │         │                                                │
│  │         └─ 项目版本                                      │
│  └─ 正在编译                                                │
│                                                             │
│  Finished dev [unoptimized + debuginfo] target(s) in 0.23s │
│  │         │   │               │                            │
│  │         │   │               └─ 包含调试信息              │
│  │         │   └─ 未优化（debug 模式）                      │
│  │         └─ 构建模式                                       │
│  └─ 编译完成                                                │
│                                                             │
│  Running `target/debug/hello_rust`                          │
│  │        │                                                  │
│  │        └─ 可执行文件路径                                  │
│  └─ 正在运行                                                │
│                                                             │
│  Hello, world!                                              │
│  └─ 程序输出                                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 层级 2：修改程序内容

**目标：** 修改 `main.rs`，体验 Rust 的基本语法。

编辑 `src/main.rs`：

```rust
fn main() {
    // 打印多行文本
    println!("你好，Rust!");
    println!("这是第二行");
    println!("第三行...");
    
    // 打印带格式的文本
    println!("================");
    println!("  Rust 学习笔记  ");
    println!("================");
}
```

运行：

```bash
cargo run
```

**输出：**

```
你好，Rust!
这是第二行
第三行...
================
  Rust 学习笔记  
================
```

**练习：** 尝试添加更多 `println!` 语句，观察输出。

### 层级 3：创建库项目

**目标：** 了解可执行项目与库项目的区别。

```bash
# 创建库项目（--lib 参数）
cargo new my_lib --lib

# 进入目录
cd my_lib

# 查看结构
ls -la
```

**库项目结构：**

```
my_lib/
├── Cargo.toml
└── src/
    └── lib.rs    # 注意：是 lib.rs 不是 main.rs
```

**Cargo.toml 对比：**

```toml
# 可执行项目
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

# 库项目（结构相同，但用途不同）
[package]
name = "my_lib"
version = "0.1.0"
edition = "2024"
```

编辑 `src/lib.rs`：

```rust
/// 计算两个数的和
/// 
/// # Examples
/// ```
/// let result = my_lib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
```

运行测试：

```bash
cargo test
```

**输出：**

```
   Compiling my_lib v0.1.0 (/path/to/my_lib)
    Finished test [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/my_lib-...)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**可执行项目 vs 库项目：**

```
┌─────────────────────────────────────────────────────────────┐
│                    项目类型对比                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  可执行项目 (cargo new name)                                │
│  ─────────────────────────                                  │
│  入口文件: src/main.rs                                      │
│  入口函数: fn main()                                        │
│  输出产物: 可执行二进制文件                                  │
│  用途: CLI 工具、服务程序、应用                              │
│                                                             │
│  库项目 (cargo new name --lib)                              │
│  ─────────────────────────                                  │
│  入口文件: src/lib.rs                                       │
│  入口函数: 无（库没有 main）                                 │
│  输出产物: .rlib 库文件                                     │
│  用途: 供其他项目引用的代码库                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 第 6 步：实际应用

### 项目初始化脚本

以下是一个实用的项目初始化脚本，综合运用本章知识点：

```bash
#!/bin/bash
# init_rust_project.sh - Rust 项目初始化脚本

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 打印带颜色的信息
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查 Rust 是否安装
check_rust() {
    if ! command -v rustc &> /dev/null; then
        error "Rust 未安装，请先运行："
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    info "Rust 版本: $(rustc --version)"
}

# 创建项目
create_project() {
    local project_name=$1
    local project_type=${2:-bin}  # 默认为可执行项目
    
    info "创建项目: $project_name"
    
    if [ "$project_type" = "lib" ]; then
        cargo new "$project_name" --lib
    else
        cargo new "$project_name"
    fi
    
    cd "$project_name"
}

# 配置项目
configure_project() {
    info "配置项目..."
    
    # 创建常用目录
    mkdir -p src/bin
    mkdir -p tests
    mkdir -p examples
    mkdir -p docs
    
    # 创建 README
    cat > README.md << 'EOF'
# 项目名称

简短描述

## 使用方法

```bash
cargo run
```

## 测试

```bash
cargo test
```

## 文档

```bash
cargo doc --open
```
EOF

    # 创建 .gitignore 补充
    cat >> .gitignore << 'EOF'

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
EOF

    info "项目配置完成"
}

# 构建并测试
build_and_test() {
    info "构建项目..."
    cargo build
    
    info "运行测试..."
    cargo test
    
    info "格式化代码..."
    cargo fmt
    
    info "代码检查..."
    cargo clippy 2>/dev/null || warn "clippy 未安装，运行: rustup component add clippy"
}

# 主函数
main() {
    local project_name=${1:-"my_project"}
    local project_type=${2:-"bin"}
    
    echo "========================================"
    echo "  Rust 项目初始化脚本"
    echo "========================================"
    echo ""
    
    check_rust
    create_project "$project_name" "$project_type"
    configure_project
    build_and_test
    
    echo ""
    echo "========================================"
    info "项目创建成功！"
    echo "========================================"
    echo ""
    echo "项目名称: $project_name"
    echo "项目类型: $project_type"
    echo "项目路径: $(pwd)"
    echo ""
    echo "常用命令:"
    echo "  cargo run      - 运行项目"
    echo "  cargo build    - 构建项目"
    echo "  cargo test     - 运行测试"
    echo "  cargo doc      - 生成文档"
    echo ""
}

# 运行主函数
main "$@"
```

**使用方法：**

```bash
# 创建可执行项目
./init_rust_project.sh my_app

# 创建库项目
./init_rust_project.sh my_lib lib
```

**输出示例：**

```
========================================
  Rust 项目初始化脚本
========================================

[INFO] Rust 版本: rustc 1.85.0 (4d91de4e4 2025-01-15)
[INFO] 创建项目: my_app
[INFO] 配置项目...
[INFO] 构建项目...
[INFO] 运行测试...
[INFO] 格式化代码...
[INFO] 代码检查...

========================================
[INFO] 项目创建成功！
========================================

项目名称: my_app
项目类型: bin
项目路径: /path/to/my_app

常用命令:
  cargo run      - 运行项目
  cargo build    - 构建项目
  cargo test     - 运行测试
  cargo doc      - 生成文档
```

---

## 小结

### 本章要点 Checklist

完成本章学习后，请确认你已掌握：

- [ ] 理解 Rust 的核心优势（内存安全、高性能、并发安全）
- [ ] 成功安装 Rust 工具链（rustc、cargo、rustup）
- [ ] 能够使用 `cargo new` 创建项目
- [ ] 能够使用 `cargo run` 运行项目
- [ ] 理解调试模式与发布模式的区别
- [ ] 配置好开发环境（VS Code + rust-analyzer）
- [ ] 了解可执行项目与库项目的区别

### 关键命令速查

```bash
# ═══════════════════════════════════════════════════════════
# 安装与更新
# ═══════════════════════════════════════════════════════════
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # 安装
rustup update                                                    # 更新
rustup self uninstall                                            # 卸载

# ═══════════════════════════════════════════════════════════
# 版本检查
# ═══════════════════════════════════════════════════════════
rustc --version    # 编译器版本
cargo --version    # 包管理器版本
rustup --version   # 工具链管理器版本

# ═══════════════════════════════════════════════════════════
# 项目管理
# ═══════════════════════════════════════════════════════════
cargo new <name>           # 创建可执行项目
cargo new <name> --lib     # 创建库项目
cargo init                 # 在当前目录初始化项目

# ═══════════════════════════════════════════════════════════
# 构建与运行
# ═══════════════════════════════════════════════════════════
cargo run                  # 构建并运行（调试模式）
cargo run --release        # 构建并运行（发布模式）
cargo build                # 构建（调试模式）
cargo build --release      # 构建（发布模式）
cargo check                # 快速检查（不生成二进制）

# ═══════════════════════════════════════════════════════════
# 测试与质量
# ═══════════════════════════════════════════════════════════
cargo test                 # 运行测试
cargo fmt                  # 格式化代码
cargo clippy               # 代码检查（linter）

# ═══════════════════════════════════════════════════════════
# 文档
# ═══════════════════════════════════════════════════════════
cargo doc --open           # 生成并打开文档
rustup doc                 # 打开本地标准库文档
rustup doc --book          # 打开 Rust Book

# ═══════════════════════════════════════════════════════════
# 组件管理
# ═══════════════════════════════════════════════════════════
rustup component list      # 查看可用组件
rustup component add rustfmt  # 添加格式化工具
rustup component add clippy   # 添加代码检查工具
```

### 术语表

| 术语 | 英文 | 解释 |
|------|------|------|
| 工具链 | Toolchain | 编译器、库、工具的集合 |
| Crate | Crate | Rust 的最小编译单元（包） |
| Package | Package | 一个或多个 crate 组成的项目 |
| Cargo | Cargo | Rust 的包管理器和构建工具 |
| rustup | rustup | Rust 的工具链管理器 |
| Edition | Edition | Rust 的版本迭代（2015/2018/2021/2024） |

---

## 下一章

[第 02 章：第一个 Rust 程序](../02-first-program/README.md) - 深入理解 Hello World 程序，掌握 Rust 基本语法结构。