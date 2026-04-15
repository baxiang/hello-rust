# 本地实验项目规范 - 设计文档

> 日期：2026-04-15
> 状态：已确认

## 概述

为教程的每一章创建本地 Cargo 项目，使用 `examples/` 目录组织所有示例代码，读者可以在本地 VSCode 中逐章编译运行。

## 目录结构

```
<章节目录>/
├── README.md                    # 章节导航
├── <小节文件>.md                # 理论内容
├── Cargo.toml                   # 章节项目配置
└── examples/
    ├── 01-<示例名称>.rs         # 对应第一小节
    ├── 02-<示例名称>.rs         # 对应第二小节
    └── ...
```

### 示例

```
1-basics/03-variables/
├── README.md
├── 01-变量基础.md
├── 02-常量与静态.md
├── 03-高级特性.md
├── 04-实战总结.md
├── Cargo.toml
└── examples/
    ├── 01-let-binding.rs
    ├── 02-mut-variable.rs
    ├── 03-shadowing.rs
    └── 04-chapter-review.rs
```

## Cargo.toml 模板

```toml
[package]
name = "chapter-<序号>-<主题>"
version = "0.1.0"
edition = "2024"

[dependencies]
# 按需添加依赖
```

### 命名规则

- `name` 格式：`chapter-<两位序号>-<主题>`，如 `chapter-03-variables`
- `edition`：统一使用 `"2024"`
- 基础章节无外部依赖，高级章节按需添加

## 示例文件规范

### 文件命名

- 格式：`<两位序号>-<示例名称>.rs`
- 示例名称使用 kebab-case（短横线命名）
- 序号与章节小节对应

### 文件内容模板

```rust
//! # 示例：<示例标题>
//! 
//! 对应章节：<章节名称>
//! 运行：cargo run --example <文件名（不含.rs）>

fn main() {
    // ✅ 正确示例
    // 代码实现
    
    // 尝试修改：
    // 取消下面的注释观察编译错误：
    // xxx;  // ❌ 错误示例说明
}
```

### 要求

1. 文件顶部必须有模块文档注释（`//!`）
2. 包含对应章节的 Markdown 说明
3. 正确示例使用 `// ✅` 标注
4. 错误示例注释掉并标注 `// ❌`
5. 鼓励读者修改代码实验

## 章节 Markdown 更新

在每章 README.md 中添加"本地实验"段落：

```markdown
## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
# 运行单个示例
cargo run --example 01-let-binding

# 编译检查所有示例
cargo check --examples
```
```

## 验证脚本更新

`scripts/verify-code.sh` 更新为：

```bash
# 遍历所有章节目录
for chapter_dir in */*/; do
    if [ -f "$chapter_dir/Cargo.toml" ]; then
        cd "$chapter_dir"
        echo "检查章节：$chapter_dir"
        cargo check --examples
        cd - > /dev/null
    fi
done
```

## 实施计划

1. 更新 `docs/style-guide.md` 添加"本地实验项目规范"章节
2. 为 `1-basics/03-variables/` 创建模板：
   - 创建 `Cargo.toml`
   - 创建 `examples/` 目录
   - 创建示例文件
3. 更新 `scripts/verify-code.sh`
4. 更新 `1-basics/03-variables/README.md` 添加本地实验说明

## 版本要求

- 统一使用 Rust 1.85+
- 统一使用 2024 Edition
