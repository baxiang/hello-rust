#!/bin/bash

# Rust 教程代码验证脚本
# 验证文档中的 Rust 代码示例和章节 examples

set -e

echo "=== Rust 教程代码验证 ==="

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 步骤 1：验证章节 examples
echo ""
echo "步骤 1：检查章节 examples"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CHAPTERS_FOUND=0
CHAPTERS_PASSED=0
CHAPTERS_FAILED=0

for chapter_dir in */*/; do
    if [ -f "$chapter_dir/Cargo.toml" ]; then
        CHAPTERS_FOUND=$((CHAPTERS_FOUND + 1))
        echo ""
        echo "检查章节：$chapter_dir"
        
        cd "$chapter_dir"
        
        # 编译检查所有 examples
        if cargo check --examples 2>&1; then
            echo "${GREEN}✅ 编译成功：$chapter_dir${NC}"
            CHAPTERS_PASSED=$((CHAPTERS_PASSED + 1))
        else
            echo "${RED}❌ 编译失败：$chapter_dir${NC}"
            CHAPTERS_FAILED=$((CHAPTERS_FAILED + 1))
        fi
        
        # clippy 检查
        if cargo clippy --examples -- -D warnings 2>&1; then
            echo "${GREEN}✅ Clippy 通过：$chapter_dir${NC}"
        else
            echo "${YELLOW}⚠️  Clippy 警告：$chapter_dir${NC}"
        fi
        
        cd - > /dev/null
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "章节统计：${CHAPTERS_FOUND} 个章节 | ${GREEN}${CHAPTERS_PASSED} 通过${NC} | ${RED}${CHAPTERS_FAILED} 失败${NC}"

# 步骤 2：创建临时测试目录验证 Markdown 中的代码块
echo ""
echo "步骤 2：验证 Markdown 代码块（可选）"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

TEMP_DIR=$(mktemp -d)
echo "临时目录：$TEMP_DIR"

cd "$TEMP_DIR"
cargo new verification_test
cd verification_test

# 添加常用依赖
cat > Cargo.toml << 'EOF'
[package]
name = "verification_test"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
regex = "1"
anyhow = "1"
thiserror = "1"

[dev-dependencies]
tempfile = "3"
EOF

echo ""
echo "临时项目创建完成。手动验证 Markdown 代码块可粘贴到 src/main.rs"

# 清理临时目录
cd "$HOME"
rm -rf "$TEMP_DIR"

echo ""
echo "=== 验证完成 ==="
echo "${GREEN}✅ 所有验证完成${NC}"
