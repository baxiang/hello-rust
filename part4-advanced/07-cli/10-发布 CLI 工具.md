## 26.10 发布 CLI 工具

### 交叉编译

```bash
# 添加目标平台
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin

# 构建
cargo build --release --target x86_64-unknown-linux-musl

# 使用 cross 简化交叉编译
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
```

### 使用 Cargo Binstall

```bash
# 安装
cargo install cargo-binstall

# 发布后安装
cargo binstall my_cli
```

### GitHub Releases

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cargo build --release
      - name: Release
        uses: softprops/action-gh-release@v1
        with:
          files: target/release/my_cli
```

---

---

## 下一步

[练习](../11-练习.md)