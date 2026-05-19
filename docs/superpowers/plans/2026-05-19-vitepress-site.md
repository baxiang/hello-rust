# VitePress 静态站点 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 Rust 中文教程的 ~300 个 Markdown 文件生成 VitePress 静态站点，部署到 GitHub Pages。

**Architecture:** VitePress 站点放在 `site/` 目录（避免与已有 `docs/` 冲突），通过符号链接引用源内容目录。中文文件名全部改为英文 slug，sidebar 由 `vitepress-sidebar` 插件自动生成。

**Tech Stack:** VitePress 1.x, vitepress-sidebar, vitepress-plugin-group-icons, GitHub Pages Actions

---

## File Structure

### New files
- `site/.vitepress/config.ts` — VitePress 主配置
- `site/.vitepress/theme/index.ts` — 自定义主题
- `site/.vitepress/theme/playground.ts` — Rust Playground 按钮
- `site/.vitepress/theme/custom.css` — 自定义样式
- `site/.vitepress/sidebar.ts` — vitepress-sidebar 配置
- `site/index.md` — 首页
- `site/package.json` — 依赖和脚本
- `scripts/rename-map.json` — 中文→英文文件名映射（见 Appendix）
- `scripts/rename-md-files.mjs` — 批量重命名脚本
- `scripts/update-links.mjs` — 内部链接更新脚本
- `.github/workflows/deploy.yml` — GitHub Pages 部署

### Symlinks (site/ → root)
- `site/1-basics/` → `../../1-basics/`
- `site/2-core/` → `../../2-core/`
- `site/3-data/` → `../../3-data/`
- `site/4-advanced/` → `../../4-advanced/`
- `site/5-projects/` → `../../5-projects/`
- `site/6-modern/` → `../../6-modern/`
- `site/exercises/` → `../../exercises/`

---

### Task 1: Initialize VitePress project

**Files:**
- Create: `site/package.json`, `site/index.md`, `site/.vitepress/config.ts`
- Create: symlinks for all 7 content directories

- [ ] **Step 1: Create site directory and package.json**

```bash
mkdir -p site/.vitepress/theme
```

Write `site/package.json`:

```json
{
  "name": "hello-rust-site",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "docs:dev": "vitepress dev site",
    "docs:build": "vitepress build site",
    "docs:preview": "vitepress preview site"
  },
  "devDependencies": {
    "vitepress": "^1.6.0",
    "vitepress-sidebar": "^1.31.0",
    "vitepress-plugin-group-icons": "^1.5.0"
  }
}
```

- [ ] **Step 2: Install dependencies**

```bash
npm install --prefix site
```

Expected: `site/node_modules/` created, `site/package-lock.json` generated

- [ ] **Step 3: Create symlinks**

```bash
cd site && ln -s ../../1-basics 1-basics && ln -s ../../2-core 2-core && ln -s ../../3-data 3-data && ln -s ../../4-advanced 4-advanced && ln -s ../../5-projects 5-projects && ln -s ../../6-modern 6-modern && ln -s ../../exercises exercises
```

- [ ] **Step 4: Create site/index.md**

VitePress home layout with: hero (name: "Rust 教程", text: "从零开始系统学习 Rust 编程语言"), two CTA buttons (开始学习 → /1-basics/01-intro/, 项目实战 → /5-projects/), four features (分层递进, 场景驱动, 编译器当导师, 代码可运行).

- [ ] **Step 5: Create skeleton config.ts**

`site/.vitepress/config.ts` with: title, description, lang 'zh-CN', base '/hello-rust/', nav bar (8 items), local search, socialLinks GitHub.

- [ ] **Step 6: Verify dev server starts**

```bash
npm run docs:dev
```

Expected: VitePress dev server starts on http://localhost:5173

- [ ] **Step 7: Commit**

```bash
git add site/ && git commit -m "feat: initialize VitePress site skeleton"
```

---

### Task 2: Batch rename Chinese MD files to English slugs

The rename mapping is in `scripts/rename-map.json`. See `scripts/rename-map.json` appendix file for full content.

