## 21.5 常用 Cargo 命令

### 构建命令

```bash
# 调试构建（默认）
cargo build
# 输出：target/debug/

# 发布构建（优化）
cargo build --release
# 输出：target/release/
# 优化级别：-O3

# 快速检查（不生成二进制）
cargo check
# 用于快速验证代码

# 清理构建文件
cargo clean

# 更新依赖
cargo update
cargo update -p serde  # 更新特定包
```

### 运行命令

```bash
# 运行程序
cargo run
cargo run --release

# 传递参数
cargo run -- arg1 arg2

# 运行特定二进制
cargo run --bin my_tool
```

### 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行测试（显示输出）
cargo test -- --nocapture

# 单线程运行
cargo test -- --test-threads=1

# 只运行文档测试
cargo test --doc

# 只运行单元测试
cargo test --lib
```

### 代码质量

```bash
# 格式化代码
cargo fmt
cargo fmt -- --check  # 检查格式

# 代码 lint
cargo clippy
cargo clippy -- -D warnings  # 警告当错误

# 生成文档
cargo doc
cargo doc --open      # 生成并打开
cargo doc --no-deps   # 不包含依赖
```

### 依赖管理

```bash
# 查看依赖树
cargo tree
cargo tree -d         # 查看重复依赖
cargo tree -i serde   # 查看谁依赖 serde

# 验证
cargo verify-project

# 查看元数据
cargo metadata
```

---

---

## 下一步

[常用 Crates 推荐](../6-常用 Crates 推荐.md)