## 第一步：创建项目

```bash
cargo new minigrep
cd minigrep
```

### Cargo.toml

```toml
[package]
name = "minigrep"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
regex = "1"
walkdir = "2"
colored = "2"
anyhow = "1"
ignore = "0.4"  # 支持 .gitignore

[dev-dependencies]
tempfile = "3"
```






## 第二步：搜索模块

### src/search.rs

```rust
use regex::Regex;
use std::fs;
use std::path::Path;

/// 搜索配置
pub struct SearchConfig {
    /// 搜索模式（正则表达式）
    pub pattern: Regex,
    /// 是否忽略大小写
    pub ignore_case: bool,
    /// 是否显示行号
    pub show_line_number: bool,
    /// 上下文行数
    pub context_lines: usize,
    /// 是否只显示文件名
    pub files_only: bool,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 文件路径
    pub file: String,
    /// 行号
    pub line_number: usize,
    /// 行内容
    pub content: String,
    /// 匹配位置
    pub matches: Vec<(usize, usize)>,
}

/// 在单个文件中搜索
pub fn search_file(path: &Path, config: &SearchConfig) -> anyhow::Result<Vec<SearchResult>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();
    
    for (idx, line) in lines.iter().enumerate() {
        let line_number = idx + 1;
        
        // 查找所有匹配位置
        let matches: Vec<(usize, usize)> = config.pattern
            .find_iter(line)
            .map(|m| (m.start(), m.end()))
            .collect();
        
        if !matches.is_empty() {
            results.push(SearchResult {
                file: path.display().to_string(),
                line_number,
                content: line.to_string(),
                matches,
            });
        }
    }
    
    Ok(results)
}

/// 搜索并返回带上下文的结果
pub fn search_with_context(
    path: &Path,
    config: &SearchConfig,
) -> anyhow::Result<Vec<(usize, String, bool)>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();
    
    for (idx, line) in lines.iter().enumerate() {
        let line_number = idx + 1;
        
        let has_match = config.pattern.is_match(line);
        
        if has_match {
            // 添加前面的上下文行
            if config.context_lines > 0 {
                let start = idx.saturating_sub(config.context_lines);
                for i in start..idx {
                    results.push((i + 1, lines[i].to_string(), false));
                }
            }
            
            // 添加匹配行
            results.push((line_number, line.to_string(), true));
            
            // 添加后面的上下文行
            if config.context_lines > 0 {
                let end = (idx + config.context_lines + 1).min(lines.len());
                for i in (idx + 1)..end {
                    results.push((i + 1, lines[i].to_string(), false));
                }
            }
            
            // 添加分隔符
            results.push((0, "--".to_string(), false));
        }
    }
    
    // 移除最后的分隔符
    if results.last().map(|r| r.1 == "--") == Some(true) {
        results.pop();
    }
    
    Ok(results)
}
```






## 第三步：文件遍历模块

### src/walker.rs

```rust
use ignore::WalkBuilder;
use std::path::Path;

/// 文件遍历配置
pub struct WalkerConfig {
    /// 是否递归搜索
    pub recursive: bool,
    /// 是否跟随符号链接
    pub follow_links: bool,
    /// 文件扩展名过滤
    pub extensions: Vec<String>,
    /// 最大深度
    pub max_depth: Option<usize>,
}

/// 遍历目录获取文件列表
pub fn walk_directory(
    path: &Path,
    config: &WalkerConfig,
) -> Vec<std::path::PathBuf> {
    let mut builder = WalkBuilder::new(path);
    
    builder
        .follow_links(config.follow_links)
        .max_depth(if config.recursive {
            config.max_depth
        } else {
            Some(1)
        });
    
    // 添加扩展名过滤
    for ext in &config.extensions {
        builder.add_filter(|entry| {
            entry.path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == ext)
                .unwrap_or(false)
        });
    }
    
    builder
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
```






## 第四步：输出格式化

### src/output.rs

