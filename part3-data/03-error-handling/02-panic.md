## 14.3 不可恢复的错误 - panic!

### panic! 宏基础

```rust
fn main() {
    // panic! 会立即终止程序
    panic!("程序崩溃了！");

    // 这行代码永远不会执行
    println!("这行不会输出");
}
```

**输出：**
```
thread 'main' panicked at '程序崩溃了！', src/main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### 自动触发 panic 的情况

```rust
fn main() {
    // 1. 数组/Vec 越界访问
    let v = vec![1, 2, 3];
    // let item = v[99];  // panic!

    // 2. 除零（整数）
    // let x = 10 / 0;  // panic!

    // 3. 断言失败
    // assert_eq!(1, 2);  // panic!
    // assert!(false);  // panic!

    // 4. 调试断言
    // debug_assert!(false);  // debug 模式 panic!

    // 5. unwrap None
    // let x: Option<i32> = None;
    // x.unwrap();  // panic!

    // 6. unwrap Err
    // let r: Result<i32, &str> = Err("error");
    // r.unwrap();  // panic!
}
```

### panic 的过程

```
panic 发生时的流程：

1. 打印错误消息
   thread 'main' panicked at '错误信息', src/main.rs:行：列

2. 展开栈（Stack Unwinding）
   • 清理沿途的所有局部变量
   • 调用所有析构函数
   • 释放所有资源

3. 打印栈回溯（如果设置了 RUST_BACKTRACE）

4. 程序终止，返回非零退出码

栈展开 vs 直接终止：
┌────────────────────────────────────────────┐
│ 默认：栈展开（推荐）                        │
│ • 安全：清理所有资源                        │
│ • 完整：调用所有析构函数                    │
│ • 开销：需要遍历栈帧                        │
├────────────────────────────────────────────┤
│ profile.release.panic = "abort"            │
│ • 快速：立即终止                            │
│ • 危险：可能泄漏资源                        │
│ • 适用：嵌入式等资源受限环境               │
└────────────────────────────────────────────┘
```

### 使用 RUST_BACKTRACE 调试

```bash
# 显示基本栈回溯
RUST_BACKTRACE=1 cargo run

# 显示完整栈回溯
RUST_BACKTRACE=full cargo run

# Cargo.toml 中设置调试选项
[profile.dev]
panic = "unwind"  # 默认，栈展开

[profile.release]
panic = "abort"   # 直接终止，减小编译产物
```

### 何时使用 panic!

```rust
// ✅ 适合 panic 的场景

// 1. 示例代码和原型
fn main() {
    let file = std::fs::File::open("hello.txt").unwrap();
}

// 2. 测试代码
#[test]
fn test_something() {
    let result = some_function();
    assert_eq!(result, expected_value);
}

// 3. 错误状态无法恢复
fn process_data(data: &[u8]) -> &str {
    // 如果数据格式不对，整个程序无法继续
    std::str::from_utf8(data).expect("数据必须是有效的 UTF-8")
}

// 4. 不变量被破坏（逻辑错误）
fn get_element(v: &[i32], index: usize) -> i32 {
    // 调用者保证索引有效，如果无效说明程序有 bug
    v[index]
}

// 5. 外部依赖失效
fn get_config() -> &'static str {
    // 配置文件应该在编译时生成
    // 如果不存在，说明部署有问题
    include_str!("../config/generated.txt")
}
```

---

---

## 下一步

[Result 枚举：可恢复错误](../4-Result 枚举：可恢复错误.md)

---

## 返回

[返回目录](../../README.md)