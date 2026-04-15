//! # 示例：Result
//!
//! 对应章节：03-Result
//! 运行：cargo run --example 03-result

use std::fs;

fn main() {
    println!("=== Result 错误处理 ===\n");

    // ✅ 正确示例：match 处理 Result
    let content = match fs::read_to_string("test.txt") {
        Ok(s) => s,
        Err(e) => {
            println!("读取失败：{}", e);
            String::new()
        }
    };

    // ✅ 正确示例：unwrap_or
    let value = fs::read_to_string("test.txt").unwrap_or_else(|_| "默认内容".to_string());
    println!("内容：{}", value);

    // ✅ 正确示例：? 操作符
    fn read_file() -> Result<String, std::io::Error> {
        let content = fs::read_to_string("test.txt")?;
        Ok(content)
    }

    match read_file() {
        Ok(s) => println!("文件内容：{}", s),
        Err(e) => println!("错误：{}", e),
    }

    // ✅ 正确示例：map 和 and_then
    let num = "42".parse::<i32>().map(|n| n * 2);
    println!("解析并翻倍：{:?}", num);

    // ✅ 正确示例：自定义错误处理
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    println!("10 / 2 = {:?}", divide(10.0, 2.0));
    println!("10 / 0 = {:?}", divide(10.0, 0.0));

    // 尝试修改：
    // 1. 创建自己的 Result 返回函数
    // 2. 使用 ? 操作符简化代码
    // 3. 思考：Result 如何保证错误不被忽略？
}
