//! # 示例：错误处理实战总结
//!
//! 对应章节：05-实战总结
//! 运行：cargo run --example 05-error-review

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 错误处理实战总结 ===\n");

    // 1. panic 处理
    println!("1. panic 用于不可恢复错误");
    // panic!("演示");  // 不可恢复

    // 2. Result 基础
    println!("\n2. Result 基础：");
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(v) => println!("  成功：{}", v),
        Err(e) => println!("  失败：{}", e),
    }

    // 3. ? 操作符
    println!("\n3. ? 操作符传播错误：");
    fn read_first_line(path: &str) -> Result<String, std::io::Error> {
        let content = fs::read_to_string(path)?;
        let first_line = content.lines().next().unwrap_or("").to_string();
        Ok(first_line)
    }

    match read_first_line("test.txt") {
        Ok(line) => println!("  第一行：{}", line),
        Err(e) => println!("  读取失败：{}", e),
    }

    // 4. 自定义错误
    println!("\n4. 错误类型总结：");
    println!("  - std::io::Error: IO 错误");
    println!("  - ParseIntError: 解析错误");
    println!("  - 自定义枚举：业务错误");
    println!("  - anyhow::Error: 应用层错误");
    println!("  - thiserror: 库层错误");

    // 5. 最佳实践
    println!("\n5. 最佳实践：");
    println!("  - 库用 thiserror，应用用 anyhow");
    println!("  - 避免 unwrap()，使用 expect()");
    println!("  - 使用 ? 简化错误传播");
    println!("  - 提供有意义的错误信息");

    println!("\n错误处理章节完成！");
    Ok(())
}
