## 21.7 构建配置（Profiles）

### 默认配置

```toml
[profile.dev]
opt-level = 0           # 无优化
debug = true            # 调试信息
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'

[profile.release]
opt-level = 3           # 最大优化
debug = false
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'

[profile.test]
opt-level = 0
debug = true

[profile.bench]
opt-level = 3
```

### 自定义配置

```toml
# 快速发布构建（平衡编译速度和运行速度）
[profile.release]
opt-level = 2
lto = "thin"

# 最小二进制大小
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

# 开发环境启用部分优化
[profile.dev]
opt-level = 1

[profile.dev.package."*"]  # 依赖的优化
opt-level = 3
```




