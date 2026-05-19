import { readdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const DIRS = ['1-basics', '2-core', '3-data', '4-advanced', '5-projects', '6-modern', 'exercises'];
const ROOT = '/Users/baxiang/Documents/hello-rust';

const RUST_TYPE_RE = /[A-Z][A-Za-z0-9_]*(?:::[A-Z][A-Za-z0-9_]*)*$/;

async function* walk(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  for (const entry of entries) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      yield* walk(full);
    } else if (entry.name.endsWith('.md')) {
      yield full;
    }
  }
}

function isInsideBacktick(line, pos) {
  let count = 0;
  for (let i = 0; i < pos; i++) {
    if (line[i] === '`') count++;
  }
  return count % 2 === 1;
}

function findRustGenerics(line) {
  const results = [];
  let i = 0;
  while (i < line.length) {
    if (line[i] === '<') {
      const before = line.slice(0, i);
      const typeMatch = before.match(RUST_TYPE_RE);
      if (typeMatch) {
        const genStart = i;
        let depth = 1;
        let j = i + 1;
        while (j < line.length && depth > 0) {
          if (line[j] === '<') depth++;
          else if (line[j] === '>') depth--;
          j++;
        }
        if (depth === 0) {
          const fullMatch = line.slice(genStart, j);
          if (isInsideBacktick(line, genStart)) {
            i = j;
            continue;
          }
          results.push({ start: genStart, end: j, text: fullMatch });
          i = j;
          continue;
        }
      }
    }
    i++;
  }
  return results;
}

function escapeAngleBrackets(text) {
  return text.replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

function processFile(content) {
  const lines = content.split('\n');
  let inCodeBlock = false;
  let modified = false;
  const outLines = [];
  const changedLineNums = [];

  for (let ln = 0; ln < lines.length; ln++) {
    const line = lines[ln];
    const trimmed = line.trimStart();

    if (trimmed.startsWith('```')) {
      inCodeBlock = !inCodeBlock;
      outLines.push(line);
      continue;
    }

    if (inCodeBlock || trimmed.startsWith('|')) {
      outLines.push(line);
      continue;
    }

    const matches = findRustGenerics(line);
    if (matches.length === 0) {
      outLines.push(line);
      continue;
    }

    let newLine = line;
    let offset = 0;

    for (const { start, end, text } of matches) {
      const adjStart = start + offset;
      const adjEnd = end + offset;
      const escaped = escapeAngleBrackets(text);
      if (escaped !== text) {
        newLine = newLine.slice(0, adjStart) + escaped + newLine.slice(adjEnd);
        offset += escaped.length - text.length;
        modified = true;
      }
    }

    if (newLine !== line) {
      changedLineNums.push(ln + 1);
    }
    outLines.push(newLine);
  }

  return { content: outLines.join('\n'), modified, changedLineNums };
}

async function main() {
  let totalFiles = 0;
  let modifiedFiles = 0;
  const details = [];

  for (const dir of DIRS) {
    const absDir = join(ROOT, dir);
    for await (const filePath of walk(absDir)) {
      totalFiles++;
      const content = await readFile(filePath, 'utf-8');
      const { content: newContent, modified, changedLineNums } = processFile(content);
      if (modified) {
        await writeFile(filePath, newContent, 'utf-8');
        modifiedFiles++;
        const relPath = filePath.replace(ROOT + '/', '');
        details.push({ file: relPath, lines: changedLineNums });

        for (const ln of changedLineNums) {
          const origLine = content.split('\n')[ln - 1].trim();
          console.log(`  ${relPath}:${ln}`);
          console.log(`    - ${origLine}`);
        }
      }
    }
  }

  console.log(`\n=== Summary ===`);
  console.log(`Scanned: ${totalFiles} files`);
  console.log(`Modified: ${modifiedFiles} files`);
}

main().catch(console.error);
