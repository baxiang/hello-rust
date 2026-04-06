# 所有权练习题

> 通过实践巩固所有权知识

## 练习说明

本练习集包含：
- 基础题：所有权判断（5题）
- 进阶题：代码填空（5题）
- 挑战题：重构代码（10题）

**建议学习路径：**
1. 完成基础题，理解所有权基本概念
2. 完成进阶题，掌握所有权修复方法
3. 完成挑战题，应用所有权最佳实践

---

## 基础题：所有权判断

### 练习 1：所有权移动判断

**题目：** 判断以下代码哪些变量在最后一行仍然有效。

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    let s4 = s3;
    
    // 问题：哪些变量在这里仍然有效？
    // A. s1
    // B. s2
    // C. s3
    // D. s4
}
```

**答案：**

```
有效变量：D (s4)

解析：
• s1 → 移动到 s2，无效
• s2 → 移动到 s3.clone() 的参数？不，clone() 不移动原值
        s2 移动到 s4，无效
• s3 → clone() 不移动，s3 有效
        但 s3 移动到 s4，无效
• s4 → 从 s3 移动，s4 有效

正确理解：
let s3 = s2.clone();  // s2.clone() 不移动 s2
let s4 = s3;          // s3 移动到 s4

所以：
s1: 无效（移动到 s2）
s2: 无效（移动到 s4）
s3: 无效（移动到 s4）
s4: 有效
```

---

### 练习 2：函数参数所有权

**题目：** 判断以下代码的输出。

```rust
fn take(s: String) {
    println!("take: {}", s);
}

fn borrow(s: &String) {
    println!("borrow: {}", s);
}

fn main() {
    let s = String::from("hello");
    
    take(s);
    // 问题：s 在这里是否有效？
    
    borrow(&s);
    // 问题：这行能执行吗？
}
```

**答案：**

```
❌ 编译错误

错误位置：第 13 行 borrow(&s)
错误信息：s 已移动到 take 函数

解析：
• take(s) 移动所有权到函数
• 函数结束时 s 被释放
• main 中的 s 无效
• borrow(&s) 使用无效变量 → 编译错误

修复方法：
fn main() {
    let s = String::from("hello");
    
    borrow(&s);  // 先借用
    take(s);     // 再移动
}
```

---

### 练习 3：Copy 类型判断

**题目：** 判断以下类型哪些是 Copy。

```rust
fn main() {
    // 类型判断：哪些是 Copy？
    
    // A. i32
    let a = 5;
    let b = a;
    println!("a = {}", a);  // 是否有效？
    
    // B. String
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 = {}", s1);  // 是否有效？
    
    // C. (i32, i32)
    let t1 = (1, 2);
    let t2 = t1;
    println!("t1 = {:?}", t1);  // 是否有效？
    
    // D. (String, i32)
    let t3 = (String::from("hi"), 5);
    let t4 = t3;
    // println!("t3 = {:?}", t3);  // 是否有效？
    
    // E. [i32; 3]
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("arr1 = {:?}", arr1);  // 是否有效？
}
```

**答案：**

```
Copy 类型判断：

A. i32 - ✅ Copy
   a 和 b 都有效

B. String - ❌ 非 Copy
   s1 无效

C. (i32, i32) - ✅ Copy（所有元素 Copy）
   t1 和 t2 都有效

D. (String, i32) - ❌ 非 Copy（String 不是 Copy）
   t3 无效

E. [i32; 3] - ✅ Copy（元素是 Copy）
   arr1 和 arr2 都有效

总结：
• 基本类型（i32, f64, bool, char）都是 Copy
• 元组：所有元素 Copy → 元组 Copy
• 数组：元素 Copy → 数组 Copy
• String, Vec 等堆分配类型不是 Copy
```

---

### 练习 4：作用域判断

**题目：** 判断变量的有效性范围。

```rust
fn main() {
    let outer = String::from("outer");
    
    {
        let inner = String::from("inner");
        println!("inner: {}", inner);
        println!("outer: {}", outer);
    }
    
    // 问题 1：inner 在这里是否有效？
    
    println!("outer: {}", outer);
    
    // 问题 2：outer 在这里是否有效？
}
```

**答案：**

```
变量有效性：

