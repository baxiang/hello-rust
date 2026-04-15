//! # 示例：错误处理概述
//!
//! 对应章节：01-错误处理概述
//! 运行：cargo run --example 01-error-overview

use std::fs;

fn main() {
    // ✅ 正确示例：panic! 演示
    println!("=== 错误处理概述 ===\n");

    // 可恢复错误：Result
    let greeting_file = fs::read_to_string("hello.txt");
    match greeting_file {
        Ok(content) => println!("文件内容：{}", content),
        Err(e) => println!("文件读取失败：{}", e),
    }

    // ✅ 正确示例：不可恢复错误
    // panic!("这是一个演示 panic");  // 取消注释测试

    // ✅ 正确示例：Result 传播
    fn read_config() -> Result<String, std::io::Error> {
        fs::read_to_string("config.toml")
    }

    match read_config() {
        Ok(config) => println!("配置：{}", config),
        Err(e) => println!("配置读取失败：{}", e),
    }

    // 尝试修改：
    // 1. 尝试读取一个存在的文件
    // 2. 使用 ? 操作符简化错误处理
    // 3. 思考：panic 和 Result 的区别？

    println!("\n错误处理是 Rust 安全性的核心！");
}
