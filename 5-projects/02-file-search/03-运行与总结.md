## 运行效果

```bash
# 基本搜索
minigrep "fn main" .

# 忽略大小写
minigrep "error" ./src -i

# 递归搜索
minigrep "TODO" . -r

# 显示上下文
minigrep "struct" ./src -C 3

# 只显示文件名
minigrep "import" . -r -l

# 按扩展名过滤
minigrep "async" . -r -e rs
```

### 输出示例

```
src/main.rs
src/main.rs:10: fn main() -> Result<()> {
src/main.rs:25:     let pattern = Regex::new(&args.pattern)?;
src/main.rs:42:     for file in files {

找到 3 个匹配
```


## 性能优化

### 并行搜索

```rust
use rayon::prelude::*;

// 并行搜索所有文件
let all_results: Vec<SearchResult> = files
    .par_iter()
    .flat_map(|file| search_file(file, &config).unwrap_or_default())
    .collect();
```

### Cargo.toml 添加依赖

```toml
[dependencies]
rayon = "1"
```






## 小结

本项目涵盖：

- ✅ 正则表达式匹配
- ✅ 目录递归遍历
- ✅ 文件读写操作
- ✅ 彩色输出格式化
- ✅ 命令行参数解析
- ✅ 并行处理优化







