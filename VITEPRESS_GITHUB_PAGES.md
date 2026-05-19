# 使用 VitePress 与 GitHub Pages 构建技术文档站点

> 从零开始搭建现代化文档站点，实现自动化构建与部署

## 概述

VitePress 是 VuePress 的继任者，基于 Vite 构建引擎，提供极快的开发体验和出色的生产性能。结合 GitHub Pages 的免费静态托管，可以快速搭建一个功能完善的技术文档站点。

### 技术选型

| 工具 | 用途 | 优势 |
|------|------|------|
| VitePress | 静态站点生成器 | 基于 Vite，开发热更新快，生产打包小 |
| GitHub Pages | 静态资源托管 | 免费、CDN 加速、与 GitHub 深度集成 |
| GitHub Actions | CI/CD 自动化 | 推送即部署，无需手动操作 |
| vitepress-sidebar | 自动生成侧边栏 | 根据文件结构自动组织导航 |

## 项目结构

```
hello-rust/
├── .vitepress/
│   ├── config.ts          # VitePress 配置文件
│   ├── sidebar.ts         # 侧边栏自动生成配置
│   └── theme/
│       ├── index.ts       # 自定义主题入口
│       ├── custom.css     # 自定义样式
│       └── playground.ts  # 自定义 Markdown 插件
├── 1-basics/              # 第一部分：基础入门
│   └── 01-intro/
│       ├── index.md       # 章节入口文件
│       ├── 01-why-rust.md # 子章节内容
│       └── examples/      # Rust 示例代码（Cargo 项目）
├── index.md               # 首页
├── package.json           # Node.js 依赖管理
└── .github/workflows/
    └── deploy.yml         # GitHub Actions 部署配置
```

## 环境搭建

### 1. 安装 Node.js

推荐使用 Node.js 20 LTS 或更高版本：

```bash
# 使用 nvm 安装（macOS/Linux）
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
nvm install 20

# 使用 Homebrew 安装（macOS）
brew install node@20
```

验证安装：

```bash
node --version   # v20.x.x
npm --version    # 10.x.x
```

### 2. 初始化项目

```bash
# 创建项目目录
mkdir hello-rust && cd hello-rust

# 初始化 npm 项目
npm init -y

# 安装 VitePress 及插件
npm install -D vitepress vitepress-sidebar vitepress-plugin-group-icons
```

### 3. 配置 package.json

```json
{
  "name": "hello-rust-site",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "docs:dev": "vitepress dev .",
    "docs:build": "vitepress build .",
    "docs:preview": "vitepress preview ."
  },
  "devDependencies": {
    "vitepress": "^1.6.0",
    "vitepress-sidebar": "^1.31.0",
    "vitepress-plugin-group-icons": "^1.5.0"
  }
}
```

启动开发服务器：

```bash
npm run docs:dev
# 访问 http://localhost:5173
```

## 核心配置

### VitePress 配置文件

`.vitepress/config.ts` 是站点的核心配置：

```typescript
import { defineConfig } from 'vitepress'
import sidebar from './sidebar'
import { playgroundPlugin } from './theme/playground'

const isProduction = process.env.NODE_ENV === 'production' || process.argv.includes('build')

export default defineConfig({
  title: '从零开始系统学习 Rust 编程',
  description: '从零开始系统学习 Rust 编程语言',
  lang: 'zh-CN',
  base: isProduction ? '/hello-rust/' : '/',
  cleanUrls: true,

  themeConfig: {
    nav: [
      { text: '首页', link: '/' },
      { text: '基础入门', link: '/1-basics/01-intro/' },
    ],

    sidebar,

    search: {
      provider: 'local',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/baxiang/hello-rust' },
    ],
  },

  markdown: {
    lineNumbers: true,
    config: (md) => {
      md.use(playgroundPlugin)
    },
  },

  srcExclude: [
    '**/examples/**',
    '**/target/**',
    '**/Cargo.toml',
    '**/Cargo.lock',
    '**/node_modules/**',
    '**/.vitepress/**',
  ],
})
```

#### 关键配置说明

| 配置项 | 说明 | 注意事项 |
|--------|------|----------|
| `base` | 站点基础路径 | GitHub Pages 部署时必须设置为 `/仓库名/` |
| `cleanUrls` | 启用干净 URL | 去除 `.html` 后缀 |
| `srcExclude` | 排除扫描的文件 | 避免将 Cargo 项目、node_modules 等纳入构建 |
| `ignoreDeadLinks` | 忽略死链检测 | 处理短路径文件引用问题 |

### 自动侧边栏配置

使用 `vitepress-sidebar` 插件根据文件结构自动生成侧边栏：

```typescript
import { generateSidebar } from 'vitepress-sidebar'

export default generateSidebar([
  {
    documentRootPath: '/1-basics',
    scanStartPath: '',
    resolvePath: '/1-basics/',
    useTitleFromFileHeading: true,    // 使用 markdown 的 # 标题作为菜单名
    collapsed: true,                  // 默认折叠
    sortMenusOrderByDescending: false,
    excludeFiles: ['Cargo.toml'],
    excludeFolders: ['examples', 'target', 'src'],
  },
  // ... 其他部分配置
])
```