```rust
use colored::Colorize;
use crate::search::SearchResult;

/// 格式化单个搜索结果
pub fn format_result(result: &SearchResult, show_line_number: bool) -> String {
    let file = result.file.blue();
    
    if show_line_number {
        let line_num = result.line_number.to_string().yellow();
        format!("{}:{}: {}", file, line_num, highlight_matches(result))
    } else {
        format!("{}: {}", file, highlight_matches(result))
    }
}

/// 高亮匹配内容
fn highlight_matches(result: &SearchResult) -> String {
    let mut output = result.content.clone();
    
    // 从后往前替换，避免位置偏移
    let matches_sorted: Vec<(usize, usize)> = result.matches
        .iter()
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .copied()
        .collect();
    
    for (start, end) in matches_sorted {
        let matched = &output[start..end];
        let highlighted = matched.red().bold().to_string();
        output.replace_range(start..end, &highlighted);
    }
    
    output
}

/// 打印搜索结果
pub fn print_results(results: &[SearchResult], config: &crate::search::SearchConfig) {
    if results.is_empty() {
        println!("未找到匹配内容");
        return;
    }
    
    // 按文件分组
    let mut current_file = String::new();
    
    for result in results {
        if result.file != current_file {
            if !current_file.is_empty() {
                println!();
            }
            current_file = result.file.clone();
            println!("{}", result.file.blue().bold());
        }
        
        println!("{}", format_result(result, config.show_line_number));
    }
    
    println!();
    println!("找到 {} 个匹配", results.len());
}
```






## 第五步：CLI 入口

### src/main.rs

```rust
use clap::Parser;
use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;

mod search;
mod walker;
mod output;

#[derive(Parser, Debug)]
#[command(name = "minigrep")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "文件内容搜索工具", long_about = None)]
struct Args {
    /// 搜索模式（支持正则表达式）
    pattern: String,
    
    /// 搜索路径
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    
    /// 忽略大小写
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
    
    /// 显示行号
    #[arg(short = 'n', long, default_value_t = true)]
    line_number: bool,
    
    /// 递归搜索
    #[arg(short = 'r', long, default_value_t = false)]
    recursive: bool,
    
    /// 上下文行数
    #[arg(short = 'C', long, default_value_t = 0)]
    context: usize,
    
    /// 只显示文件名
    #[arg(short = 'l', long, default_value_t = false)]
    files_only: bool,
    
    /// 文件扩展名过滤
    #[arg(short = 'e', long)]
    extension: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // 构建正则表达式
    let pattern = if args.ignore_case {
        Regex::new(&format!("(?i){}", args.pattern))?
    } else {
        Regex::new(&args.pattern)?
    };
    
    // 搜索配置
    let search_config = search::SearchConfig {
        pattern,
        ignore_case: args.ignore_case,
        show_line_number: args.line_number,
        context_lines: args.context,
        files_only: args.files_only,
    };
    
    // 遍历配置
    let walker_config = walker::WalkerConfig {
        recursive: args.recursive,
        follow_links: false,
        extensions: args.extension.map(|e| vec![e]).unwrap_or_default(),
        max_depth: None,
    };
    
    // 遍历文件
    let files = walker::walk_directory(&args.path, &walker_config);
    
    // 搜索每个文件
    let mut all_results = Vec::new();
    for file in files {
        if args.context > 0 {
            let results = search::search_with_context(&file, &search_config)?;
            for (line_num, content, is_match) in results {
                if is_match {
                    println!("{}:{}: {}", 
                        file.display().to_string().blue(),
                        line_num.to_string().yellow(),
                        content.red().bold()
                    );
                } else if line_num == 0 {
                    println!("{}", content.dimmed());
                } else {
                    println!("{}-{}- {}", 
                        file.display().to_string().blue(),
                        line_num.to_string().dimmed(),
                        content.dimmed()
                    );
                }
            }
        } else {
            let results = search::search_file(&file, &search_config)?;
            all_results.extend(results);
        }
    }
    
    if args.context == 0 {
        output::print_results(&all_results, &search_config);
    }
    
    Ok(())
}
```