问题 1：inner 无效
• inner 在内层作用域定义
• 离开作用域时被释放
• 外层不能访问

问题 2：outer 有效
• outer 在外层定义
• 仍在作用域内
• 可以使用

规则：
• 内层作用域可以访问外层变量
• 外层作用域不能访问内层变量
• 变量离开作用域自动释放
```

---

### 练习 5：所有权流转追踪

**题目：** 追踪所有权流转路径。

```rust
fn create() -> String {
    String::from("created")
}

fn process(s: String) -> String {
    println!("process: {}", s);
    s
}

fn consume(s: String) {
    println!("consume: {}", s);
}

fn main() {
    let s1 = create();
    let s2 = process(s1);
    let s3 = process(s2);
    consume(s3);
    
    // 问题：追踪所有权从 create 到 consume 的流转
}
```

**答案：**

```
所有权流转路径：

1. create() 创建 String
   └── 返回所有权给 s1
   
2. process(s1)
   ├── s1 移动到函数参数
   ├── 函数返回所有权
   └── s2 获得所有权
   
3. process(s2)
   ├── s2 移动到函数参数
   ├── 函数返回所有权
   └── s3 获得所有权
   
4. consume(s3)
   ├── s3 移动到函数参数
   └── 函数结束，s3 被释放
   
5. main 结束
   └── 无变量需要释放

流转图：
create ──▶ s1 ──▶ process ──▶ s2 ──▶ process ──▶ s3 ──▶ consume ──▶ (释放)
```

---

## 进阶题：代码填空

### 练习 6：修复所有权错误

**题目：** 填空修复代码，使其能编译运行。

```rust
fn main() {
    let s = String::from("hello");
    
    // 填空：使 print 能调用两次
    print(____);
    print(____);
    
    println!("still valid: {}", s);
}

fn print(s: ____) {
    println!("{}", s);
}
```

**答案：**

```rust
fn main() {
    let s = String::from("hello");
    
    // 借用而不是移动
    print(&s);
    print(&s);
    
    println!("still valid: {}", s);
}

fn print(s: &str) {  // 接受 &str，最通用
    println!("{}", s);
}
```

---

### 练习 7：函数返回所有权

**题目：** 完成函数使其返回正确的所有权。

```rust
fn append_suffix(____) -> ____ {
    // 填空：添加 "_processed" 后缀
    ____
}

fn main() {
    let s = String::from("data");
    let result = append_suffix(s);
    
    println!("result: {}", result);
}
```

**答案：**

```rust
fn append_suffix(mut s: String) -> String {
    s.push_str("_processed");
    s  // 返回所有权
}

fn main() {
    let s = String::from("data");
    let result = append_suffix(s);
    
    println!("result: {}", result);
}
```

---

### 练习 8：结构体所有权

**题目：** 完成结构体和方法。

```rust
struct Person {
    name: ____,
    age: ____,
}

impl Person {
    fn new(name: ____, age: ____) -> Self {
        Self { name, age }
    }
    
    fn get_name(&self) -> ____ {
        ____
    }
    
    fn set_age(____, age: ____) {
        ____
    }
}

fn main() {
    let p = Person::new(String::from("Alice"), 30);
    println!("name: {}", p.get_name());
    
    let mut p2 = p;
    p2.set_age(31);
}
```

**答案：**

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

fn main() {
    let p = Person::new(String::from("Alice"), 30);
    println!("name: {}", p.get_name());
    
    let mut p2 = p;  // p 移动到 p2
    p2.set_age(31);
    println!("new age: {}", p2.age);
}
```

---

### 练习 9：集合所有权

**题目：** 完成 Vec 操作。

