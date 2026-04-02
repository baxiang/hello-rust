# WASM 基础

> 理解 WebAssembly 的核心概念和 Rust 编译到 WASM 的方法。

## 什么是 WebAssembly？

WebAssembly（WASM）是一种新的二进制指令格式，可以在现代 Web 浏览器中高效运行。

### 核心特点

```
┌──────────────────────────────────────────┐
│        WebAssembly 架构                    │
├──────────────────────────────────────────┤
│                                          │
│  ┌────────────────────────────────────┐ │
│  │  多语言支持                          │ │
│  │  ┌────────┬────────┬────────────┐ │ │
│  │  │ Rust   │ C/C++  │ Assembly   │ │ │
│  │  └────────┴────────┴────────────┘ │ │
│  └────────────────────────────────────┘ │
│                                          │
│  ┌────────────────────────────────────┐ │
│  │  WASM 二进制格式 (.wasm)            │ │
│  │  - 紧凑的二进制编码                  │ │
│  │  - 快速解析和加载                    │ │
│  │  - 平台无关                          │ │
│  └────────────────────────────────────┘ │
│                                          │
│  ┌────────────────────────────────────┐ │
│  │  浏览器运行环境                      │ │
│  │  - 与 JavaScript 并行运行            │ │
│  │  - 共享内存（SharedArrayBuffer）     │ │
│  │  - 调用 JavaScript API               │ │
│  └────────────────────────────────────┘ │
│                                          │
└──────────────────────────────────────────┘
```

### 性能优势

| 特性 | JavaScript | WebAssembly |
|------|-----------|-------------|
| 解析速度 | 慢（文本解析） | 快（二进制解码） |
| 编译速度 | JIT 编译耗时 | 预编译优化 |
| 执行速度 | 动态类型开销 | 接近原生性能 |
| 内存效率 | 灵活但开销大 | 紧凑线性内存 |
| 启动时间 | 较快 | 极快 |

### 应用场景

```
✅ 适合 WebAssembly 的场景：
- 图像/视频处理
- 游戏、3D 渲染
- 音频处理
- 加密计算
- 科学计算
- 性能密集型算法

❌ 不适合 WebAssembly 的场景：
- 简单 DOM 操作
- 事件处理
- UI 逻辑
- 网络请求
```

## Rust 编译到 WASM

### 安装工具

```bash
# 1. 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 添加 WASM 目标
rustup target add wasm32-unknown-unknown

# 3. 安装 wasm-pack（推荐）
cargo install wasm-pack

# 4. 安装 cargo-generate（可选，用于模板）
cargo install cargo-generate
```

### 创建 WASM 项目

#### 方法 1：使用 wasm-pack 模板（推荐）

```bash
# 创建项目
cargo generate --git https://github.com/rustwasm/wasm-pack-template

# 或手动创建
cargo new --lib my-wasm-project
```

#### 方法 2：手动配置

```toml
# Cargo.toml
[package]
name = "my-wasm-project"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"

[dev-dependencies]
wasm-bindgen-test = "0.2"

[profile.release]
opt-level = 3
lto = true
```

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 编译 WASM

```bash
# 开发模式（快速编译）
wasm-pack build

# 生产模式（优化）
wasm-pack build --release

# 生成 npm 包
wasm-pack build --target web
wasm-pack build --target bundler
wasm-pack build --target nodejs
```

**输出文件：**

```
pkg/
├── my_wasm_project.js       # JavaScript 绑定
├── my_wasm_project_bg.js    # 底层 WASM 加载
├── my_wasm_project_bg.wasm  # WASM 二进制
├── my_wasm_project.d.ts     # TypeScript 类型定义
└── package.json             # npm 包配置
```

## wasm-pack 工具

### 构建目标

| 目标 | 输出格式 | 适用场景 |
|------|---------|---------|
| `web` | ES module | 直接浏览器加载 |
| `bundler` | CommonJS/ES module | webpack、rollup |
| `nodejs` | CommonJS | Node.js 环境 |

### 常用命令

```bash
# 构建项目
wasm-pack build

# 构建 + 发布到 npm
wasm-pack build --release
wasm-pack publish

# 测试 WASM
wasm-pack test --headless --firefox

# 创建文档
wasm-pack build --doc
```

### 配置文件

```json
// package.json（由 wasm-pack 生成）
{
  "name": "my-wasm-project",
  "version": "0.1.0",
  "files": [
    "my_wasm_project_bg.wasm",
    "my_wasm_project.js",
    "my_wasm_project_bg.js",
    "my_wasm_project.d.ts"
  ],
  "module": "my_wasm_project.js",
  "types": "my_wasm_project.d.ts",
  "sideEffects": false
}
```

## WASM 项目结构

### 基本结构

```
my-wasm-project/
├── Cargo.toml
├── src/
│   ├── lib.rs           # WASM 导出函数
│   └── utils.rs         # 辅助函数
├── pkg/                 # wasm-pack 输出
│   ├── my_wasm_project_bg.wasm
│   ├── my_wasm_project.js
│   └── package.json
├── tests/
│   └── web.rs           # WASM 测试
└── www/                 # Web 应用（可选）
    ├── package.json
    ├── webpack.config.js
    ├── index.html
    ├── index.js
    └── bootstrap.js
```

### 项目示例

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

// 导出函数到 JavaScript
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// 导出结构体
#[wasm_bindgen]
pub struct Calculator {
    value: i32,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        Calculator { value: 0 }
    }
    
    pub fn add(&mut self, x: i32) {
        self.value += x;
    }
    
    pub fn get(&self) -> i32 {
        self.value
    }
}
```

## 第一个 WASM 应用

### 1. 创建项目

```bash
# 创建 Rust 库项目
cargo new --lib hello-wasm
cd hello-wasm
```

### 2. 配置项目

```toml
# Cargo.toml
[package]
name = "hello-wasm"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

