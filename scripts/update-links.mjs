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
