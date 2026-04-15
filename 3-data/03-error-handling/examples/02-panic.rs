//! # 示例：panic
//!
//! 对应章节：02-panic
//! 运行：cargo run --example 02-panic

fn main() {
    println!("=== Panic 演示 ===\n");

    // ✅ 正确示例：显式 panic
    // panic!("这是一个 panic");  // 取消注释测试

    // ✅ 正确示例：越界访问导致 panic
    let v = vec![1, 2, 3];
    // let x = v[99];  // panic! 越界

    // ✅ 正确示例：unwrap 导致 panic
    let _x: Option<i32> = None;
    // x.unwrap();  // panic! None 上 unwrap

    // ✅ 正确示例：expect 带上下文
    // x.expect("期望有值但实际为 None");

    // ✅ 正确示例：安全处理
    let result = v.get(99);
    match result {
        Some(val) => println!("值：{}", val),
        None => println!("索引越界，安全处理"),
    }

    // ✅ 正确示例：assert! 宏
    let age = 25;
    assert!(age >= 0, "年龄不能为负");
    assert_eq!(age + 1, 26);
    println!("assert 检查通过");

    // 尝试修改：
    // 1. 取消 panic 相关注释观察错误
    // 2. 使用 RUST_BACKTRACE=1 运行查看堆栈
    // 3. 思考：什么时候应该 panic？

    println!("\npanic 用于不可恢复的错误！");
}