**Files:**
- Create: `scripts/rename-map.json`, `scripts/rename-md-files.mjs`
- Modify: ~180 `.md` files (filenames only)

- [ ] **Step 1: Create `scripts/rename-map.json`**

Full mapping in `scripts/rename-map.json`. Format: `{ "dir/path": { "old.md": "new.md" } }`.

Slug conventions: `实战总结`→`practical-summary`, `基础`→`basics`, `概述`→`overview`, `高级特性`→`advanced-features`, `常见问题`→`faq`, `详解`→`details`. Technical terms in English: `所有权`→`ownership`, `生命周期`→`lifetime`, `闭包`→`closure`, etc. Stray exercise files: `*-exercises.md`→`exercises.md`.

- [ ] **Step 2: Write the rename script**

Write `scripts/rename-md-files.mjs`:

```javascript
import { rename, readFile, writeFile } from 'node:fs/promises'
import { join } from 'node:path'

const mapPath = join(import.meta.dirname, 'rename-map.json')
const rootDir = join(import.meta.dirname, '..')
const map = JSON.parse(await readFile(mapPath, 'utf-8'))
const log = []

for (const [dir, files] of Object.entries(map)) {
  for (const [oldName, newName] of Object.entries(files)) {
    const oldPath = join(rootDir, dir, oldName)
    const newPath = join(rootDir, dir, newName)
    try {
      await rename(oldPath, newPath)
      log.push(`${dir}/${oldName} -> ${dir}/${newName}`)
    } catch (e) {
      console.error(`FAILED: ${dir}/${oldName}: ${e.message}`)
    }
  }
}

await writeFile(join(rootDir, 'scripts', 'rename-log.txt'), log.join('\n'))
console.log(`Renamed ${log.length} files`)
```

- [ ] **Step 3: Run the rename script**

```bash
node scripts/rename-md-files.mjs
```

Expected: ~180 files renamed, `scripts/rename-log.txt` created

- [ ] **Step 4: Verify no Chinese filenames remain**

```bash
find 1-basics 2-core 3-data 4-advanced 5-projects 6-modern -name "*.md" | perl -ne 'print if /[\x{4e00}-\x{9fff}]/'
```

Expected: no output

- [ ] **Step 5: Commit**

```bash
git add -A && git commit -m "refactor: rename all Chinese MD filenames to English slugs"
```

---

### Task 3: Update all internal links

After renaming, ~280 internal links still point to old Chinese filenames.

**Files:**
- Create: `scripts/update-links.mjs`
- Modify: ~100 `.md` files (link targets updated)

- [ ] **Step 1: Write the link update script**

Write `scripts/update-links.mjs`:

```javascript
import { readFile, writeFile, readdir } from 'node:fs/promises'
import { join } from 'node:path'

const rootDir = join(import.meta.dirname, '..')
const mapPath = join(import.meta.dirname, 'rename-map.json')
const map = JSON.parse(await readFile(mapPath, 'utf-8'))

const replacements = {}
for (const [dir, files] of Object.entries(map)) {
  for (const [oldName, newName] of Object.entries(files)) {
    replacements[oldName] = newName
  }
}

async function processDir(dirPath) {
  const entries = await readdir(dirPath, { withFileTypes: true })
  for (const entry of entries) {
    const fullPath = join(dirPath, entry.name)
    if (entry.isDirectory() && !entry.name.startsWith('.') && entry.name !== 'node_modules' && entry.name !== 'target') {
      await processDir(fullPath)
    } else if (entry.name.endsWith('.md')) {
      let content = await readFile(fullPath, 'utf-8')
      let changed = false
      for (const [oldName, newName] of Object.entries(replacements)) {
        if (content.includes(oldName)) {
          content = content.replaceAll(oldName, newName)
          changed = true
        }
      }
      if (changed) {
        await writeFile(fullPath, content)
        console.log(`Updated: ${fullPath}`)
      }
    }
  }
}

await processDir(rootDir)
```

- [ ] **Step 2: Run the link update script**

```bash
node scripts/update-links.mjs
```

Expected: ~100 files updated

- [ ] **Step 3: Verify no broken links remain**

