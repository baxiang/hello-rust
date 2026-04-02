## 第七步：CLI 入口

### src/main.rs

```rust
use clap::Parser;
use anyhow::Result;
use std::fs;
use chrono::{DateTime, Utc};

mod models;
mod parser;
mod analyzer;
mod report;

#[derive(Parser, Debug)]
#[command(name = "log-analyzer")]
#[command(about = "日志分析工具")]
struct Args {
    /// 日志文件路径
    #[arg(short, long)]
    file: String,
    
    /// 日志格式 (common, nginx, json)
    #[arg(short, long, default_value = "common")]
    format: String,
    
    /// 过滤级别
    #[arg(short = 'l', long)]
    level: Option<String>,
    
    /// 过滤关键词
    #[arg(short = 'k', long)]
    keyword: Option<String>,
    
    /// 输出报告文件
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // 读取文件
    let content = fs::read_to_string(&args.file)?;
    
    // 解析日志
    let entries = match args.format.as_str() {
        "common" => parser::common::CommonLogParser::new().parse_file(&content),
        "nginx" => parser::nginx::NginxLogParser::new().parse_file(&content),
        "json" => content.lines()
            .filter_map(|line| parser::json::JsonLogParser.parse_line(line))
            .collect(),
        _ => return Err(anyhow::anyhow!("不支持的格式: {}", args.format)),
    };
    
    // 应用过滤
    let filter = analyzer::filter::Filter::new();
    let filtered = filter.apply(&entries);
    
    // 统计分析
    let stats = analyzer::stats::StatsAnalyzer::analyze(&filtered);
    
    // 生成报告
    let report = report::text::TextReport::generate(&stats);
    
    // 输出
    if let Some(output_path) = args.output {
        fs::write(&output_path, &report)?;
        println!("报告已保存到: {}", output_path);
    } else {
        println!("{}", report);
    }
    
    Ok(())
}
```

---

---

## 下一步

[运行效果](../08-运行效果.md)