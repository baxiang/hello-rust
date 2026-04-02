# 教程发布流程

> Markdown 教程的发布和维护流程

---

## 发布准备

### 1. 内容完整性检查

**检查清单：**

- [ ] 所有章节内容完整
- [ ] 练习题已创建
- [ ] 代码示例验证通过
- [ ] 链接有效性检查
- [ ] 格式一致性检查

### 2. 质量审核

**审核要点：**

- [ ] 代码正确性
- [ ] 概念解释准确
- [ ] 语言流畅清晰
- [ ] 符合风格指南

### 3. 版本标注

更新 README.md 和 AGENTS.md 中的版本信息：

```markdown
- 教程版本：X.X
- 更新日期：YYYY-MM-DD
- Rust 版本：1.XX+
```

---

## 发布渠道

### 1. GitHub Pages

将教程发布为静态网站：

**配置步骤：**

1. 在仓库 Settings 中启用 GitHub Pages
2. 选择 Source: `main` 分支，`/ (root)` 目录
3. 创建 `.nojekyll` 文件（禁用 Jekyll）
4. 等待构建完成

**访问地址：**

```
https://your-username.github.io/hello-rust/
```

---

### 2. GitBook

使用 GitBook 平台发布：

**步骤：**

1. 在 GitBook 创建空间
2. 连接 GitHub 仓库
3. 配置同步设置
4. 自动生成网站

**优势：**
- 专业文档界面
- 搜索功能
- 版本管理

---

### 3. 独立网站

使用静态站点生成器：

**推荐工具：**

- **mdBook：** Rust 官方文档工具
  ```bash
  cargo install mdbook
  mdbook build
  ```

- **Hugo：** 快速静态站点生成器
- **Docsify：** 无需构建的文档网站

**mdBook 配置示例：**

```toml
# book.toml
[book]
title = "Rust 编程语言中文教程"
authors = ["作者"]
language = "zh"

[build]
build-dir = "book"

[output.html]
git-repository-url = "https://github.com/your-repo/hello-rust"
```

---

## 版本管理

### 版本号规则

使用语义化版本（Semantic Versioning）：

```
MAJOR.MINOR.PATCH

示例：
1.0.0  # 首次正式发布
1.1.0  # 新增章节
1.0.1  # 修复错误
2.0.0  # 大规模重写
```

**版本变化规则：**

- **MAJOR：** 大规模改动，结构调整
- **MINOR：** 新增章节，内容扩充
- **PATCH：** 错误修复，格式调整

---

### 发布标签

使用 Git 标签标记版本：

```bash
# 创建标签
git tag -a v1.0.0 -m "首次正式发布"

# 推送标签
git push origin v1.0.0

# 查看所有标签
git tag -l
```

---

### GitHub Release

创建 GitHub Release：

1. 在 GitHub 创建新 Release
2. 选择标签
3. 编写发布说明
4. 附上重要更新内容

**发布说明模板：**

```markdown
# 版本 X.X.X

## 新增内容

- 新增第 X 章
- 新增练习题

## 修复问题

- 修复第 Y 章错误
- 更正代码示例

## 改进

- 优化格式
- 更新依赖

---

**完整更新日志：** 查看 [CHANGELOG.md](./CHANGELOG.md)
```

---

## 持续维护

### 更新流程

**常规更新：**

1. 创建 Issue 讨论更新内容
2. Fork 并修改内容
3. 提交 Pull Request
4. 审核合并
5. 发布新版本

**紧急修复：**

1. 直接在 main 分支修复
2. 发布 PATCH 版本
3. 记录在 CHANGELOG

---

### CHANGELOG 维护

维护更新日志文件：

```markdown
# CHANGELOG

## [1.1.0] - 2026-04-03

### 新增
- 新增第 29-31 章内容
- 创建练习题框架

### 修复
- 修复第 7 章代码示例错误

### 改进
- 优化可视化图解

## [1.0.0] - 2026-03-01

### 新增
- 完成所有基础章节
- 创建项目实战部分
```

---

## 传播推广

### 社区分享

**分享渠道：**

1. **Rust 中文社区**
   - Rust.cc 论坛
   - Rust 中文微信群

2. **技术平台**
   - 掘金
   - 知乎
   - CSDN

3. **社交媒体**
   - 微博
   - Twitter

---

### 收集反馈

**反馈渠道：**

1. GitHub Issues
2. GitHub Discussions
3. 社群讨论
4. 用户邮件

**反馈处理：**

- 及时回复 Issue
- 分类整理反馈
- 定期改进内容

---

## 发布检查清单

发布前必须完成：

- [ ] 所有测试通过
- [ ] CI/CD 流程成功
- [ ] 内容审核完成
- [ ] 版本号更新
- [ ] CHANGELOG 更新
- [ ] Git 标签创建
- [ ] GitHub Release 创建
- [ ] 发布说明编写
- [ ] 社区分享通知

---

## 相关工具

### 自动化工具

- **GitHub Actions：** 自动化发布流程
- **mdBook：** 自动生成文档网站
- **markdown-link-check：** 链接检查

### 分析工具

- **Google Analytics：** 访问统计
- **GitHub Insights：** 仓库分析

---

## 常见问题

### Q: 如何处理版本兼容性？

**A:** 在 README 中标注最低 Rust 版本，并说明版本差异。

### Q: 如何回滚到旧版本？

**A:** 使用 Git 标签切换到旧版本：

```bash
git checkout v1.0.0
```

### Q: 如何同时维护多个版本？

**A:** 使用 Git 分支：

```bash
git branch v1.x-maintenance
```

---

## 相关文档

- [CONTRIBUTING.md](../CONTRIBUTING.md) - 贡献指南
- [CI/CD 配置](../.github/workflows/ci.yml) - 自动化流程
- [GitHub Release 指南](https://docs.github.com/en/repositories/releasing-projects-on-github)

---

**更新日志：**
- 2026-04-03: 创建发布流程文档