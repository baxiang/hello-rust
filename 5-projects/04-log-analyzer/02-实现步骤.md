## 第一步：创建项目

```bash
cargo new log-analyzer
cd log-analyzer
```

### Cargo.toml

```toml
[package]
name = "log-analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
regex = "1"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
colored = "2"
tokio = { version = "1", features = ["full"] }
notify = "6"  # 文件监控
plotters = "0.3"  # 图表生成
```






## 第二步：日志模型

### src/models/mod.rs

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

/// 单条日志记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 日志级别
    pub level: LogLevel,
    /// 来源
    pub source: String,
    /// 消息内容
    pub message: String,
    /// 附加字段
    pub fields: HashMap<String, String>,
}

/// 日志统计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStats {
    /// 总条数
    pub total: usize,
    /// 各级别统计
    pub by_level: HashMap<LogLevel, usize>,
    /// 各来源统计
    pub by_source: HashMap<String, usize>,
    /// 时间范围
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
    /// 错误列表
    pub errors: Vec<LogEntry>,
}
```






## 第三步：日志解析器

### src/parser/common.rs

```rust
use chrono::{DateTime, Utc};
use regex::Regex;
use crate::models::{LogEntry, LogLevel};
use std::collections::HashMap;

/// 通用日志格式解析器
pub struct CommonLogParser {
    pattern: Regex,
}

impl CommonLogParser {
    pub fn new() -> Self {
        // 格式: 2024-01-15 10:30:00 [INFO] source: message
        let pattern = Regex::new(
            r"(\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2})\s+\[(\w+)\]\s+(\w+):\s+(.*)"
        ).unwrap();
        
        Self { pattern }
    }
    
    pub fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let caps = self.pattern.captures(line)?;
        
        let timestamp = DateTime::parse_from_str(
            &caps[1], "%Y-%m-%d %H:%M:%S"
        )
        .ok()?
        .with_timezone(&Utc);
        
        let level = match &caps[2] {
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "FATAL" => LogLevel::Fatal,
            _ => LogLevel::Info,
        };
        
        Some(LogEntry {
            timestamp,
            level,
            source: caps[3].to_string(),
            message: caps[4].to_string(),
            fields: HashMap::new(),
        })
    }
    
    pub fn parse_file(&self, content: &str) -> Vec<LogEntry> {
        content
            .lines()
            .filter_map(|line| self.parse_line(line))
            .collect()
    }
}
```

### src/parser/nginx.rs

```rust
use chrono::{DateTime, Utc};
use regex::Regex;
use crate::models::{LogEntry, LogLevel};
use std::collections::HashMap;

/// Nginx 日志解析器
pub struct NginxLogParser {
    pattern: Regex,
}

impl NginxLogParser {
    pub fn new() -> Self {
        // Nginx 默认格式
        // 192.168.1.1 - - [15/Jan/2024:10:30:00 +0000] "GET /path HTTP/1.1" 200 1234
        let pattern = Regex::new(
            r"(\S+)\s+-\s+-\s+\[([^\]]+)\]\s+\"(\S+)\s+(\S+)\s+(\S+)\"\s+(\d+)\s+(\d+)"
        ).unwrap();
        
        Self { pattern }
    }
    
    pub fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let caps = self.pattern.captures(line)?;
        
        let timestamp = DateTime::parse_from_str(
            &caps[2], "%d/%b/%Y:%H:%M:%S %z"
        )
        .ok()?
        .with_timezone(&Utc);
        
        let status: u16 = caps[6].parse().ok()?;
        let level = if status >= 500 {
            LogLevel::Error
        } else if status >= 400 {
            LogLevel::Warn
        } else {
            LogLevel::Info
        };
        
        let mut fields = HashMap::new();
        fields.insert("ip".to_string(), caps[1].to_string());
        fields.insert("method".to_string(), caps[3].to_string());
        fields.insert("path".to_string(), caps[4].to_string());
        fields.insert("status".to_string(), caps[6].to_string());
        fields.insert("size".to_string(), caps[7].to_string());
        
        Some(LogEntry {
            timestamp,
            level,
            source: "nginx".to_string(),
            message: format!("{} {} {}", caps[3], caps[4], caps[6]),
            fields,
        })
    }
}
```

### src/parser/json.rs

```rust
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::models::{LogEntry, LogLevel};
use std::collections::HashMap;

/// JSON 日志解析器
pub struct JsonLogParser;

#[derive(Deserialize)]
struct JsonLog {
    timestamp: String,
    level: String,
    source: String,
    message: String,
    #[serde(default)]
    fields: HashMap<String, serde_json::Value>,
}

impl JsonLogParser {
    pub fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let json: JsonLog = serde_json::from_str(line).ok()?;
        
        let timestamp = DateTime::parse_from_rfc3339(&json.timestamp)
            .ok()?
            .with_timezone(&Utc);
        