[profile.release]
opt-level = 3
lto = true
```

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // 导入 JavaScript 函数
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello from WASM, {}!", name));
}

#[wasm_bindgen]
pub fn compute(x: i32) -> i32 {
    x * x + 2 * x + 1
}
```

### 3. 构建 WASM

```bash
# 构建
wasm-pack build --target web
```

### 4. 创建 Web 页面

```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Hello WASM</title>
</head>
<body>
    <script type="module">
        import init, { greet, compute } from './pkg/hello_wasm.js';
        
        async function run() {
            await init();
            
            greet('World');
            console.log('compute(5) =', compute(5));
        }
        
        run();
    </script>
</body>
</html>
```

### 5. 运行测试

```bash
# 使用本地服务器
python3 -m http.server 8080

# 或使用 npm
npm install -g serve
serve .

# 打开浏览器访问 http://localhost:8080
```

## WASM 调试

### console 日志

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn debug_example() {
    log("This is a log message");
    error("This is an error message");
}
```

### panic 处理

```rust
use wasm_bindgen::prelude::*;
use console_error_panic_hook;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// Cargo.toml
[dependencies]
console_error_panic_hook = "0.1"
```

### WASM 测试

```rust
// tests/web.rs
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[wasm_bindgen_test]
fn test_greet() {
    let result = greet("World");
    assert!(result.contains("Hello"));
}
```

```bash
# 运行测试
wasm-pack test --headless --firefox
```

## WASM 性能

### 性能对比

```html
<script>
// JavaScript 版本
function fibJS(n) {
    if (n <= 1) return n;
    return fibJS(n - 1) + fibJS(n - 2);
}

// WASM 版本
import init, { fibonacci as fibWASM } from './pkg/my_wasm.js';

async function benchmark() {
    await init();
    
    const n = 40;
    
    // JavaScript 测试
    const startJS = performance.now();
    const resultJS = fibJS(n);
    const timeJS = performance.now() - startJS;
    
    // WASM 测试
    const startWASM = performance.now();
    const resultWASM = fibWASM(n);
    const timeWASM = performance.now() - startWASM;
    
    console.log(`JavaScript: ${timeJS}ms`);
    console.log(`WASM: ${timeWASM}ms`);
    console.log(`Speedup: ${timeJS / timeWASM}x`);
}
</script>
```

**典型结果：**

```
JavaScript: 1200ms
WASM: 15ms
Speedup: 80x
```

### 优化建议

```rust
// ✅ 避免频繁跨边界调用
#[wasm_bindgen]
pub fn process_batch(data: Vec<i32>) -> Vec<i32> {
    // 批量处理，减少 JavaScript ↔ WASM 调用
    data.iter().map(|x| x * 2).collect()
}

// ❌ 频繁跨边界调用
#[wasm_bindgen]
pub fn process_item(item: i32) -> i32 {
    item * 2  // 每次调用都跨边界
}

// ✅ 使用引用传递大对象
#[wasm_bindgen]
pub fn process_image(image: &[u8]) -> Vec<u8> {
    // 传递切片，避免复制
}

// ✅ 预分配内存
#[wasm_bindgen]
pub fn compute() -> Vec<i32> {
    let mut result = Vec::with_capacity(1000);
    // ...
    result
}
```

## WASM 限制

### 当前限制

| 限制 | 说明 | 解决方案 |
|------|------|---------|
| 无法直接访问 DOM | 需要 JavaScript 中介 | wasm-bindgen |
| 无标准库支持 | 部分功能不可用 | wasm-bindgen-futures |
| 单线程 | WASM 主线程单线程 | Web Workers |
| 文件系统访问 | 受限 | 虚拟文件系统 |

### 解决方案

```rust
// DOM 操作：通过 JavaScript
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = document)]
    pub fn getElementById(id: &str) -> Element;
}

// 异步：使用 wasm-bindgen-futures
use wasm_bindgen_futures::JsFuture;

async fn fetch_data() -> Result<String, JsValue> {
    let promise = js_sys::fetch("data.json");
    let response = JsFuture::from(promise).await?;
    // ...
}

// 多线程：使用 Web Workers
// 需要特殊配置和 SharedArrayBuffer
```

## 小结

**WebAssembly 核心概念：**

- WASM 是高性能二进制格式
- Rust 可以编译到 WASM
- wasm-pack 简化构建流程
- wasm-bindgen 实现 JavaScript 互操作

**关键工具：**

- `wasm-pack`: 构建、打包、发布
- `wasm-bindgen`: JavaScript 绑定
- `wasm-bindgen-test`: WASM 测试

**应用场景：**

- 性能密集型计算
- 图像/视频处理
- 游戏、音频处理

**下一步：**
下一节我们将深入学习 wasm-bindgen 的 JavaScript 互操作功能。

## 练习

### 练习 1：第一个 WASM 函数

创建并部署一个简单的 WASM 函数：

```rust
// TODO: 实现一个计算平方的函数
// TODO: 编译到 WASM
// TODO: 在浏览器中调用并测试
```

### 练习 2：性能对比

对比 WASM 和 JavaScript 的性能：

```html
// TODO: 实现同一算法的 JS 和 WASM 版本
// TODO: 测量并对比执行时间
// TODO: 分析性能差异的原因
```

### 练习 3：调试 WASM

使用 console.log 调试 WASM：

```rust
// TODO: 导入 console.log
// TODO: 在 WASM 函数中打印调试信息
// TODO: 在浏览器中查看输出
```