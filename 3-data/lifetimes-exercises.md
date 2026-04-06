# 第 17 章：生命周期练习题

## 基础练习（1-5）

### 练习 1：悬垂引用诊断

分析以下代码的错误原因并修复：

```rust
// ❌ 错误代码
fn get_string() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let result = get_string();
    println!("{}", result);
}
```

**要求：**
- 解释编译错误信息
- 提供至少两种修复方案
- 比较不同方案的优劣

**预期错误：**
```
error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:2:5
  |
2 |     &s
  |     ^^ returns a reference to data owned by the current function
```

---

### 练习 2：生命周期标注

为以下函数添加正确的生命周期标注：

```rust
// ❌ 缺少生命周期标注
fn longest_string(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn first_char(s: &str) -> &str {
    &s[0..1]
}

fn combine(s1: &str, s2: &str, delimiter: &str) -> String {
    format!("{}{}{}", s1, delimiter, s2)
}
```

**要求：**
- 为每个函数添加必要的生命周期标注
- 解释为什么某些函数不需要标注
- 绘制函数调用的内存布局图

---

### 练习 3：结构体生命周期

修复以下结构体的生命周期错误：

```rust
// ❌ 错误结构体
struct Document {
    title: &str,
    content: &str,
}

impl Document {
    fn new(title: &str, content: &str) -> Document {
        Document { title, content }
    }
    
    fn get_title(&self) -> &str {
        self.title
    }
    
    fn get_excerpt(&self, start: usize, end: usize) -> &str {
        &self.content[start..end]
    }
}
```

**要求：**
- 添加正确的生命周期标注
- 实现 `new`、`get_title`、`get_excerpt` 方法
- 提供测试代码验证生命周期正确性

---

### 练习 4：生命周期省略规则应用

判断以下函数是否可以省略生命周期标注，并写出编译器推断的结果：

```rust
fn function1(x: &str) -> &str;
fn function2(x: &str, y: &str) -> &str;
fn function3(x: &str, y: &str);
fn function4(&self) -> &str;
fn function5(&self, x: &str) -> &str;
fn function6(&self, x: &str, y: &str) -> &str;
fn function7(&mut self, x: &str) -> &mut str;
```

**要求：**
- 应用三条省略规则逐个分析
- 写出编译器推断的完整签名
- 标注哪些需要显式标注生命周期

---

### 练习 5：生命周期约束

实现以下带生命周期约束的函数：

```rust
// 要求：返回两个字符串中第一个不为空的
// 如果都为空，返回第一个参数
fn first_non_empty<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 实现
}

// 要求：返回长的字符串，生命周期约束为 'a: 'b
fn longer_with_constraint<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str
where
    'a: 'b,
{
    // 实现
}

// 测试代码
fn main() {
    let long = String::from("long string");
    {
        let short = String::from("short");
        let result = longer_with_constraint(&long, &short);
        println!("{}", result);
    }
    println!("{}", long);  // long 仍然有效
}
```

---

## 中级练习（6-10）

### 练习 6：多生命周期结构体

实现一个包含多个生命周期参数的结构体：

```rust
// 要求：实现 TextComparison 结构体
// 包含两个不同生命周期的文本引用
struct TextComparison<'a, 'b> {
    original: &'a str,
    modified: &'b str,
}

impl<'a, 'b> TextComparison<'a, 'b> {
    // 实现以下方法：
    fn new(original: &'a str, modified: &'b str) -> Self;
    fn get_original(&self) -> &'a str;
    fn get_modified(&self) -> &'b str;
    fn compare(&self) -> ComparisonResult;
}

enum ComparisonResult {
    Same,
    Different { added: String, removed: String },
}

fn main() {
    let original = String::from("Hello World");
    {
        let modified = String::from("Hello Rust");
        let comparison = TextComparison::new(&original, &modified);
        
        match comparison.compare() {
            ComparisonResult::Same => println!("文本相同"),
            ComparisonResult::Different { added, removed } => {
                println!("添加: {}", added);
                println!("删除: {}", removed);
            }
        }
    }
    println!("{}", original);  // original 仍然有效
}
```

---

### 练习 7：零拷贝解析器

实现一个零拷贝的 CSV 解析器：

```rust
// 要求：利用生命周期实现零拷贝 CSV 解析
struct CsvParser<'a> {
    input: &'a str,
    delimiter: char,
}

impl<'a> CsvParser<'a> {
    fn new(input: &'a str, delimiter: char) -> Self;
    fn parse_row(&mut self) -> Option<Vec<&'a str>>;
    fn remaining(&self) -> &'a str;
}

fn main() {
    let csv_data = "name,age,city\nAlice,30,Beijing\nBob,25,Shanghai";
    let mut parser = CsvParser::new(csv_data, ',');
    
    while let Some(row) = parser.parse_row() {
        println!("Row: {:?}", row);
    }
    
    // 所有解析结果都引用原始 csv_data，零拷贝
}
```

