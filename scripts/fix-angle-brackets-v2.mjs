import { readdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const DIRS = ['1-basics', '2-core', '3-data', '4-advanced', '5-projects', '6-modern', 'exercises'];
const ROOT = '/Users/baxiang/Documents/hello-rust';

const VALID_HTML_TAGS = new Set([
  'details', 'summary', 'br', 'hr', 'a', 'img', 'p', 'div', 'span',
  'table', 'tr', 'td', 'th', 'thead', 'tbody', 'ul', 'ol', 'li',
  'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'strong', 'em', 'code', 'pre',
  'blockquote', 'sup', 'sub', 'input', 'label', 'button', 'form',
  'select', 'option', 'textarea', 'section', 'article', 'header',
  'footer', 'nav', 'aside', 'main', 'figure', 'figcaption', 'dl',
  'dt', 'dd', 'del', 'ins', 'mark', 'small', 'abbr', 'cite', 'kbd',
  'samp', 'var', 'output', 'progress', 'meter', 'ruby', 'rt', 'rp',
  'bdi', 'bdo', 'wbr', 'caption', 'col', 'colgroup', 'datalist',
  'fieldset', 'legend', 'optgroup', 'picture', 'source', 'track',
  'video', 'audio', 'canvas', 'svg', 'math', 'iframe', 'embed',
  'object', 'param', 'map', 'area', 'noscript', 'script', 'style',
  'link', 'meta', 'title', 'base', 'template', 'slot', 'component',
]);

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

function extractBlockquotePrefix(line) {
  const match = line.match(/^((?:\s*>[\s]*)*)/);
  return match ? match[1] : '';
}

function isInsideInlineCode(line, pos) {
  let count = 0;
  for (let i = 0; i < pos; i++) {
    if (line[i] === '`') count++;
  }
  return count % 2 === 1;
}

function escapeContent(text) {
  let result = '';
  let i = 0;
  const len = text.length;

  while (i < len) {
    if (text[i] === '<') {
      if (isInsideInlineCode(text, i)) {
        result += '<';
        i++;
        continue;
      }

      const tagMatch = text.slice(i).match(/^<\/?([a-zA-Z][a-zA-Z0-9_-]*)/);
      if (tagMatch) {
        const tagName = tagMatch[1].toLowerCase();
        if (VALID_HTML_TAGS.has(tagName)) {
          const fullTagMatch = text.slice(i).match(/^<\/?[a-zA-Z][a-zA-Z0-9_-]*(?:\s[^>]*)?\/?>/);
          if (fullTagMatch) {
            result += fullTagMatch[0];
            i += fullTagMatch[0].length;
            continue;
          }
        }
      }

      result += '&lt;';
      i++;
    } else if (text[i] === '>') {
      if (isInsideInlineCode(text, i)) {
        result += '>';
        i++;
        continue;
      }

      const afterText = text.slice(i);
      const closeTagMatch = afterText.match(/^<\/([a-zA-Z][a-zA-Z0-9_-]*)\s*>/);
      if (closeTagMatch) {
        const tagName = closeTagMatch[1].toLowerCase();
        if (VALID_HTML_TAGS.has(tagName)) {
          result += closeTagMatch[0];
          i += closeTagMatch[0].length;
          continue;
        }
      }

      result += '&gt;';
      i++;
    } else {
      result += text[i];
      i++;
    }
  }

  return result;
}

function processLine(line) {
  const prefix = extractBlockquotePrefix(line);
  const content = line.slice(prefix.length);
  const escapedContent = escapeContent(content);
  return prefix + escapedContent;
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

    if (inCodeBlock) {
      outLines.push(line);
      continue;
    }

    if (!line.includes('<') && !line.includes('>')) {
      outLines.push(line);
      continue;
    }

    const newLine = processLine(line);
    if (newLine !== line) {
      modified = true;
      changedLineNums.push(ln + 1);
    }
    outLines.push(newLine);
  }

  return { content: outLines.join('\n'), modified, changedLineNums };
}

async function main() {
  let totalFiles = 0;
  let modifiedFiles = 0;

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
        for (const ln of changedLineNums) {
          console.log(`  ${relPath}:${ln}`);
        }
      }
    }
  }

  console.log(`\n=== Summary ===`);
  console.log(`Scanned: ${totalFiles} files`);
  console.log(`Modified: ${modifiedFiles} files`);
}

main().catch(console.error);