```rust
fn main() {
    let mut vec = Vec::new();
    
    // 添加三个元素
    vec.push(____);
    vec.push(____);
    vec.push(____);
    
    // 取出第一个元素
    let first = ____;
    
    println!("first: {}", first);
    println!("remaining: {:?}", vec);
}
```

**答案：**

```rust
fn main() {
    let mut vec: Vec<String> = Vec::new();
    
    vec.push(String::from("first"));
    vec.push(String::from("second"));
    vec.push(String::from("third"));
    
    let first = vec.remove(0);  // 移除并获取所有权
    
    println!("first: {}", first);
    println!("remaining: {:?}", vec);
}
```

---

### 练习 10：条件所有权

**题目：** 根据条件决定所有权处理。

```rust
fn process(s: String) -> ____ {
    if s.len() > 5 {
        // 返回修改后的 String
        let mut s = s;
        s.push_str("_long");
        ____
    } else {
        // 不修改，返回原值
        ____
    }
}

fn main() {
    let s1 = String::from("hello world");
    let r1 = process(s1);
    println!("r1: {}", r1);
    
    let s2 = String::from("hi");
    let r2 = process(s2);
    println!("r2: {}", r2);
}
```

**答案：**

```rust
fn process(s: String) -> String {
    if s.len() > 5 {
        let mut s = s;
        s.push_str("_long");
        s  // 返回所有权
    } else {
        s  // 返回原所有权
    }
}

fn main() {
    let s1 = String::from("hello world");
    let r1 = process(s1);
    println!("r1: {}", r1);  // hello world_long
    
    let s2 = String::from("hi");
    let r2 = process(s2);
    println!("r2: {}", r2);  // hi
}
```

---

## 挑战题：重构代码

### 练习 11：重构过度克隆

**问题代码：**

```rust
fn main() {
    let data = String::from("important data");
    
    let copy1 = data.clone();
    let copy2 = data.clone();
    let copy3 = data.clone();
    
    process(copy1);
    process(copy2);
    process(copy3);
    
    println!("original: {}", data);
}

fn process(s: String) {
    println!("processing: {}", s);
}
```

**要求：**
- 减少不必要的克隆
- 保持功能不变
- 使代码更高效

**参考答案：**

```rust
fn main() {
    let data = String::from("important data");
    
    // 使用引用，无需克隆
    process(&data);
    process(&data);
    process(&data);
    
    println!("original: {}", data);
}

fn process(s: &str) {  // 接受 &str
    println!("processing: {}", s);
}
```

---

### 练习 12：重构循环所有权错误

**问题代码：**

```rust
fn main() {
    let items = vec![
        String::from("item1"),
        String::from("item2"),
        String::from("item3"),
    ];
    
    for _ in 0..3 {
        for item in &items {
            consume(item.clone());  // 每次都克隆
        }
    }
}

fn consume(s: String) {
    println!("{}", s);
}
```

**要求：**
- 修改函数签名避免克隆
- 保持功能不变

**参考答案：**

```rust
fn main() {
    let items = vec![
        String::from("item1"),
        String::from("item2"),
        String::from("item3"),
    ];
    
    for _ in 0..3 {
        for item in &items {
            process(item);  // 直接传递引用
        }
    }
}

fn process(s: &str) {  // 接受引用
    println!("{}", s);
}
```

---

### 练习 13：设计安全的 API

**问题：** 设计一个字符串处理器 API

**要求：**
- 支持链式操作
- 不消耗原值所有权
- 支持修改和查询操作

**参考答案：**

