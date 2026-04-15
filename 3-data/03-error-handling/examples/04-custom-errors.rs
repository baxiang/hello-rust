//! # 示例：自定义错误类型
//!
//! 对应章节：04-自定义错误
//! 运行：cargo run --example 04-custom-errors

use std::fmt;
use std::fs;
use std::num::ParseIntError;

// ✅ 正确示例：自定义错误枚举
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误：{}", e),
            AppError::Parse(e) => write!(f, "解析错误：{}", e),
            AppError::NotFound(msg) => write!(f, "未找到：{}", msg),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn process_file(path: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(path)?;
    let number: i32 = content.trim().parse()?;
    if number < 0 {
        return Err(AppError::NotFound("负数不被允许".to_string()));
    }
    Ok(number)
}

fn main() {
    println!("=== 自定义错误类型 ===\n");

    match process_file("data.txt") {
        Ok(n) => println!("处理成功：{}", n),
        Err(e) => println!("处理失败：{}", e),
    }

    // ✅ 正确示例：使用 anyhow 简化
    fn demo_anyhow() -> anyhow::Result<()> {
        let content = fs::read_to_string("config.json")
            .map_err(|e| anyhow::anyhow!("读取配置失败：{}", e))?;
        println!("配置：{}", content);
        Ok(())
    }

    if let Err(e) = demo_anyhow() {
        println!("anyhow 错误：{:?}", e);
    }

    println!("\n自定义错误类型章节完成！");
}
