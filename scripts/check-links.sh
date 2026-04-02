#!/bin/bash

# Markdown 链接检查脚本
# 检查文档中的链接是否有效

set -e

echo "=== Markdown 链接检查 ==="

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# 检查是否安装了 markdown-link-check
if ! command -v markdown-link-check &> /dev/null; then
    echo "${YELLOW}安装 markdown-link-check...${NC}"
    npm install -g markdown-link-check
fi

# 检查所有 Markdown 文件
find . -name "*.md" -type f | while read file; do
    echo ""
    echo "检查文件：$file"
    
    if markdown-link-check "$file" --config .markdown-link-check.json 2>&1; then
        echo "${GREEN}✅ 链接有效：$file${NC}"
    else
        echo "${YELLOW}⚠️  链接问题：$file${NC}"
    fi
done

echo ""
echo "=== 链接检查完成 ==="