```rust
struct StringProcessor {
    data: String,
}

impl StringProcessor {
    fn new(data: String) -> Self {
        Self { data }
    }
    
    // 查询操作（借用）
    fn get(&self) -> &str {
        &self.data
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn contains(&self, pattern: &str) -> bool {
        self.data.contains(pattern)
    }
    
    // 修改操作（可变借用）
    fn append(&mut self, suffix: &str) -> &mut Self {
        self.data.push_str(suffix);
        self
    }
    
    fn trim(&mut self) -> &mut Self {
        self.data = self.data.trim().to_string();
        self
    }
    
    fn uppercase(&mut self) -> &mut Self {
        self.data = self.data.to_uppercase();
        self
    }
    
    // 提取操作（返回所有权）
    fn extract(self) -> String {
        self.data
    }
    
    // 克隆操作
    fn clone_data(&self) -> String {
        self.data.clone()
    }
}

fn main() {
    let original = String::from("  hello world  ");
    
    // 链式修改
    let mut processor = StringProcessor::new(original.clone());
    processor.trim().uppercase().append("!");
    
    println!("处理后: {}", processor.get());
    println!("原值: {}", original);  // 原值仍有效
    
    // 提取处理后的值
    let result = processor.extract();
    println!("提取: {}", result);
}
```

---

### 练习 14：重构函数参数类型

**问题代码：**

```rust
fn format_string(s: String) -> String {
    format!("Formatted: {}", s)
}

fn print_string(s: String) {
    println!("{}", s);
}

fn save_string(s: String) -> Result<(), std::io::Error> {
    std::fs::write("output.txt", &s)?;
    Ok(())
}

fn main() {
    let data = String::from("data");
    
    let formatted = format_string(data.clone());
    print_string(formatted.clone());
    save_string(formatted.clone())?;
    
    println!("original: {}", data);
}
```

**要求：**
- 优化函数签名
- 减少克隆
- 保持功能

**参考答案：**

```rust
fn format_string(s: &str) -> String {
    format!("Formatted: {}", s)
}

fn print_string(s: &str) {
    println!("{}", s);
}

fn save_string(s: &str) -> Result<(), std::io::Error> {
    std::fs::write("output.txt", s)?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let data = String::from("data");
    
    let formatted = format_string(&data);
    print_string(&formatted);
    save_string(&formatted)?;
    
    println!("original: {}", data);
    Ok(())
}
```

---

### 练习 15：实现资源管理

**问题：** 实现一个文件资源管理器

**要求：**
- 打开文件获取所有权
- 读取内容（借用）
- 关闭文件（自动）
- 遵循 RAII

**参考答案：**

```rust
use std::fs::File;
use std::io::{self, Read};

struct FileResource {
    file: File,
    path: String,
}

impl FileResource {
    fn open(path: String) -> io::Result<Self> {
        let file = File::open(&path)?;
        Ok(Self { file, path })
    }
    
    fn read_content(&mut self) -> io::Result<String> {
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;
        Ok(content)
    }
    
    fn get_path(&self) -> &str {
        &self.path
    }
}

impl Drop for FileResource {
    fn drop(&mut self) {
        println!("自动关闭文件: {}", self.path);
    }
}

fn main() -> io::Result<()> {
    let mut resource = FileResource::open(String::from("test.txt"))?;
    
    println!("文件路径: {}", resource.get_path());
    
    if let Ok(content) = resource.read_content() {
        println!("内容长度: {}", content.len());
    }
    
    // resource 离开作用域，自动关闭
    Ok(())
}
```

---

### 练习 16：优化数据结构

**问题代码：**

```rust
struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }
    
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}
```

**要求：**
- 优化 get 方法，避免不必要的克隆
- 支持多种字符串类型

**参考答案：**

```rust
use std::collections::HashMap;

struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    fn get(&self, key: &str) -> Option<&str> {  // 返回引用
        self.data.get(key).map(|s| s.as_str())
    }
    
    fn get_owned(&self, key: &str) -> Option<String> {  // 需要时才克隆
        self.data.get(key).cloned()
    }
    
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    fn set_str(&mut self, key: &str, value: &str) {  // 接受引用
        self.data.insert(key.to_string(), value.to_string());
    }
}

fn main() {
    let mut cache = Cache {
        data: HashMap::new(),
    };
    
    cache.set(String::from("name"), String::from("Rust"));
    
    // 使用引用
    if let Some(value) = cache.get("name") {
        println!("值: {}", value);
    }
    
    // 需要所有权时
    if let Some(value) = cache.get_owned("name") {
        println!("拥有值: {}", value);
    }
}
```

