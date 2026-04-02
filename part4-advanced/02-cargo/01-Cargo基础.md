## 21.1 Cargo 是什么

```
┌─────────────────────────────────────────────────────┐
│              Cargo 能做什么                          │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📦 依赖管理     - 自动下载和编译依赖               │
│  🔨 构建项目     - 编译、链接、生成二进制           │
│  🧪 运行测试     - 执行单元测试和集成测试           │
│  📚 生成文档     - 自动生成 API 文档                 │
│  🚀 发布 crate   - 发布到 crates.io                 │
│  ✅ 代码检查     - 格式化、lint                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Cargo vs 其他包管理器

| 工具 | 语言 | 特点 |
|------|------|------|
| Cargo | Rust | 内置、快速、可靠 |
| npm | JavaScript | 生态最大、依赖多 |
| pip | Python | 简单易用 |
| Maven | Java | 配置复杂 |

---

---

## 下一步

[创建项目](../2-创建项目.md)

---

## 21.2 创建项目

### 新建二进制项目

```bash
# 创建可执行程序
cargo new my_app
cd my_app

# 项目结构
my_app/
├── Cargo.toml      # 项目清单
├── src/
│   └── main.rs     # 程序入口
└── .git/
```

### 新建库项目

```bash
# 创建库
cargo new my_lib --lib
cd my_lib

# 项目结构
my_lib/
├── Cargo.toml      # 项目清单
├── src/
│   └── lib.rs      # 库入口
└── .git/
```

### 在现有目录初始化

```bash
# 已有代码，初始化 Cargo
cd existing_project
cargo init

# 指定类型
cargo init --lib     # 初始化为库
cargo init --bin     # 初始化为二进制
```

---

---

## 下一步

[Cargo.toml 详解](../3-Cargo.toml 详解.md)

---

## 返回

[返回目录](../../README.md)