**要求：**
- 实现解析逻辑
- 确保所有返回值都引用原始数据
- 绘制内存布局图说明零拷贝优势

---

### 练习 8：'static 生命周期应用

实现一个全局配置系统：

```rust
// 要求：使用 'static 实现全局配置
use std::sync::OnceLock;

struct Config {
    version: &'static str,
    environment: &'static str,
    max_connections: usize,
}

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

fn init_config() -> &'static Config {
    GLOBAL_CONFIG.get_or_init(|| {
        Config {
            version: "1.0.0",
            environment: "production",
            max_connections: 100,
        }
    })
}

fn get_config() -> &'static Config {
    GLOBAL_CONFIG.get().expect("Config not initialized")
}

// 实现以下功能：
fn update_max_connections(new_value: usize);
fn print_config();

fn main() {
    init_config();
    print_config();
    update_max_connections(200);
    print_config();
}
```

---

### 练习 9：Trait 中的生命周期

实现带生命周期参数的 Trait：

```rust
// 要求：定义和使用带生命周期的 Trait
trait Parser<'a> {
    fn parse(&self, input: &'a str) -> &'a str;
    fn parse_all(&self, inputs: Vec<&'a str>) -> Vec<&'a str>;
}

struct SimpleParser;

impl<'a> Parser<'a> for SimpleParser {
    // 实现方法
}

struct TrimParser;

impl<'a> Parser<'a> for TrimParser {
    // 实现方法：返回去除首尾空格后的切片
}

fn main() {
    let parsers: Vec<Box<dyn for<'a> Parser<'a>>> = vec![
        Box::new(SimpleParser),
        Box::new(TrimParser),
    ];
    
    let input = "  hello world  ";
    for parser in &parsers {
        println!("{}", parser.parse(input));
    }
}
```

---

### 练习 10：生命周期与闭包

理解闭包与生命周期的交互：

```rust
// 练习：实现以下函数
fn create_processor<'a>(data: &'a str) -> impl Fn(usize) -> &'a str {
    // 返回一个闭包，根据索引返回 data 的切片
    // 例如：processor(5) 返回 data[5..]
}

fn create_multi_processor<'a>(data: &'a str) -> impl Fn(&str) -> &'a str {
    // 返回一个闭包，根据分隔符分割 data
    // 例如：processor(",") 返回逗号前的部分
}

fn main() {
    let text = String::from("hello,world,test");
    
    let index_processor = create_processor(&text);
    println!("{}", index_processor(5));  // "world,test"
    
    let split_processor = create_multi_processor(&text);
    println!("{}", split_processor(","));  // "hello"
}
```

---

## 高级练习（11-15）

### 练习 11：生命周期协变和逆变

理解生命周期协变的应用：

```rust
// 练习：实现以下场景
fn store_short<'short>(s: &'short str) -> Box<dyn Fn() -> &'short str + 'short> {
    // 存储短生命周期的引用
}

fn use_long_as_short<'long, 'short>(s: &'long str, f: fn(&'short str))
where
    'long: 'short,
{
    // 长生命周期可以用于短生命周期的场景
    f(s);  // ✅ 正确
}

// 实现测试代码
fn main() {
    let long_string = String::from("long lived");
    
    // 使用协变特性
    fn print_short(s: &str) {
        println!("{}", s);
    }
    
    use_long_as_short(&long_string, print_short);
}
```

---

### 练习 12：高阶 Trait 约束

实现使用 for<'a> 的回调系统：

```rust
// 练习：实现高阶 Trait 约束的回调系统
trait Callback {
    fn call(&self, data: &str) -> &str;
}

// 使用 for<'a> 约束
fn register_callback<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str + 'static,
{
    // 注册回调
}

// 实现：
struct CallbackRegistry {
    callbacks: Vec<Box<dyn for<'a> Fn(&'a str) -> &'a str>>,
}

impl CallbackRegistry {
    fn new() -> Self;
    fn add<F>(&mut self, f: F) where F: for<'a> Fn(&'a str) -> &'a str + 'static;
    fn process(&self, input: &str) -> Vec<&str>;
}

fn main() {
    let registry = CallbackRegistry::new();
    registry.add(|s| s);
    registry.add(|s| &s[0..5]);
    
    let input = String::from("hello world");
    let results = registry.process(&input);
    println!("Results: {:?}", results);
}
```

---

### 练习 13：Web 请求处理模拟

实现简化的 Web 请求处理器：