### 自定义主题与样式

`.vitepress/theme/index.ts` 引入自定义样式：

```typescript
import DefaultTheme from 'vitepress/theme'
import './custom.css'

export default DefaultTheme
```

自定义样式示例：

```css
/* Hero 首页左对齐布局 */
.VPHero .container {
  max-width: 1152px;
}

.VPHero .main {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-align: left;
}

.VPHero .name {
  white-space: nowrap;
  font-size: clamp(1.4rem, 3.5vw, 2.5rem);
}
```

## 首页设计

使用 VitePress 的 `home` 布局创建首页：

```yaml
---
layout: home

hero:
  name: "从零开始系统学习 Rust 编程"
  tagline: 编译器是免费导师，错误信息是学习线索而非障碍
  actions:
    - theme: brand
      text: 开始学习
      link: /1-basics/01-intro/
    - theme: alt
      text: 项目实战
      link: /5-projects/

features:
  - title: 分层递进
    details: 先熟练基础语法，再攻克所有权，最后进阶实战
  - title: 场景驱动
    details: 每个概念从"为什么需要"开始，用实际问题引入
  - title: 编译器当导师
    details: 教会读懂错误信息，编译器是最好的老师
  - title: 代码可运行
    details: 每章配有 Cargo 项目，cargo run --example 即可运行
---
```

### Hero 区域字段说明

| 字段 | 说明 | 示例 |
|------|------|------|
| `name` | 站点名称 | 显示在最顶部，字体较大 |
| `tagline` | 描述标语 | 在名称下方的辅助文字 |
| `actions` | 操作按钮 | 支持 `brand`（主色）和 `alt`（次色）两种主题 |

### Features 区域

每个 feature 对象包含：
- `title`: 标题
- `details`: 详细描述
- `icon`（可选）: 自定义图标

## 章节内容规范

每个章节是一个独立的目录，包含 `index.md` 作为入口：

```markdown
# 第 01 章：简介与环境搭建

> **本章代码基于 Rust 2024 Edition (Rust 1.85+) 编写**

> 开启 Rust 学习之旅，搭建开发环境，了解 Rust 的优势与应用场景。

## 本章目标

完成本章学习后，你将掌握：
- 理解 Rust 的核心优势与应用场景
- 成功安装 Rust 工具链

## 章节内容

### 为什么学习 Rust
- [为什么学习 Rust](./01-why-rust) - Rust 核心价值、解决的问题

### 安装与配置
- [安装与配置](./02-installation) - 跨平台安装、工具链

## 预计学习时间

- 基础学习：1 小时
- 练习巩固：0.5 小时
- 总计：1.5 小时

## 常见问题

- 如何选择合适的 IDE？
- rustup 安装慢怎么办？

---

## 本地实验

本章示例代码位于 `examples/` 目录。

```bash
# 运行单个示例
cargo run --example 01-why-rust
cargo run --example 02-install-check
```
```

## GitHub Pages 部署

### 1. 创建 GitHub 仓库

```bash
# 初始化 git 仓库
git init
git add .
git commit -m "initial commit"

# 添加远程仓库
git remote add origin git@github.com:username/hello-rust.git
git push -u origin main
```

### 2. 配置 GitHub Actions

创建 `.github/workflows/deploy.yml`：

```yaml
name: Deploy VitePress site to Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: false

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm

      - name: Install dependencies
        run: npm ci

      - name: Build
        run: npm run docs:build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: .vitepress/dist

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

#### 工作流配置说明

| 配置项 | 说明 |
|--------|------|
| `on.push.branches` | 推送到 main 分支时触发部署 |
| `on.workflow_dispatch` | 允许手动触发 |
| `permissions` | 设置 GitHub Pages 部署权限 |
| `concurrency` | 确保同一时间只有一个部署任务 |
| `cache: npm` | 缓存 node_modules 加速构建 |

### 3. 启用 GitHub Pages

1. 进入仓库 **Settings** → **Pages**
2. 在 **Source** 中选择 **GitHub Actions**
3. 保存后，下次推送 main 分支将自动部署

### 4. 配置 base 路径

确保 `.vitepress/config.ts` 中的 `base` 设置为正确的仓库路径：

```typescript
base: '/hello-rust/'  // 替换为你的仓库名
```

## 高级功能

### 自定义 Markdown 插件

可以创建自定义 Markdown 插件来扩展功能：

```typescript
// .vitepress/theme/playground.ts
export const playgroundPlugin = (md: any) => {
  // 自定义 Markdown 规则
  // 例如：添加 Rust Playground 按钮
}
```

### 本地搜索

使用 VitePress 内置的本地搜索：

```typescript
search: {
  provider: 'local',
}
```

### 代码块行号

```typescript
markdown: {
  lineNumbers: true,
}
```

### 多语言支持

```typescript
export default defineConfig({
  lang: 'zh-CN',
  themeConfig: {
    lastUpdatedText: '最后更新',
    docFooter: {
      prev: '上一页',
      next: '下一页',
    },
  },
})
```

## 构建与部署命令速查

| 命令 | 说明 |
|------|------|
| `npm run docs:dev` | 启动本地开发服务器，支持热更新 |
| `npm run docs:build` | 构建生产版本，输出到 `.vitepress/dist` |
| `npm run docs:preview` | 预览生产构建结果 |

---

## 常见构建错误与排查指南

### 1. `<T>` 或 `<model>` 被解析为未闭合 HTML 标签
**现象**：`Build failed in X.XXs ✖ building client + server bundles... Element is missing end tag.`
**原因**：Vue 编译器将 Markdown 正文中的 `<泛型>` 或 `<占位符>` 视为 HTML 标签。
**修复**：使用反引号包裹，如 `` `<T>` `` 或 `` `model` ``。

```markdown
<!-- ❌ 错误写法 -->
Ollama 拉取命令：ollama pull <model>

