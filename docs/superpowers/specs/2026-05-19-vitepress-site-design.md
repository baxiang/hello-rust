# VitePress 静态站点设计

> 将 Rust 中文教程的 ~300 个 Markdown 文件生成 VitePress 静态站点，部署到 GitHub Pages。

## 方案选型

VitePress + vitepress-sidebar 插件。选择理由：
- vitepress-sidebar 自动从目录结构生成 sidebar，解决 300 个条目的维护问题
- VitePress 原生 Markdown 扩展适合代码教程
- Vue 生态轻量，构建快

## 项目结构

在项目根目录新建 `docs/` 作为 VitePress 站点根目录。内容通过符号链接引用源目录，保证教程 Markdown 只维护一份。

```
hello-rust/
├── docs/                        # VitePress 站点
│   ├── .vitepress/
│   │   ├── config.ts            # 主配置（nav、sidebar、markdown 增强）
│   │   ├── theme/
│   │   │   ├── index.ts         # 自定义主题（注册 Playground 按钮）
│   │   │   └── playground.ts    # Rust Playground 代码块增强
│   │   └── sidebar.ts           # vitepress-sidebar 配置
│   ├── index.md                 # 首页
│   ├── 1-basics/ -> ../../1-basics/      # 符号链接
│   ├── 2-core/   -> ../../2-core/
│   ├── 3-data/   -> ../../3-data/
│   ├── 4-advanced/ -> ../../4-advanced/
│   ├── 5-projects/ -> ../../5-projects/
│   ├── 6-modern/ -> ../../6-modern/
│   ├── exercises/ -> ../../exercises/
│   └── package.json
├── 1-basics/                    # 原有内容（Markdown 文件名改为英文）
├── ...
```

## 文件名英文化

所有中文 Markdown 文件名改为英文 slug，URL 和文件路径 1:1 对应，无需 rewrite。

- `01-conditional-expressions.md` → `01-conditional-expressions.md`
- `02-loops.md` → `02-loops.md`
- `README.md` 保持不变（VitePress 默认映射为目录 index）
- 内部链接同步更新（`./01-conditional-expressions.md` → `./01-conditional-expressions.md`）
- 用脚本批量完成重命名 + 链接更新

重命名规则：
- 保留 `NN-` 数字前缀（排序）
- 中文部分转为英文 slug（kebab-case）
- 目录名已经是英文，不改

## Sidebar 与导航

**Nav 栏**：首页 | 基础入门 | 核心概念 | 数据与特性 | 高级主题 | 项目实战 | 现代实践 | 练习题

**Sidebar** 由 `vitepress-sidebar` 自动生成，7 个分组：

| 分组 | 目录 | 条目数 |
|------|------|--------|
| 基础入门 | 1-basics/ | 6 章 |
| 核心概念 | 2-core/ | 5 章 |
| 数据与特性 | 3-data/ | 8 章 |
| 高级主题 | 4-advanced/ | 9 章 |
| 项目实战 | 5-projects/ | 9 项目 |
| 现代实践 | 6-modern/ | 3 章 |
| 练习题 | exercises/ | 31 文件 |

插件配置要点：
- `collapsed: true` — 默认折叠
- 忽略非 `.md` 文件（`Cargo.toml`、`examples/`、`.rs`）
- 排序按文件名数字前缀

## Rust 代码块增强

1. **Playground 按钮**：自定义 markdown-it 插件，检测 ` ```rust ` 代码块，右上角添加"在 Playground 中运行"按钮，点击跳转 `https://play.rust-lang.org/` 并自动填入代码
2. **代码组图标**：`vitepress-plugin-group-icons` 给代码块标签加语言图标
3. **行高亮**：VitePress 内置 `{1,3-5}` 语法

## 部署

- GitHub Pages，`deploy.yml` 自动部署
- 触发：push 到 `main`
- 流程：`npm install` → `npm run docs:build` → `actions/deploy-pages@v4`
- `config.ts` 中 `base: '/hello-rust/'`

## 构建命令

```json
{
  "scripts": {
    "docs:dev": "vitepress dev docs",
    "docs:build": "vitepress build docs",
    "docs:preview": "vitepress preview docs"
  }
}
```

## 不在范围内

- 搜索功能（可用 VitePress 内置本地搜索，后续按需开启）
- i18n 多语言
- 自定义域名
