import type { PluginSimple } from 'markdown-it'
import type MarkdownIt from 'markdown-it'

const PLAYGROUND_URL = 'https://play.rust-lang.org/'

function encodeToPlayground(code: string): string {
  const payload = JSON.stringify({
    channel: 'stable',
    mode: 'debug',
    edition: '2021',
    code,
  })
  return PLAYGROUND_URL + '#code=' + encodeURIComponent(payload)
}

export const playgroundPlugin: PluginSimple = (md: MarkdownIt) => {
  const defaultFence = md.renderer.rules.fence!

  md.renderer.rules.fence = (tokens, idx, options, env, self) => {
    const token = tokens[idx]
    const info = token.info.trim().toLowerCase()

    if (info === 'rust' || info.startsWith('rust,')) {
      const html = defaultFence(tokens, idx, options, env, self)
      const url = encodeToPlayground(token.content)
      const button = `<a href="${url}" target="_blank" rel="noopener" class="vp-playground-btn" title="Run in Rust Playground">&#9654; Run</a>`
      return `<div class="vp-playground">${html}${button}</div>`
    }

    return defaultFence(tokens, idx, options, env, self)
  }
}
