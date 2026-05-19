## 第一步：创建项目

```bash
cargo new kv-store
cd kv-store
```

### Cargo.toml

```toml
[package]
name = "kv-store"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
dashmap = "5"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
thiserror = "1"
bytes = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```






## 第二步：数据类型

### src/store/types.rs

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 存储值类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    /// 字符串
    String(String),
    /// 列表
    List(Vec<String>),
    /// 哈希表
    Hash(std::collections::HashMap<String, String>),
}

/// 存储条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// 值
    pub value: Value,
    /// 过期时间（可选）
    pub expires_at: Option<DateTime<Utc>>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

impl Entry {
    /// 创建新条目
    pub fn new(value: Value) -> Self {
        Self {
            value,
            expires_at: None,
            created_at: Utc::now(),
        }
    }
    
    /// 设置过期时间
    pub fn with_expire(mut self, seconds: u64) -> Self {
        self.expires_at = Some(Utc::now() + chrono::Duration::seconds(seconds as i64));
        self
    }
    
    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

impl Value {
    /// 创建字符串值
    pub fn string(s: impl Into<String>) -> Self {
        Value::String(s.into())
    }
    
    /// 创建列表值
    pub fn list(items: Vec<String>) -> Self {
        Value::List(items)
    }
    
    /// 创建哈希值
    pub fn hash(fields: std::collections::HashMap<String, String>) -> Self {
        Value::Hash(fields)
    }
    
    /// 获取字符串值
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    
    /// 获取列表值
    pub fn as_list(&self) -> Option<&Vec<String>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }
    
    /// 获取哈希值
    pub fn as_hash(&self) -> Option<&std::collections::HashMap<String, String>> {
        match self {
            Value::Hash(h) => Some(h),
            _ => None,
        }
    }
}
```






## 第三步：存储引擎

### src/store/engine.rs

```rust
use dashmap::DashMap;
use std::sync::Arc;
use crate::store::types::{Entry, Value};

/// 存储引擎
pub struct Store {
    /// 数据存储
    data: DashMap<String, Entry>,
}

