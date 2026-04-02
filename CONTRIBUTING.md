# 贡献指南

感谢你对 Rust 中文教程的关注！欢迎任何形式的贡献。

---

## 贡献方式

### 1. 报告问题

发现教程中的错误或不清楚的地方：

- 在 [GitHub Issues](https://github.com/your-repo/hello-rust/issues) 提交问题
- 描述具体章节和问题描述
- 提供改进建议

**Issue 模板：**

```markdown
**章节：** 第 X 章 - 章节名
**问题描述：** [详细描述]
**建议改进：** [可选]
```

---

### 2. 提交练习题答案

为练习题提供你的解答：

1. Fork 本仓库
2. 在 `solutions/用户名/章节名/` 创建解答文件
3. 提交 Pull Request

**答案格式：**

```rust
// 文件：solutions/yourname/01-rust-intro/01-solution.rs

// 练习 X 解答

fn main() {
    // 你的代码
}
```

---

### 3. 完善章节内容

为未完成的章节添加内容：

1. 选择标记为 `🚧 待完善` 的章节
2. Fork 并创建分支
3. 添加内容并遵循风格指南
4. 提交 Pull Request

**需要完善的章节：**
- [ ] 练习题（任务11）
- [ ] Playground 示例（任务12）
- [ ] 可视化图解（任务13）
- [ ] 项目测试策略（任务15）

---

### 4. 添加新内容

提议添加新的章节或内容：

- 先在 Issues 中讨论
- 获得同意后提交 Pull Request
- 遵循现有格式和风格

---

## 开发流程

### 1. Fork 和克隆

```bash
# Fork 仓库到你的账户
# 克隆你的 fork
git clone https://github.com/your-username/hello-rust.git
cd hello-rust

# 添加上游仓库
git remote add upstream https://github.com/original-owner/hello-rust.git
```

### 2. 创建分支

```bash
# 创建特性分支
git checkout -b feature/your-feature-name

# 或修复分支
git checkout -b fix/issue-number
```

### 3. 开发和测试

```bash
# 编辑文件
vim path/to/file.md

# 运行验证脚本（如果有）
./scripts/verify-code.sh
./scripts/check-links.sh

# 本地预览（使用 Markdown 渲染工具）
```

### 4. 提交更改

```bash
# 添加更改
git add path/to/file.md

# 提交
git commit -m "描述你的更改"

# 推送到你的 fork
git push origin feature/your-feature-name
```

### 5. 创建 Pull Request

- 在 GitHub 上创建 Pull Request
- 描述更改内容和原因
- 等待审核和合并

---

## 文档风格指南

详见 [docs/style-guide.md](./docs/style-guide.md)

### 核心原则

1. **使用简洁清晰的语言**
2. **提供代码示例**
3. **标注正确/错误示例**
4. **使用可视化图解**
5. **包含练习题**

---

## Markdown 格式规范

### 标题层级

```markdown
# 章节标题（一级）
> 章节简介

## 主要节（二级）
### 子节（三级）
```

### 代码块

```markdown
```rust
// ✅ 正确示例
fn valid_code() { }

// ❌ 错误示例
fn invalid_code() { }
```
```

### 可视化图解

使用 ASCII 艺术：

```markdown
┌─────────────────┐
│  内存布局图     │
└─────────────────┘
```

### 练习题格式

```markdown
### 练习 X：题目名

**题目：** [描述]

```rust
// 代码示例
```

<details>
<summary>查看答案</summary>

[答案与解释]

</details>
```

---

## 代码示例规范

### Rust 代码风格

遵循 [AGENTS.md](./AGENTS.md) 中的代码风格指南：

1. 使用 `cargo fmt` 格式化
2. 使用 `cargo clippy` 检查
3. 提供类型标注
4. 处理错误而非 panic

### 示例结构

```rust
// ✅ 正确示例
fn example() {
    // 代码说明
    
    // 实现
}

// ❌ 错误示例（附带错误信息）
fn bad_example() {
    // 错误代码
    // 错误信息：[编译错误]
}
```

---

## 审核流程

### Pull Request 审核

1. **自动检查**
   - 代码格式检查
   - 链接有效性检查
   - CI/CD 流程

2. **人工审核**
   - 内容准确性
   - 格式一致性
   - 教学价值

3. **反馈和修改**
   - 审核者提出建议
   - 作者进行修改
   - 再次审核

4. **合并**
   - 通过所有检查
   - 审核者批准
   - 合并到主分支

---

## 社区准则

### 行为准则

- 尊重所有贡献者
- 使用建设性语言
- 接受不同观点
- 专注于教程改进

### 沟通渠道

- GitHub Issues：问题和建议
- Pull Requests：代码贡献
- Discussions：一般讨论

---

## 许可证

本项目采用 MIT 许可证。贡献的内容将采用相同许可证。

---

## 致谢

感谢所有贡献者的付出！

---

## 常见问题

### Q: 如何开始贡献？

**A:** 从提交 Issue 或完善练习题开始。

### Q: 我的 Pull Request 没被合并？

**A:** 查看审核意见，按要求修改。

### Q: 我不懂 Rust，能贡献吗？

**A:** 可以！帮助检查格式、链接、翻译等。

### Q: 如何成为维护者？

**A:** 持续高质量贡献，会被邀请加入。

---

## 相关文档

- [AGENTS.md](./AGENTS.md) - AI 编码代理指南
- [README.md](./README.md) - 教程总览
- [docs/style-guide.md](./docs/style-guide.md) - 详细风格指南

---

**联系方式：**
- GitHub Issues: [提交问题](https://github.com/your-repo/hello-rust/issues)
- Email: [联系维护者]

---

**更新日志：**
- 2026-04-03: 创建贡献指南