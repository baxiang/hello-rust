## 21.9 发布 Crate

### 准备工作

```bash
# 1. 注册 crates.io 账号
# 访问 https://crates.io/login

# 2. 获取 API token
# 登录后访问 Account Settings

# 3. 本地登录
cargo login <your_api_token>
```

### 发布前检查

```bash
# 检查包内容
cargo package --list

# 本地测试打包
cargo package

# 测试发布（不实际上传）
cargo publish --dry-run
```

### 发布

```bash
# 发布到 crates.io
cargo publish

# 发布到私有 registry
cargo publish --registry my-registry
```

### 版本管理

```bash
# 1. 更新 Cargo.toml 中的 version
# 2. 提交更改
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# 3. 打标签
git tag v0.2.0

# 4. 推送
git push
git push --tags

# 5. 发布
cargo publish
```

---

---

## 下一步

[配置优化](../10-配置优化.md)