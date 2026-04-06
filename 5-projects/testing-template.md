# 测试策略

> 确保代码质量和功能正确性

---

## 测试类型

### 单元测试
测试单个函数和模块。

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        // 测试代码
    }
}
```

### 集成测试
测试模块协作。

**位置：** `tests/` 目录

```rust
use project_name::*;

#[test]
fn test_workflow() {
    // 集成测试代码
}
```

### 文档测试
测试文档示例。

```rust
/// # Examples
/// 
/// ```
/// use project_name::Feature;
/// // 示例代码
/// ```
pub fn feature() { }
```

---

## 测试场景

根据项目特性，重点测试：

1. **核心功能**
2. **错误处理**
3. **边界条件**
4. **数据持久化**

---

## 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 显示输出
cargo test -- --nocapture

# 测试覆盖率
cargo tarpaulin
```

---

## 测试最佳实践

- 使用 AAA 模式（Arrange-Act-Assert）
- 测试命名描述预期行为
- 测试隔离，避免共享状态
- 测试错误情况

---

**完整测试策略详见各项目具体文档。**
