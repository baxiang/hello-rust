//! # 示例：完整程序示例
//!
//! 对应章节：03-完整示例
//! 运行：cargo run --example 03-complete-example

fn main() {
    // ✅ 正确示例：完整程序结构
    println!("=== Rust 程序完整示例 ===\n");

    // 1. 变量声明
    let name = "Rust 教程";
    let version = "1.0.0";
    let edition = "2024";

    // 2. 输出信息
    println!("项目名称：{}", name);
    println!("项目版本：{}", version);
    println!("Edition：{}", edition);

    // 3. 简单计算
    let chapter_count = 31;
    let project_count = 9;
    let total_examples = chapter_count + project_count;
    println!("\n章节数：{}", chapter_count);
    println!("项目数：{}", project_count);
    println!("总计：{} 个学习单元", total_examples);

    // 4. 字符串操作
    let greeting = format!("欢迎学习 {}", name);
    println!("\n{}", greeting);

    // 尝试修改：
    // 1. 添加更多变量展示你的信息
    // 2. 使用 format! 创建复杂字符串
    // 3. 尝试使用 println! 的不同格式化选项
}