---

### 练习 17：实现字符串池

**问题：** 实现一个字符串池避免重复字符串

**要求：**
- 相同字符串只存储一次
- 返回引用而不是克隆
- 支持添加和查询

**参考答案：**

```rust
use std::collections::HashSet;

struct StringPool {
    pool: HashSet<String>,
}

impl StringPool {
    fn new() -> Self {
        Self {
            pool: HashSet::new(),
        }
    }
    
    fn add(&mut self, s: String) -> &str {
        // 如果已存在，返回现有引用
        // 如果不存在，插入并返回引用
        if !self.pool.contains(&s) {
            self.pool.insert(s);
        }
        self.pool.get(&s).unwrap().as_str()
    }
    
    fn get(&self, s: &str) -> Option<&str> {
        self.pool.get(s).map(|s| s.as_str())
    }
    
    fn size(&self) -> usize {
        self.pool.len()
    }
}

fn main() {
    let mut pool = StringPool::new();
    
    let s1 = pool.add(String::from("hello"));
    let s2 = pool.add(String::from("hello"));  // 相同字符串
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("池大小: {}", pool.size());  // 1，不是 2
    
    if let Some(s) = pool.get("hello") {
        println!("找到: {}", s);
    }
}
```

---

### 练习 18：实现借用检查器

**问题：** 实现一个简单的借用状态追踪器

**要求：**
- 追踪变量的借用状态
- 检测可变借用冲突
- 检测悬垂引用

**参考答案：**

```rust
#[derive(Debug, PartialEq)]
enum BorrowState {
    NotBorrowed,
    ImmutablyBorrowed(usize),  // 借用次数
    MutablyBorrowed,
}

struct BorrowTracker<T> {
    value: T,
    state: BorrowState,
}

impl<T> BorrowTracker<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            state: BorrowState::NotBorrowed,
        }
    }
    
    fn borrow(&self) -> Result<&T, &'static str> {
        match &self.state {
            BorrowState::NotBorrowed | 
            BorrowState::ImmutablyBorrowed(_) => Ok(&self.value),
            BorrowState::MutablyBorrowed => Err("已有可变借用"),
        }
    }
    
    fn borrow_mut(&mut self) -> Result<&mut T, &'static str> {
        match &self.state {
            BorrowState::NotBorrowed => {
                self.state = BorrowState::MutablyBorrowed;
                Ok(&mut self.value)
            },
            BorrowState::ImmutablyBorrowed(_) => Err("已有不可变借用"),
            BorrowState::MutablyBorrowed => Err("已有可变借用"),
        }
    }
}

fn main() {
    let mut tracker = BorrowTracker::new(String::from("data"));
    
    // 不可变借用
    if let Ok(ref1) = tracker.borrow() {
        println!("借用 1: {}", ref1);
    }
    
    if let Ok(ref2) = tracker.borrow() {
        println!("借用 2: {}", ref2);
    }
    
    // 可变借用（此时有不可变借用，应该失败）
    if let Err(e) = tracker.borrow_mut() {
        println!("错误: {}", e);
    }
}
```

---

### 练习 19：重构数据处理管道

**问题代码：**

```rust
fn main() {
    let data = vec![
        String::from("item1"),
        String::from("item2"),
        String::from("item3"),
    ];
    
    let mut results = Vec::new();
    for item in data {
        let processed = process(item);
        results.push(processed);
    }
    
    println!("results: {:?}", results);
}

fn process(s: String) -> String {
    s.to_uppercase()
}
```

**要求：**
- 支持多次处理同一数据
- 不消耗原数据所有权
- 使用迭代器优化

**参考答案：**