impl Store {
    /// 创建新存储
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }
    
    /// 设置键值
    pub fn set(&self, key: String, value: Value) {
        let entry = Entry::new(value);
        self.data.insert(key, entry);
    }
    
    /// 设置键值带过期时间
    pub fn setex(&self, key: String, value: Value, seconds: u64) {
        let entry = Entry::new(value).with_expire(seconds);
        self.data.insert(key, entry);
    }
    
    /// 获取值
    pub fn get(&self, key: &str) -> Option<Value> {
        self.data.get(key).and_then(|entry| {
            if entry.is_expired() {
                // 过期则删除
                self.data.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        })
    }
    
    /// 删除键
    pub fn del(&self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
    
    /// 检查键是否存在
    pub fn exists(&self, key: &str) -> bool {
        self.data.get(key).map(|e| !e.is_expired()).unwrap_or(false)
    }
    
    /// 获取所有键
    pub fn keys(&self) -> Vec<String> {
        self.data
            .iter()
            .filter(|entry| !entry.is_expired())
            .map(|entry| entry.key().clone())
            .collect()
    }
    
    /// 清空所有数据
    pub fn flush(&self) {
        self.data.clear();
    }
    
    /// 获取数据数量
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    // === List 操作 ===
    
    /// 列表左推入
    pub fn lpush(&self, key: String, value: String) -> anyhow::Result<usize> {
        let entry = self.data.get_mut(&key);
        
        match entry {
            Some(mut e) => {
                match &mut e.value {
                    Value::List(list) => {
                        list.insert(0, value);
                        Ok(list.len())
                    }
                    _ => anyhow::bail!("键类型不是列表"),
                }
            }
            None => {
                let list = Value::list(vec![value]);
                self.set(key, list);
                Ok(1)
            }
        }
    }
    
    /// 列表右推入
    pub fn rpush(&self, key: String, value: String) -> anyhow::Result<usize> {
        let entry = self.data.get_mut(&key);
        
        match entry {
            Some(mut e) => {
                match &mut e.value {
                    Value::List(list) => {
                        list.push(value);
                        Ok(list.len())
                    }
                    _ => anyhow::bail!("键类型不是列表"),
                }
            }
            None => {
                let list = Value::list(vec![value]);
                self.set(key, list);
                Ok(1)
            }
        }
    }
    
    /// 列表左弹出
    pub fn lpop(&self, key: &str) -> Option<String> {
        self.data.get_mut(key).and_then(|mut entry| {
            match &mut entry.value {
                Value::List(list) if !list.is_empty() => Some(list.remove(0)),
                _ => None,
            }
        })
    }
    
    /// 列表右弹出
    pub fn rpop(&self, key: &str) -> Option<String> {
        self.data.get_mut(key).and_then(|mut entry| {
            match &mut entry.value {
                Value::List(list) if !list.is_empty() => Some(list.pop()),
                _ => None,
            }
        })
    }
    
    /// 获取列表长度
    pub fn llen(&self, key: &str) -> usize {
        self.get(key)
            .and_then(|v| v.as_list().map(|l| l.len()))
            .unwrap_or(0)
    }
    
    /// 获取列表范围
    pub fn lrange(&self, key: &str, start: usize, end: usize) -> Option<Vec<String>> {
        self.get(key).and_then(|v| {
            v.as_list().map(|list| {
                let end = end.min(list.len());
                if start < end {
                    list[start..end].to_vec()
                } else {
                    vec![]
                }
            })
        })
    }
    
    // === Hash 操作 ===
    
    /// 设置哈希字段
    pub fn hset(&self, key: String, field: String, value: String) -> anyhow::Result<bool> {
        let entry = self.data.get_mut(&key);
        
        match entry {
            Some(mut e) => {
                match &mut e.value {
                    Value::Hash(hash) => {
                        let is_new = !hash.contains_key(&field);
                        hash.insert(field, value);
                        Ok(is_new)
                    }
                    _ => anyhow::bail!("键类型不是哈希"),
                }
            }
            None => {
                let mut hash = std::collections::HashMap::new();
                hash.insert(field, value);
                self.set(key, Value::hash(hash));
                Ok(true)
            }
        }
    }
    
    /// 获取哈希字段
    pub fn hget(&self, key: &str, field: &str) -> Option<String> {
        self.get(key).and_then(|v| {
            v.as_hash().and_then(|h| h.get(field).cloned())
        })
    }
    
    /// 获取所有哈希字段
    pub fn hgetall(&self, key: &str) -> Option<std::collections::HashMap<String, String>> {
        self.get(key).and_then(|v| v.as_hash().cloned())
    }
    
    /// 删除哈希字段
    pub fn hdel(&self, key: &str, field: &str) -> bool {
        self.data.get_mut(key).and_then(|mut entry| {
            match &mut entry.value {
                Value::Hash(hash) => hash.remove(field).is_some(),
                _ => false,
            }
        }).unwrap_or(false)
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
```






## 第四步：命令协议

### src/command/protocol.rs

```rust
use serde::{Deserialize, Serialize};

/// 响应结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    /// 成功（简单字符串）
    Ok(String),
    /// 错误
    Error(String),
    /// 整数
    Integer(i64),
    /// 批量字符串
    Bulk(String),
    /// 数组
    Array(Vec<Response>),
    /// 空
    Nil,
}

impl Response {
    /// 成功响应
    pub fn ok() -> Self {
        Response::Ok("OK".to_string())
    }
    
    /// 错误响应
    pub fn error(msg: impl Into<String>) -> Self {
        Response::Error(msg.into())
    }
    
    /// 整数响应
    pub fn integer(n: i64) -> Self {
        Response::Integer(n)
    }
    
    /// 批量字符串响应
    pub fn bulk(s: impl Into<String>) -> Self {
        Response::Bulk(s.into())
    }
    
    /// 空响应
    pub fn nil() -> Self {
        Response::Nil
    }
    
    /// 数组响应
    pub fn array(items: Vec<Response>) -> Self {
        Response::Array(items)
    }
    
    /// 序列化为字符串
    pub fn to_string(&self) -> String {
        match self {
            Response::Ok(s) => format!("+{}\r\n", s),
            Response::Error(e) => format!("-{}\r\n", e),
            Response::Integer(n) => format!(":{}\r\n", n),
            Response::Bulk(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Response::Nil => "$-1\r\n".to_string(),
            Response::Array(items) => {
                let mut result = format!("*{}\r\n", items.len());
                for item in items {
                    result.push_str(&item.to_string());
                }
                result
            }
        }
    }
}
```






## 第五步：命令解析

### src/command/parser.rs

```rust
use bytes::Bytes;

/// 命令解析器
pub struct CommandParser;

impl CommandParser {
    /// 解析命令
    pub fn parse(input: &[u8]) -> Option<Vec<String>> {
        let input = std::str::from_utf8(input).ok()?;
        
        // 简单协议：命令用空格分隔
        // 例如：SET key value
        let parts: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        if parts.is_empty() {
            None
        } else {
            Some(parts)
        }
    }
    
    /// 解析 RESP 协议（Redis 协议）
    pub fn parse_resp(input: &[u8]) -> Option<Vec<String>> {
        let input = std::str::from_utf8(input).ok()?;
        let lines: Vec<&str> = input.lines().collect();
        
        if lines.is_empty() {
            return None;
        }
        
        // 检查数组标记
        if !lines[0].starts_with('*') {
            return Self::parse(input);
        }
        
        let count: usize = lines[0][1..].parse().ok()?;
        let mut result = Vec::with_capacity(count);
        
        let mut i = 1;
        while i < lines.len() && result.len() < count {
            if lines[i].starts_with('$') {
                let len: usize = lines[i][1..].parse().ok()?;
                i += 1;
                if i < lines.len() {
                    result.push(lines[i].to_string());
                }
            }
            i += 1;
        }
        
        Some(result)
    }
}
```