```bash
grep -r '](./' --include="*.md" 1-basics 2-core 3-data 4-advanced 5-projects 6-modern exercises | perl -ne 'print if /[\x{4e00}-\x{9fff}]/'
```

Expected: no output

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "fix: update internal links after filename rename"
```

---

### Task 4: Configure vitepress-sidebar and complete config.ts

**Files:**
- Create: `site/.vitepress/sidebar.ts`
- Modify: `site/.vitepress/config.ts`

- [ ] **Step 1: Create sidebar configuration**

Write `site/.vitepress/sidebar.ts` using `generateSidebar` from `vitepress-sidebar` with 7 groups (1-basics, 2-core, 3-data, 4-advanced, 5-projects, 6-modern, exercises). Each group: `useTitleFromFileHeading: true`, `collapsed: true`, exclude `examples/`, `target/`, `src/`, `Cargo.toml`.

- [ ] **Step 2: Update config.ts**

Add to `site/.vitepress/config.ts`: import sidebar, import `playgroundPlugin` from theme, add `sidebar` to themeConfig, add `markdown.config` to use playgroundPlugin, add `srcExclude: ['**/examples/**', '**/target/**', '**/Cargo.toml']`, add `markdown.lineNumbers: true`.

- [ ] **Step 3: Verify sidebar renders**

```bash
npm run docs:dev
```

Expected: Sidebar shows 7 collapsed groups with chapter links

- [ ] **Step 4: Commit**

```bash
git add site/.vitepress/sidebar.ts site/.vitepress/config.ts && git commit -m "feat: configure vitepress-sidebar with 7 content groups"
```

---

### Task 5: Add Rust Playground button for code blocks

**Files:**
- Create: `site/.vitepress/theme/playground.ts`
- Create: `site/.vitepress/theme/custom.css`
- Modify: `site/.vitepress/theme/index.ts`

- [ ] **Step 1: Create the Playground markdown-it plugin**

Write `site/.vitepress/theme/playground.ts`: a markdown-it fence renderer override that wraps `rust` code blocks in a `<div class="vp-playground">` with a `<a>` button linking to `https://play.rust-lang.org/#code=` + URL-encoded JSON payload `{channel: "stable", mode: "debug", edition: "2021", code}`.

- [ ] **Step 2: Create custom CSS**

Write `site/.vitepress/theme/custom.css`: `.vp-playground` relative positioning, `.vp-playground-btn` absolute top-right, opacity 0, visible on hover, styled to match VitePress copy button variables.

- [ ] **Step 3: Create theme entry**

Write `site/.vitepress/theme/index.ts`: extends DefaultTheme, imports custom.css.

- [ ] **Step 4: Verify Playground button**

```bash
npm run docs:dev
```

Expected: Hover over any `rust` code block shows "Run" button in top-right

- [ ] **Step 5: Commit**

```bash
git add site/.vitepress/theme/ && git commit -m "feat: add Rust Playground button for code blocks"
```

---

### Task 6: Add GitHub Pages deployment workflow

**Files:**
- Create: `.github/workflows/deploy.yml`

- [ ] **Step 1: Create deploy.yml**

Write `.github/workflows/deploy.yml`: trigger on push to main, permissions contents: write + pages: write, uses actions/configure-pages, actions/upload-pages-artifact (path: site/.vitepress/dist), actions/deploy-pages. Build step: `npm ci --prefix site && npm run docs:build`.

- [ ] **Step 2: Add site/dist and site/node_modules to .gitignore**

Add `site/dist/` and `site/node_modules/` to `.gitignore`.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/deploy.yml .gitignore && git commit -m "feat: add GitHub Pages deployment workflow"
```

---

### Task 7: Final verification

- [ ] **Step 1: Run full build**

```bash
npm run docs:build
```

Expected: Build succeeds, output in `site/.vitepress/dist/`

- [ ] **Step 2: Preview production build**

```bash
npm run docs:preview
```

Expected: Preview server starts, all pages accessible

- [ ] **Step 3: Verify no broken internal links**

Manual spot-check of sidebar navigation across all 7 groups.

- [ ] **Step 4: Final commit and push**

```bash
git push
```
