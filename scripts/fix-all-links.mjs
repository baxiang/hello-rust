import { readFile, writeFile, readdir } from 'node:fs/promises'
import { join, relative, dirname } from 'node:path'

const rootDir = join(import.meta.dirname, '..')
const mapPath = join(import.meta.dirname, 'rename-map.json')

const renameMap = JSON.parse(await readFile(mapPath, 'utf-8'))

const filenameMap = {}
for (const [dir, files] of Object.entries(renameMap)) {
  for (const [oldName, newName] of Object.entries(files)) {
    filenameMap[oldName] = newName
  }
}

const dirPrefixReplacements = [
  [/\bpart1-basics\//g, '1-basics/'],
  [/\bpart2-core\//g, '2-core/'],
  [/\bpart3-data\//g, '3-data/'],
  [/\bpart4-advanced\//g, '4-advanced/'],
  [/\bpart5-projects\//g, '5-projects/'],
  [/\bpart6-modern\//g, '6-modern/'],
]

const exerciseNameReplacements = [
  [/\bownership-exercises\.md\b/g, 'exercises.md'],
  [/\bownership-exercises\b/g, 'exercises'],
  [/\breferences-exercises\.md\b/g, 'exercises.md'],
  [/\breferences-exercises\b/g, 'exercises'],
  [/\blifetimes-exercises\.md\b/g, 'exercises.md'],
  [/\blifetimes-exercises\b/g, 'exercises'],
  [/\btraits-exercises\.md\b/g, 'exercises.md'],
  [/\btraits-exercises\b/g, 'exercises'],
]

const chapterToExercise = {
  '2-core/01-ownership': '07-ownership',
  '2-core/02-references': '08-references',
  '2-core/03-slices': '09-slices',
  '2-core/04-structs': '10-structs',
  '2-core/05-enums': '11-enums',
  '3-data/01-collections': '12-collections',
  '3-data/02-hashmap': '13-hashmap',
  '3-data/03-error-handling': '14-error-handling',
  '3-data/04-generics': '15-generics',
  '3-data/05-traits': '16-traits',
  '3-data/06-lifetimes': '17-lifetimes',
  '3-data/07-closures': '18-closures',
  '3-data/08-iterators': '19-iterators',
  '4-advanced/01-modules': '20-modules',
  '4-advanced/02-cargo': '21-cargo',
  '4-advanced/03-smart-pointers': '22-smart-pointers',
  '4-advanced/04-concurrency': '23-concurrency',
  '4-advanced/05-unsafe': '24-unsafe',
  '4-advanced/06-macros': '25-macros',
  '4-advanced/07-cli': '26-cli',
  '4-advanced/08-web': '27-web',
  '4-advanced/09-testing': '28-testing',
  '6-modern/01-rust-2024': '29-rust-2024',
  '6-modern/02-async': '30-async',
  '6-modern/03-webassembly': '31-webassembly',
}

const chapterOrder = [
  '1-basics/01-intro',
  '1-basics/02-first-program',
  '1-basics/03-variables',
  '1-basics/04-types',
  '1-basics/05-functions',
  '1-basics/06-control-flow',
  '2-core/01-ownership',
  '2-core/02-references',
  '2-core/03-slices',
  '2-core/04-structs',
  '2-core/05-enums',
  '3-data/01-collections',
  '3-data/02-hashmap',
  '3-data/03-error-handling',
  '3-data/04-generics',
  '3-data/05-traits',
  '3-data/06-lifetimes',
  '3-data/07-closures',
  '3-data/08-iterators',
  '4-advanced/01-modules',
  '4-advanced/02-cargo',
  '4-advanced/03-smart-pointers',
  '4-advanced/04-concurrency',
  '4-advanced/05-unsafe',
  '4-advanced/06-macros',
  '4-advanced/07-cli',
  '4-advanced/08-web',
  '4-advanced/09-testing',
  '6-modern/01-rust-2024',
  '6-modern/02-async',
  '6-modern/03-webassembly',
]

const nextChapterNames = {
  '1-basics/03-variables': { label: '04-数据类型', next: '../04-types' },
  '1-basics/04-types': { label: '05-函数', next: '../05-functions' },
  '1-basics/05-functions': { label: '06-控制流', next: '../06-control-flow' },
  '1-basics/06-control-flow': { label: '07-所有权与借用', next: '../../2-core/01-ownership' },
  '4-advanced/02-cargo': { label: '22-智能指针', next: '../03-smart-pointers' },
  '4-advanced/03-smart-pointers': { label: '23-并发编程', next: '../04-concurrency' },
  '4-advanced/04-concurrency': { label: '24-Unsafe-Rust', next: '../05-unsafe' },
  '4-advanced/05-unsafe': { label: '25-宏', next: '../06-macros' },
  '4-advanced/06-macros': { label: '26-命令行工具', next: '../07-cli' },
  '4-advanced/07-cli': { label: '27-Web 服务器', next: '../08-web' },
  '4-advanced/08-web': { label: '28-测试', next: '../09-testing' },
}

let totalFilesModified = 0
let totalLinesChanged = 0

async function walkDir(dirPath, callback) {
  const entries = await readdir(dirPath, { withFileTypes: true })
  for (const entry of entries) {
    const fullPath = join(dirPath, entry.name)
    if (entry.isDirectory()) {
      if (['.git', 'node_modules', 'target', '.vitepress', 'site', 'dist'].includes(entry.name)) continue
      await walkDir(fullPath, callback)
    } else if (entry.name.endsWith('.md')) {
      await callback(fullPath)
    }
  }
}

function escapeRegex(str) {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

function looksLikeCode(line) {
  const trimmed = line.trimStart()
  if (trimmed.length === 0) return false
  const codeIndicators = [
    /^(let |mut |fn |pub |use |impl |struct |enum |trait |mod |const |static |type |where |match |if |else |for |while |loop |return |break |continue |unsafe |async |await |macro_rules!)/,
    /^[a-z_]+\(/,
    /^\}/,
    /^\{/,
    /^#\[/,
    /^\/\/[!/]/,
    /^\s*\.\w+\(/,
    /^\s*self::/,
    /^\s*super::/,
    /^\s*crate::/,
  ]
  for (const re of codeIndicators) {
    if (re.test(trimmed)) return true
  }
  return false
}

function isProseLinkLine(line) {
  return line.includes('](') && !line.trimStart().startsWith('```') && !looksLikeCode(line)
}

function getChapterDir(filePath) {
  const relPath = relative(rootDir, filePath)
  const parts = relPath.split('/')
  if (parts.length >= 2) {
    return parts[0] + '/' + parts[1]
  }
  return null
}

function fixLine(line, filePath, isExercisesDir) {
  if (line.trimStart().startsWith('```')) return { line, changed: false }

  const hasRelevantContent = line.includes('.md') || line.includes('part1-') || line.includes('part2-') ||
    line.includes('part3-') || line.includes('part4-') || line.includes('part5-') ||
    line.includes('part6-') || line.includes('-exercises') || line.includes('](') ||
    line.includes('exercises/')

  if (!hasRelevantContent) return { line, changed: false }

  let text = line
  let changed = false

  // 1. Fix Chinese filename links
  if (isProseLinkLine(line)) {
    for (const [oldName, newName] of Object.entries(filenameMap)) {
      const oldBase = oldName.replace(/\.md$/, '')
      const newBase = newName.replace(/\.md$/, '')
      const re = new RegExp(`(\\]\\([^)]*?)${escapeRegex(oldBase)}((?:\\.md)?[)#"])`, 'g')
      const newStr = text.replace(re, `$1${newBase}$2`)
      if (newStr !== text) { text = newStr; changed = true }
    }
  }

  // 2. Fix directory prefixes
  if (line.includes('](') || !looksLikeCode(line)) {
    for (const [pattern, replacement] of dirPrefixReplacements) {
      const newStr = text.replace(pattern, replacement)
      if (newStr !== text) { text = newStr; changed = true }
    }
  }

  // 3. Fix old exercise file names
  if (line.includes('](') || line.includes('-exercises')) {
    for (const [pattern, replacement] of exerciseNameReplacements) {
      const newStr = text.replace(pattern, replacement)
      if (newStr !== text) { text = newStr; changed = true }
    }
  }

  // 4. Fix exercise links with wrong section numbers
  // e.g. ../../exercises/02-references -> ../../exercises/08-references
  const chapterDir = getChapterDir(filePath)
  if (chapterDir && chapterToExercise[chapterDir] && text.includes('exercises/')) {
    const correctExercise = chapterToExercise[chapterDir]
    // Find the local section number pattern in the exercise link
    // The current link uses the local section number like 02-references
    // We need to replace it with the global chapter number like 08-references
    const localSectionMatch = chapterDir.match(/\/(\d+)-/)
    if (localSectionMatch) {
      const localNum = localSectionMatch[1]
      // Replace patterns like exercises/02-references with exercises/08-references
      // But only if the topic part after the number matches
      const re = new RegExp(`(exercises\\/)${localNum}(-[a-z-]+)`, 'g')
      const newStr = text.replace(re, (match, prefix, suffix) => {
        const correctBase = correctExercise.replace(/^\d+-/, '')
        if (suffix.substring(1) === correctBase) {
          return prefix + correctExercise
        }
        // Check if it's the same topic even with different naming
        return match
      })
      if (newStr !== text) { text = newStr; changed = true }
    }
  }

  // 5. Fix "next chapter" navigation links with Chinese names
  if (chapterDir && nextChapterNames[chapterDir]) {
    const { label, next } = nextChapterNames[chapterDir]
    // Match patterns like: [07-所有权与借用](07-所有权与借用) or [下一章：07-所有权与借用](07-所有权与借用)
    const re = new RegExp(`(\\]\\()${escapeRegex(label)}((?:\\.md)?[)])`, 'g')
    const newStr = text.replace(re, `$1${next}$2`)
    if (newStr !== text) { text = newStr; changed = true }
  }

  // 6. Remove .md extensions from internal links (except in exercises/ dir)
  if (!isExercisesDir && isProseLinkLine(line)) {
    const newStr = text.replace(/(\]\([^)]*?)\.md([)#"])/g, '$1$2')
    if (newStr !== text) { text = newStr; changed = true }
  }

  return { line: text, changed }
}

async function processFile(filePath) {
  const relativePath = relative(rootDir, filePath)
  const isExercisesDir = relativePath.startsWith('exercises/')
  const isDocsSuperpowers = relativePath.startsWith('docs/superpowers/')

  if (isDocsSuperpowers) return

  let content = await readFile(filePath, 'utf-8')
  const lines = content.split('\n')

  let fileChanged = false
  let fileLinesChanged = 0
  const newLines = []

  let inCodeBlock = false

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]
    const trimmed = line.trimStart()

    if (trimmed.startsWith('```')) {
      inCodeBlock = !inCodeBlock
      newLines.push(line)
      continue
    }

    if (inCodeBlock) {
      if (isProseLinkLine(line) && (line.includes('.md)') || line.includes('exercises/'))) {
        const result = fixLine(line, filePath, isExercisesDir)
        if (result.changed) {
          fileChanged = true
          fileLinesChanged++
        }
        newLines.push(result.line)
      } else {
        newLines.push(line)
      }
      continue
    }

    const result = fixLine(line, filePath, isExercisesDir)
    if (result.changed) {
      fileChanged = true
      fileLinesChanged++
    }
    newLines.push(result.line)
  }

  if (fileChanged) {
    const newContent = newLines.join('\n')
    await writeFile(filePath, newContent)
    totalFilesModified++
    totalLinesChanged += fileLinesChanged
    console.log(`Fixed: ${relativePath} (${fileLinesChanged} lines)`)
  }
}

console.log('Starting comprehensive link fix (v4)...')
console.log(`Rename map has ${Object.keys(filenameMap).length} filename entries`)
console.log(`Chapter-to-exercise mapping: ${Object.keys(chapterToExercise).length} entries`)
console.log(`Next-chapter mapping: ${Object.keys(nextChapterNames).length} entries`)
console.log('')

await walkDir(rootDir, processFile)

console.log('')
console.log('=== Summary ===')
console.log(`Files modified: ${totalFilesModified}`)
console.log(`Lines changed: ${totalLinesChanged}`)