```rust
fn main() {
    let data = vec![
        String::from("item1"),
        String::from("item2"),
        String::from("item3"),
    ];
    
    // 使用迭代器，不消耗所有权
    let results: Vec<String> = data.iter()
        .map(|item| process(item))
        .collect();
    
    println!("results: {:?}", results);
    println!("原数据仍可用: {:?}", data);
}

fn process(s: &str) -> String {
    s.to_uppercase()
}
```

---

### 练习 20：实现所有权转移模式

**问题：** 实现一种安全的所有权转移机制

**要求：**
- 支持所有权转移
- 转移后原持有者无效
- 支持检查所有权状态

**参考答案：**

```rust
struct OwnedData {
    data: String,
    owner_id: u64,
}

struct Owner {
    id: u64,
    holding: Option<OwnedData>,
}

impl Owner {
    fn new(id: u64) -> Self {
        Self {
            id,
            holding: None,
        }
    }
    
    fn take(&mut self, data: OwnedData) {
        self.holding = Some(OwnedData {
            data: data.data,
            owner_id: self.id,
        });
    }
    
    fn give(&mut self) -> Option<OwnedData> {
        self.holding.take()
    }
    
    fn has_data(&self) -> bool {
        self.holding.is_some()
    }
}

fn main() {
    let mut owner1 = Owner::new(1);
    let mut owner2 = Owner::new(2);
    
    let data = OwnedData {
        data: String::from("important data"),
        owner_id: 0,
    };
    
    // owner1 获得所有权
    owner1.take(data);
    println!("owner1 拥有数据: {}", owner1.has_data());
    
    // 转移给 owner2
    if let Some(d) = owner1.give() {
        owner2.take(d);
    }
    
    println!("owner1 不再拥有: {}", !owner1.has_data());
    println!("owner2 拥有数据: {}", owner2.has_data());
}
```

---

## 练习题答案总结

### 基础题答案表

| 练习 | 核心概念 | 答案要点 |
|------|----------|---------|
| 1 | 移动语义 | clone() 不移动原值 |
| 2 | 函数参数 | 先借用再移动 |
| 3 | Copy trait | 元素决定组合类型 |
| 4 | 作用域 | 内层不能访问外层 |
| 5 | 所有权流转 | 追踪所有权路径 |

### 进阶题答案表

| 练习 | 核心概念 | 答案要点 |
|------|----------|---------|
| 6 | 引用参数 | &str 最通用 |
| 7 | 返回所有权 | 返回修改后的 String |
| 8 | 结构体所有权 | 借用方法返回 &str |
| 9 | 集合所有权 | remove 获取所有权 |
| 10 | 条件所有权 | 分支返回所有权 |

### 挑战题答案表

| 练习 | 核心概念 | 重构要点 |
|------|----------|---------|
| 11 | 减少克隆 | 使用引用参数 |
| 12 | 循环所有权 | 引用避免克隆 |
| 13 | API 设计 | 链式操作，RAII |
| 14 | 函数签名 | &str 参数 |
| 15 | 资源管理 | Drop 自动清理 |
| 16 | 数据结构 | 返回引用优化 |
| 17 | 字符串池 | 共享所有权 |
| 18 | 借用检查 | 状态追踪 |
| 19 | 数据管道 | 迭代器优化 |
| 20 | 所有权转移 | 安全转移机制 |

---

## 学习建议

### 完成练习后的检查清单

- [ ] 理解所有权移动的本质
- [ ] 掌握函数参数的传递方式
- [ ] 能够区分 Copy 和 Clone
- [ ] 理解作用域与生命周期
- [ ] 能够追踪所有权流转
- [ ] 掌握引用的使用时机
- [ ] 能够优化函数签名
- [ ] 理解结构体所有权
- [ ] 能够设计安全的 API
- [ ] 掌握性能优化技巧

### 继续学习

完成这些练习后，建议：
1. 回顾错误案例，加深理解
2. 尝试编写实际项目代码
3. 学习引用和借用章节
4. 研究生命周期概念

➡️ [第 8 章：引用与借用](../02-references/)