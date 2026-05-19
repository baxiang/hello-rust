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
