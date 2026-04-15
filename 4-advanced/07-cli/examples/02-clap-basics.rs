//! # 示例：CLI 基础
//!
//! 对应章节：02-Clap基础
//! 运行：cargo run --example 02-clap-basics

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "demo", about = "CLI 演示")]
struct Args {
    /// 用户名
    #[arg(short, long)]
    name: Option<String>,

    /// 重复次数
    #[arg(short, long, default_value = "1")]
    count: u32,
}

fn main() {
    let args = Args::parse();

    let name = args.name.unwrap_or_else(|| "World".to_string());
    for _ in 0..args.count {
        println!("Hello, {}!", name);
    }
}
