## 第六步：爬虫核心

### src/crawler/mod.rs

```rust
use crate::config::Config;
use crate::crawler::{fetcher::Fetcher, parser::HtmlParser, queue::UrlQueue};
use crate::storage::file::FileStorage;
use futures::stream::{self, StreamExt};
use std::sync::Arc;

/// 爬虫结果
#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub url: String,
    pub title: Option<String>,
    pub text: String,
    pub links: Vec<String>,
    pub depth: usize,
}

/// 爬虫
pub struct Crawler {
    fetcher: Fetcher,
    queue: UrlQueue,
    storage: FileStorage,
    config: Config,
}

impl Crawler {
    /// 创建爬虫
    pub fn new(config: Config) -> Self {
        let fetcher = Fetcher::new(&config.crawler);
        let queue = UrlQueue::new();
        let storage = FileStorage::new(&config.storage.output_dir);
        
        Self {
            fetcher,
            queue,
            storage,
            config,
        }
    }
    
    /// 启动爬虫
    pub async fn run(&self, start_url: String) -> anyhow::Result<()> {
        // 添加起始 URL
        self.queue.push(start_url).await;
        
        tracing::info!("开始爬取...");
        
        // 并发爬取
        while !self.queue.is_empty().await {
            let batch: Vec<String> = self.get_batch().await;
            
            if batch.is_empty() {
                break;
            }
            
            // 并发处理
            let results = stream::iter(batch)
                .map(|url| self.crawl_page(url, 0))
                .buffer_unordered(self.config.crawler.max_concurrent)
                .collect::<Vec<_>>()
                .await;
            
            // 处理结果
            for result in results {
                if let Ok(Some(r)) = result {
                    // 保存结果
                    self.save_result(&r)?;
                    
                    // 添加新链接到队列
                    self.queue.push_batch(r.links).await;
                    
                    tracing::info!(
                        "已爬取: {} (标题: {})",
                        r.url,
                        r.title.unwrap_or_default()
                    );
                }
            }
            
            tracing::info!(
                "队列: {}, 已处理: {}",
                self.queue.size().await,
                self.queue.visited_count().await
            );
        }
        
        tracing::info!("爬取完成");
        Ok(())
    }
    
    /// 获取一批 URL
    async fn get_batch(&self) -> Vec<String> {
        let mut batch = Vec::new();
        
        for _ in 0..self.config.crawler.max_concurrent {
            if let Some(url) = self.queue.pop().await {
                batch.push(url);
            } else {
                break;
            }
        }
        
        batch
    }
    
    /// 爬取单个页面
    async fn crawl_page(&self, url: String, depth: usize) -> anyhow::Result<Option<CrawlResult>> {
        // 检查深度
        if depth > self.config.crawler.max_depth {
            return Ok(None);
        }
        
        // 获取网页
        let html = self.fetcher.fetch(&url).await?;
        
        // 解析网页
        let parser = HtmlParser::from_html(&html);
        
        let title = parser.title();
        let text = parser.text();
        let links = parser.links(&url);
        
        Ok(Some(CrawlResult {
            url,
            title,
            text,
            links,
            depth,
        }))
    }
    
    /// 保存结果
    fn save_result(&self, result: &CrawlResult) -> anyhow::Result<()> {
        if self.config.storage.save_text {
            let filename = self.storage.save_text(&result.url, &result.text)?;
            tracing::debug!("保存文本: {}", filename);
        }
        
        Ok(())
    }
}
```






## 第七步：文件存储

### src/storage/file.rs

```rust
use std::fs;
use std::path::PathBuf;

/// 文件存储
pub struct FileStorage {
    output_dir: PathBuf,
}

impl FileStorage {
    /// 创建存储
    pub fn new(output_dir: &str) -> Self {
        let path = PathBuf::from(output_dir);
        
        if !path.exists() {
            fs::create_dir_all(&path).ok();
        }
        
        Self { output_dir: path }
    }
    
    /// 保存文本
    pub fn save_text(&self, url: &str, text: &str) -> anyhow::Result<String> {
        let filename = self.url_to_filename(url);
        let path = self.output_dir.join(&filename);
        
        fs::write(&path, text)?;
        
        Ok(filename)
    }
    
    /// URL 转文件名
    fn url_to_filename(&self, url: &str) -> String {
        url
            .replace("://", "_")
            .replace("/", "_")
            .replace(".", "_")
            .replace(":", "_")
            + ".txt"
    }
    
    /// 保存 JSON
    pub fn save_json<T: serde::Serialize>(&self, filename: &str, data: &T) -> anyhow::Result<()> {
        let path = self.output_dir.join(filename);
        let content = serde_json::to_string_pretty(data)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
```






## 第八步：主入口

### src/main.rs

```rust
use clap::Parser;
use anyhow::Result;

mod config;
mod crawler;
mod storage;

#[derive(Parser, Debug)]
#[command(name = "web-crawler")]
#[command(about = "网络爬虫")]
struct Args {
    /// 起始 URL
    #[arg(short, long)]
    url: String,
    
    /// 最大深度
    #[arg(short = 'd', long, default_value = "2")]
    depth: usize,
    
    /// 并发数
    #[arg(short = 'c', long, default_value = "5")]
    concurrent: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    // 加载配置
    let config = config::Config::load()
        .unwrap_or_else(|_| {
            config::Config {
                crawler: config::CrawlerConfig {
                    max_depth: args.depth,
                    max_concurrent: args.concurrent,
                    timeout_seconds: 30,
                    user_agent: "RustCrawler/0.1".to_string(),
                },
                storage: config::StorageConfig {
                    output_dir: "./output".to_string(),
                    save_html: false,
                    save_text: true,
                },
            }
        });
    
    // 创建爬虫
    let crawler = crawler::Crawler::new(config);
    
    // 运行爬虫
    crawler.run(args.url).await?;
    
    Ok(())
}
```






## 运行效果

```bash
# 爬取单个网站
web-crawler -u https://example.com

# 设置深度和并发
web-crawler -u https://example.com -d 3 -c 10
```

### 输出示例

```
开始爬取...
已爬取: https://example.com (标题: Example Domain)
队列: 5, 已处理: 1
已爬取: https://example.com/about (标题: About)
已爬取: https://example.com/contact (标题: Contact)
队列: 10, 已处理: 3
...
爬取完成
```






## 小结

本项目涵盖：

- ✅ 异步 HTTP 请求
- ✅ HTML 解析
- ✅ CSS 选择器
- ✅ URL 解析与规范化
- ✅ 并发控制
- ✅ 文件存储