```rust
// 练习：实现请求-响应生命周期管理
struct Request<'a> {
    method: &'a str,
    path: &'a str,
    body: &'a str,
}

struct Response<'a> {
    status: u16,
    headers: Vec<(&'a str, &'a str)>,
    body: &'a str,
}

trait Handler<'a> {
    fn handle(&self, req: &Request<'a>) -> Response<'a>;
}

struct Router<'a> {
    routes: HashMap<&'a str, Box<dyn Handler<'a>>>,
}

impl<'a> Router<'a> {
    fn new() -> Self;
    fn add_route<H>(&mut self, path: &'a str, handler: H)
    where
        H: Handler<'a> + 'static;
    fn route(&self, req: &Request<'a>) -> Option<Response<'a>>;
}

// 实现具体处理器
struct HomeHandler;
struct ApiHandler;

fn main() {
    let mut router = Router::new();
    router.add_route("/", HomeHandler);
    router.add_route("/api", ApiHandler);
    
    let req = Request {
        method: "GET",
        path: "/api",
        body: "",
    };
    
    let resp = router.route(&req);
    // 处理响应
}
```

---

### 练习 14：数据库连接池模拟

实现安全的连接池生命周期管理：

```rust
// 练习：实现连接池
struct Connection {
    id: usize,
    in_use: bool,
}

struct ConnectionPool {
    pool: Vec<Connection>,
}

impl ConnectionPool {
    fn new(size: usize) -> Self;
    
    // ❌ 错误方式：返回 Connection 的引用
    // fn get(&self) -> &Connection { ... }
    
    // ✅ 正确方式：使用回调
    fn with_connection<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Connection) -> R;
    
    fn available_count(&self) -> usize;
}

fn main() {
    let pool = ConnectionPool::new(5);
    
    pool.with_connection(|conn| {
        println!("Using connection {}", conn.id);
        // 执行数据库操作
    });
    
    println!("Available: {}", pool.available_count());
}
```

**要求：**
- 解释为什么直接返回引用有问题
- 使用回调方式管理生命周期
- 绘制生命周期关系图

---

### 练习 15：综合案例 - 文本处理系统

实现完整的文本处理系统：

```rust
// 综合练习：实现文本处理系统
struct TextProcessor<'a> {
    source: &'a str,
}

impl<'a> TextProcessor<'a> {
    fn new(source: &'a str) -> Self;
    fn extract(&self, start: usize, end: usize) -> &'a str;
    fn find(&self, pattern: &str) -> Option<usize>;
    fn replace(&self, from: &str, to: &str) -> String;
}

struct ProcessingPipeline<'a> {
    processors: Vec<Box<dyn for<'b> Fn(&'b str) -> &'b str>>,
    source: &'a str,
}

impl<'a> ProcessingPipeline<'a> {
    fn new(source: &'a str) -> Self;
    fn add_processor<F>(&mut self, f: F)
    where
        F: for<'b> Fn(&'b str) -> &'b str + 'static;
    fn run(&self) -> &'a str;
}

fn main() {
    let text = String::from("Hello World, Rust Programming");
    
    // 使用 TextProcessor
    let processor = TextProcessor::new(&text);
    let excerpt = processor.extract(0, 11);
    println!("Excerpt: {}", excerpt);
    
    // 使用 Pipeline
    let mut pipeline = ProcessingPipeline::new(&text);
    pipeline.add_processor(|s| &s[0..5]);
    pipeline.add_processor(|s| s.trim());
    
    let result = pipeline.run();
    println!("Pipeline result: {}", result);
}
```

**要求：**
- 综合应用所有生命周期知识
- 理解零拷贝的优势
- 管理复杂生命周期关系

---

## 练习题答案要点

### 答案 1：悬垂引用修复

```rust
// ✅ 方案 1：返回所有权
fn get_string() -> String {
    String::from("hello")
}

// ✅ 方案 2：接收输入参数
fn get_string_ref(s: &String) -> &String {
    s
}

// ✅ 方案 3：返回 'static
fn get_static() -> &'static str {
    "hello"
}
```

### 答案 2：生命周期标注

```rust
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn first_char(s: &str) -> &str {  // 省略规则自动推断
    &s[0..1]
}

fn combine(s1: &str, s2: &str, delimiter: &str) -> String {  // 无返回引用，无需标注
    format!("{}{}{}", s1, delimiter, s2)
}
```

### 答案 3：结构体生命周期

```rust
struct Document<'a> {
    title: &'a str,
    content: &'a str,
}

impl<'a> Document<'a> {
    fn new(title: &'a str, content: &'a str) -> Self {
        Document { title, content }
    }
    
    fn get_title(&self) -> &str {  // 省略规则自动推断
        self.title
    }
    
    fn get_excerpt(&self, start: usize, end: usize) -> &str {
        &self.content[start..end]
    }
}
```

---

## 自我评估标准

完成所有练习后，你应该能够：

1. ✅ 准确判断何时需要生命周期标注
2. ✅ 正确应用三条省略规则
3. ✅ 理解生命周期协变和逆变
4. ✅ 使用 for<'a> 高阶约束
5. ✅ 实现零拷贝解析器
6. ✅ 管理 Trait 对象的生命周期
7. ✅ 设计安全的连接池
8. ✅ 处理 Web 请求的生命周期

---

## 下一步学习

完成本章练习后，继续学习：
- 第 18 章：闭包
- 第 19 章：迭代器
- 实战项目：文本解析器