<!-- ✅ 正确写法 -->
Ollama 拉取命令：`ollama pull qwen2.5`
```

### 2. `{{ }}` 或 `{% %}` 被解析为 Vue 模板语法
**现象**：`Duplicate attribute.` 或 `Element is missing end tag.`
**原因**：Vue 编译器将 `{{ variable }}` 或 Jinja2 语法 `{% if %}` 误识别为 Vue 模板。
**修复**：使用反引号包裹，或在 `config.ts` 中配置 `vue.template.compilerOptions.isCustomElement`。

```markdown
<!-- ❌ 错误写法 -->
Jinja2 变量：{{ user.name }}

<!-- ✅ 正确写法 -->
Jinja2 变量：`{{ user.name }}`
```

### 3. 分裂的表格导致解析失败
**现象**：表格中间有空行时，空行后的 `<T>` 会被当作独立标签解析。
**原因**：Markdown 表格中间有空行会被断开，后续行的 `<T>` 脱离表格上下文。
**修复**：确保表格行紧密相连，或在表格前添加子标题分隔。

```markdown
<!-- ❌ 错误写法 -->
| 列1 | 列2 |
| --- | --- |
| A | B |

| C | D | <!-- 空行导致表格断裂 -->

<!-- ✅ 正确写法 -->
| 列1 | 列2 |
| --- | --- |
| A | B |
| C | D |
```

### 4. 扫描到示例项目目录中的 Markdown
**现象**：构建失败，报错路径指向 `*_demo/app/...` 等示例代码目录。
**原因**：VitePress 默认扫描根目录下所有 `.md` 文件，包括示例项目的 `README.md`，这些文件通常包含大量未转义的模板语法。
**修复**：在 `config.ts` 的 `srcExclude` 和 `sidebar.ts` 的 `excludeFolders` 中添加通配模式。

```typescript
// .vitepress/config.ts
export default defineConfig({
  srcExclude: [
    '**/*_demo/**',
    '**/*_basics/**',
    '**/flask_demo/**',
    // ... 其他通配符
  ],
})
```

### 5. 导航栏点击出现 404 (Page Not Found)
**现象**：点击顶部导航栏（如"基础入门"），页面显示 404，但侧边栏可能正常。
**原因**：`config.ts` 中 `nav` 的 `link` 指向了目录根路径（如 `/1-basics/`），但该目录下**没有** `README.md` 或 `index.md`。VitePress 默认将目录根路径映射为 `index.md`，若文件不存在则报 404。
**修复**：将 `link` 指向该目录下**实际存在的第一个子章节**。

```typescript
// ❌ 错误写法：目录根没有 index.md/README.md
{ text: '基础入门', link: '/1-basics/' },

// ✅ 正确写法：指向具体存在的页面
{ text: '基础入门', link: '/1-basics/01-intro/' },
```

---

## 常见问题

### 1. 部署后页面 404

- 检查 `base` 配置是否正确（必须以 `/` 开头和结尾）
- 确认 GitHub Pages 的 Source 设置为 GitHub Actions
- 检查仓库名与 `base` 路径是否一致

### 2. 侧边栏不显示

- 确认 `sidebar.ts` 中的 `documentRootPath` 和 `resolvePath` 正确
- 检查 `excludeFolders` 是否误排除了内容目录
- 确认 `useTitleFromFileHeading: true` 以使用 markdown 标题

### 3. 构建失败

- 检查 Node.js 版本（推荐 20+）
- 确认 `srcExclude` 排除了不需要构建的文件
- 查看 GitHub Actions 日志定位具体错误

### 4. 图片路径问题

图片放在 `.vitepress/public/` 目录，引用时使用绝对路径：

```markdown
![图片描述](/image.png)
```

## 参考资源

- [VitePress 官方文档](https://vitepress.dev/)
- [GitHub Pages 文档](https://docs.github.com/en/pages)
- [VitePress Sidebar 插件](https://github.com/nicklahh/vitepress-sidebar)
- [Rust 官方文档](https://doc.rust-lang.org/)
