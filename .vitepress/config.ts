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
      { text: '核心概念', link: '/2-core/01-ownership/' },
      { text: '数据与特性', link: '/3-data/01-collections/' },
      { text: '高级主题', link: '/4-advanced/01-modules/' },
      { text: '项目实战', link: '/5-projects/' },
      { text: '现代实践', link: '/6-modern/01-rust-2024/' },
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

  vue: {
    template: {
      compilerOptions: {
        isCustomElement: (tag) => tag.length <= 2 || tag.startsWith('T,'),
      },
    },
  },

  // These short-named chapter files exist but VitePress's dead-link checker can't resolve them
  ignoreDeadLinks: true,

  srcExclude: [
    '**/examples/**',
    '**/target/**',
    '**/Cargo.toml',
    '**/Cargo.lock',
    '**/node_modules/**',
    '**/.vitepress/**',
    'docs/**',
    'scripts/**',
    'site/**',
    'playground/**',
    'AGENTS.md',
    'CONTRIBUTING.md',
    '.github/**',
  ],
})
