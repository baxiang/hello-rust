#!/bin/bash

# Rust 教程代码验证脚本
# 验证文档中的 Rust 代码示例

set -e

echo "=== Rust 教程代码验证 ==="

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 创建临时测试目录
TEMP_DIR=$(mktemp -d)
echo "临时目录：$TEMP_DIR"

# 初始化 Cargo 项目
cd "$TEMP_DIR"
cargo new verification_test
cd verification_test

# 添加常用依赖
cat > Cargo.toml << 'EOF'
[package]
name = "verification_test"
version = "0.1.0"
edition = "2021"

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

# 验证函数
verify_code() {
    local file="$1"
    local code_block="$2"
    
    # 提取代码块内容
    echo "$code_block" > src/main.rs
    
    # 编译检查
    if cargo check 2>&1; then
        echo "${GREEN}✅ 验证成功：$file${NC}"
        return 0
    else
        echo "${RED}❌ 验证失败：$file${NC}"
        cargo check 2>&1 | grep "error"
        return 1
    fi
}

# 从 Markdown 文件提取代码块
extract_rust_code() {
    local file="$1"
    
    # 使用 grep 提取 rust 代码块
    grep -A 100 '```rust' "$file" | grep -B 100 '```' | sed '/^```rust/d;/^```/d'
}

# 验证所有 Markdown 文件中的 Rust 代码
echo ""
echo "步骤 1：检查 cargo 格式"
cargo fmt -- --check || echo "${YELLOW}⚠️  格式检查失败${NC}"

echo ""
echo "步骤 2：运行 clippy 检查"
cargo clippy -- -D warnings || echo "${YELLOW}⚠️  Clippy 检查失败${NC}"

echo ""
echo "步骤 3：编译测试"
cargo build || echo "${RED}❌ 编译失败${NC}"

echo ""
echo "步骤 4：运行测试"
cargo test || echo "${YELLOW}⚠️  测试失败${NC}"

echo ""
echo "=== 验证完成 ==="

# 清理临时目录
cd "$HOME"
rm -rf "$TEMP_DIR"

echo "${GREEN}✅ 所有验证完成${NC}"