        let level = match json.level.to_uppercase().as_str() {
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "FATAL" => LogLevel::Fatal,
            _ => LogLevel::Info,
        };
        
        let fields: HashMap<String, String> = json.fields
            .into_iter()
            .filter_map(|(k, v)| {
                Some((k, v.to_string()))
            })
            .collect();
        
        Some(LogEntry {
            timestamp,
            level,
            source: json.source,
            message: json.message,
            fields,
        })
    }
}
```






## 第四步：统计分析

### src/analyzer/stats.rs

```rust
use chrono::{DateTime, Utc};
use crate::models::{LogEntry, LogLevel, LogStats};
use std::collections::HashMap;

/// 统计分析器
pub struct StatsAnalyzer;

impl StatsAnalyzer {
    /// 计算统计信息
    pub fn analyze(entries: &[LogEntry]) -> LogStats {
        let total = entries.len();
        
        // 按级别统计
        let by_level: HashMap<LogLevel, usize> = entries
            .iter()
            .fold(HashMap::new(), |mut acc, entry| {
                *acc.entry(entry.level).or_insert(0) += 1;
                acc
            });
        
        // 按来源统计
        let by_source: HashMap<String, usize> = entries
            .iter()
            .fold(HashMap::new(), |mut acc, entry| {
                *acc.entry(entry.source.clone()).or_insert(0) += 1;
                acc
            });
        
        // 时间范围
        let time_range = if entries.is_empty() {
            (Utc::now(), Utc::now())
        } else {
            let min = entries.iter().map(|e| e.timestamp).min().unwrap();
            let max = entries.iter().map(|e| e.timestamp).max().unwrap();
            (min, max)
        };
        
        // 错误列表
        let errors: Vec<LogEntry> = entries
            .iter()
            .filter(|e| e.level == LogLevel::Error || e.level == LogLevel::Fatal)
            .cloned()
            .collect();
        
        LogStats {
            total,
            by_level,
            by_source,
            time_range,
            errors,
        }
    }
    
    /// 按时间段聚合
    pub fn aggregate_by_hour(entries: &[LogEntry]) -> HashMap<String, usize> {
        entries
            .iter()
            .fold(HashMap::new(), |mut acc, entry| {
                let hour = entry.timestamp.format("%Y-%m-%d %H:00").to_string();
                *acc.entry(hour).or_insert(0) += 1;
                acc
            })
    }
    
    /// 按错误类型聚合
    pub fn aggregate_errors(entries: &[LogEntry]) -> HashMap<String, usize> {
        entries
            .iter()
            .filter(|e| e.level == LogLevel::Error)
            .fold(HashMap::new(), |mut acc, entry| {
                // 提取错误关键词
                let key = extract_error_key(&entry.message);
                *acc.entry(key).or_insert(0) += 1;
                acc
            })
    }
}

fn extract_error_key(message: &str) -> String {
    // 简化错误消息，提取关键词
    let words: Vec<&str> = message.split_whitespace().take(5).collect();
    words.join(" ")
}
```






## 第五步：过滤器

### src/analyzer/filter.rs

```rust
use chrono::{DateTime, Utc};
use crate::models::{LogEntry, LogLevel};

/// 过滤条件
pub struct Filter {
    /// 时间范围
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// 日志级别
    pub levels: Vec<LogLevel>,
    /// 来源
    pub sources: Vec<String>,
    /// 消息关键词
    pub keywords: Vec<String>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            time_range: None,
            levels: Vec::new(),
            sources: Vec::new(),
            keywords: Vec::new(),
        }
    }
    
    /// 设置时间范围
    pub fn with_time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.time_range = Some((start, end));
        self
    }
    
    /// 设置日志级别
    pub fn with_levels(mut self, levels: Vec<LogLevel>) -> Self {
        self.levels = levels;
        self
    }
    
    /// 设置来源
    pub fn with_sources(mut self, sources: Vec<String>) -> Self {
        self.sources = sources;
        self
    }
    
    /// 设置关键词
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }
    
    /// 应用过滤
    pub fn apply(&self, entries: &[LogEntry]) -> Vec<LogEntry> {
        entries
            .iter()
            .filter(|entry| self.matches(entry))
            .cloned()
            .collect()
    }
    
    fn matches(&self, entry: &LogEntry) -> bool {
        // 时间范围检查
        if let Some((start, end)) = self.time_range {
            if entry.timestamp < start || entry.timestamp > end {
                return false;
            }
        }
        
        // 级别检查
        if !self.levels.is_empty() && !self.levels.contains(&entry.level) {
            return false;
        }
        
        // 来源检查
        if !self.sources.is_empty() && !self.sources.contains(&entry.source) {
            return false;
        }
        
        // 关键词检查
        if !self.keywords.is_empty() {
            let has_keyword = self.keywords.iter().any(|k| {
                entry.message.contains(k)
            });
            if !has_keyword {
                return false;
            }
        }
        
        true
    }
}
```







