# 第 8 章：引用与借用 - 练习题

> 通过实践掌握引用和借用的核心概念。

## 基础题（1-5）

### 练习 1：引用基础

**题目：** 编写函数 `get_length`，接受字符串切片的引用并返回长度。

**要求：**
- 不转移所有权
- 使用 `&str` 类型

**示例：**
```rust
fn main() {
    let s = String::from("hello");
    let len = get_length(&s);
    println!("'{}' 的长度是 {}", s, len);  // s 仍然有效
}
```

<details>
<summary>查看答案</summary>

```rust
fn get_length(s: &str) -> usize {
    s.len()
}
```

</details>

---

### 练习 2：可变引用

**题目：** 编写函数 `append_suffix`，接受字符串的可变引用并追加后缀 "!"。

**示例：**
```rust
fn main() {
    let mut message = String::from("Hello");
    append_suffix(&mut message);
    println!("{}", message);  // "Hello!"
}
```

<details>
<summary>查看答案</summary>

```rust
fn append_suffix(s: &mut String) {
    s.push('!');
}
```

</details>

---

### 练习 3：多个不可变引用

**题目：** 编写函数 `compare_strings`，接受两个字符串引用，返回较长的那个引用。

**示例：**
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let longer = compare_strings(&s1, &s2);
    println!("较长的是：{}", longer);  // "world!"
}
```

<details>
<summary>查看答案</summary>

```rust
fn compare_strings<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

</details>

---

### 练习 4：引用与所有权

**题目：** 分析以下代码，指出错误并修复。

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);  // 问题：这里会发生什么？
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

**要求：**
- 保持 `takes_ownership` 函数签名不变
- 使 `main` 函数中的 `println!` 能正常工作

<details>
<summary>查看答案</summary>

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s.clone());  // 克隆，不转移所有权
    println!("{}", s);           // ✅ 现在可以工作
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

或使用引用：

```rust
fn main() {
    let s = String::from("hello");
    borrows_value(&s);  // 借用，不转移所有权
    println!("{}", s);  // ✅ 可以工作
}

fn borrows_value(s: &String) {
    println!("{}", s);
}
```

</details>

---

### 练习 5：解引用

**题目：** 编写函数 `add_one`，接受 `i32` 的引用并将其值加 1。

**示例：**
```rust
fn main() {
    let mut x = 5;
    add_one(&mut x);
    println!("x = {}", x);  // 6
}
```

<details>
<summary>查看答案</summary>

```rust
fn add_one(num: &mut i32) {
    *num += 1;  // *num 解引用
}
```

</details>

---

## 中级题（6-10）

### 练习 6：借用规则实践

**题目：** 修复以下代码中的借用错误。

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    let first = &numbers[0];
    numbers.push(6);  // ❌ 借用错误
    
    println!("first = {}", first);
}
```

<details>
<summary>查看答案</summary>

**方案 1：提前使用**
```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    let first = numbers[0];  // 复制值（i32 是 Copy 类型）
    numbers.push(6);          // ✅ 现在可以修改
    
    println!("first = {}", first);
}
```

**方案 2：分离作用域**
```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    {
        let first = &numbers[0];
        println!("first = {}", first);
    }  // first 借用结束
    
    numbers.push(6);  // ✅ 可以修改
}
```

</details>

---

### 练习 7：结构体字段借用

**题目：** 编写函数，更新 `Person` 的年龄并返回名字的引用。

```rust
struct Person {
    name: String,
    age: u32,
}

// 实现这个函数
fn update_age_and_get_name(person: &mut Person, new_age: u32) -> &str {
    // 你的代码
}

fn main() {
    let mut person = Person {
        name: String::from("Alice"),
        age: 25,
    };
    
    let name = update_age_and_get_name(&mut person, 26);
    println!("Name: {}, Age: {}", name, person.age);
}
```

<details>
<summary>查看答案</summary>

```rust
fn update_age_and_get_name(person: &mut Person, new_age: u32) -> &str {
    person.age = new_age;
    &person.name
}
```

</details>

---

### 练习 8：迭代器与借用

**题目：** 编写函数，找出向量中的最大值并返回其引用。

```rust
fn find_max(numbers: &Vec<i32>) -> Option<&i32> {
    // 你的代码
}

fn main() {
    let nums = vec![3, 7, 2, 9, 4];
    match find_max(&nums) {
        Some(max) => println!("最大值：{}", max),
        None => println!("向量是空的"),
    }
}
```

<details>
<summary>查看答案</summary>

```rust
fn find_max(numbers: &Vec<i32>) -> Option<&i32> {
    if numbers.is_empty() {
        return None;
    }
    
    let mut max = &numbers[0];
    for num in numbers.iter().skip(1) {
        if num > max {
            max = num;
        }
    }
    Some(max)
}
```

或使用标准库：

```rust
fn find_max(numbers: &Vec<i32>) -> Option<&i32> {
    numbers.iter().max()
}
```

</details>

---

### 练习 9：字符串处理

**题目：** 编写函数，将字符串中的所有单词首字母大写。

```rust
fn capitalize_words(text: &mut String) {
    // 你的代码
}

fn main() {
    let mut s = String::from("hello world rust");
    capitalize_words(&mut s);
    println!("{}", s);  // "Hello World Rust"
}
```

<details>
<summary>查看答案</summary>

```rust
fn capitalize_words(text: &mut String) {
    let mut chars: Vec<char> = text.chars().collect();
    
    for i in 0..chars.len() {
        if i == 0 || chars[i - 1] == ' ' {
            chars[i] = chars[i].to_uppercase().next().unwrap();
        }
    }
    
    *text = chars.into_iter().collect();
}
```

</details>

---

### 练习 10：多维数据访问

**题目：** 编写函数，交换矩阵的两行。

```rust
fn swap_rows(matrix: &mut Vec<Vec<i32>>, row1: usize, row2: usize) {
    // 你的代码
}

fn main() {
    let mut matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    
    swap_rows(&mut matrix, 0, 2);
    
    for row in &matrix {
        println!("{:?}", row);
    }
    // [7, 8, 9]
    // [4, 5, 6]
    // [1, 2, 3]
}
```

<details>
<summary>查看答案</summary>

```rust
fn swap_rows(matrix: &mut Vec<Vec<i32>>, row1: usize, row2: usize) {
    if row1 < matrix.len() && row2 < matrix.len() {
        matrix.swap(row1, row2);
    }
}
```

</details>

---

## 进阶题（11-15）

### 练习 11：生命周期标注

**题目：** 为以下函数添加正确的生命周期标注。

```rust
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

**要求：** 理解为什么需要生命周期标注，以及编译器如何推断。

<details>
<summary>查看答案</summary>

```rust
fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

**解释：**
- 返回的字符串切片必须与输入参数有相同的生命周期
- `<'a>` 声明生命周期参数
- `&'a str` 表示生命周期为 `'a` 的字符串切片

</details>

---

### 练习 12：复杂借用场景

**题目：** 实现一个简单的文本编辑器，支持插入和删除操作。

```rust
struct TextEditor {
    content: String,
    cursor: usize,
}

impl TextEditor {
    fn new() -> Self {
        TextEditor {
            content: String::new(),
            cursor: 0,
        }
    }
    
    fn insert(&mut self, text: &str) {
        // 在光标位置插入文本
    }
    
    fn delete(&mut self, length: usize) {
        // 从光标位置删除指定长度的字符
    }
    
    fn get_content(&self) -> &str {
        // 返回内容引用
    }
    
    fn move_cursor(&mut self, position: usize) {
        // 移动光标
    }
}

fn main() {
    let mut editor = TextEditor::new();
    
    editor.insert("Hello");
    editor.move_cursor(5);
    editor.insert(" World");
    
    println!("{}", editor.get_content());  // "Hello World"
    
    editor.move_cursor(5);
    editor.delete(6);
    
    println!("{}", editor.get_content());  // "Hello"
}
```

<details>
<summary>查看答案</summary>

```rust
impl TextEditor {
    fn new() -> Self {
        TextEditor {
            content: String::new(),
            cursor: 0,
        }
    }
    
    fn insert(&mut self, text: &str) {
        self.content.insert_str(self.cursor, text);
        self.cursor += text.len();
    }
    
    fn delete(&mut self, length: usize) {
        let end = (self.cursor + length).min(self.content.len());
        self.content.replace_range(self.cursor..end, "");
    }
    
    fn get_content(&self) -> &str {
        &self.content
    }
    
    fn move_cursor(&mut self, position: usize) {
        self.cursor = position.min(self.content.len());
    }
}
```

</details>

---

### 练习 13：引用与集合

**题目：** 实现函数，从向量中移除所有偶数，返回被移除的数字。

```rust
fn remove_evens(numbers: &mut Vec<i32>) -> Vec<i32> {
    // 你的代码
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let removed = remove_evens(&mut nums);
    
    println!("剩余：{:?}", nums);     // [1, 3, 5, 7]
    println!("移除：{:?}", removed);   // [2, 4, 6, 8]
}
```

<details>
<summary>查看答案</summary>

```rust
fn remove_evens(numbers: &mut Vec<i32>) -> Vec<i32> {
    let mut removed = Vec::new();
    let mut i = 0;
    
    while i < numbers.len() {
        if numbers[i] % 2 == 0 {
            removed.push(numbers.remove(i));
        } else {
            i += 1;
        }
    }
    
    removed
}
```

或使用 `drain_filter`（Rust 1.61+）：

```rust
fn remove_evens(numbers: &mut Vec<i32>) -> Vec<i32> {
    numbers.drain_filter(|x| x % 2 == 0).collect()
}
```

</details>

---

### 练习 14：缓存计算结果

**题目：** 实现一个带缓存的计算器。

```rust
struct CachedCalculator {
    cache: std::collections::HashMap<u32, u64>,
    calculations: u32,
}

impl CachedCalculator {
    fn new() -> Self {
        CachedCalculator {
            cache: std::collections::HashMap::new(),
            calculations: 0,
        }
    }
    
    fn fibonacci(&mut self, n: u32) -> u64 {
        // 如果缓存中有，直接返回
        // 否则计算并缓存
    }
    
    fn get_cache(&self) -> &std::collections::HashMap<u32, u64> {
        // 返回缓存引用
    }
    
    fn get_calculation_count(&self) -> u32 {
        // 返回计算次数
    }
}

fn main() {
    let mut calc = CachedCalculator::new();
    
    println!("fib(10) = {}", calc.fibonacci(10));
    println!("fib(10) = {}", calc.fibonacci(10));  // 从缓存读取
    
    println!("计算次数：{}", calc.get_calculation_count());
    println!("缓存大小：{}", calc.get_cache().len());
}
```

<details>
<summary>查看答案</summary>

```rust
use std::collections::HashMap;

struct CachedCalculator {
    cache: HashMap<u32, u64>,
    calculations: u32,
}

impl CachedCalculator {
    fn new() -> Self {
        CachedCalculator {
            cache: HashMap::new(),
            calculations: 0,
        }
    }
    
    fn fibonacci(&mut self, n: u32) -> u64 {
        if let Some(&result) = self.cache.get(&n) {
            return result;
        }
        
        self.calculations += 1;
        
        let result = if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            self.fibonacci(n - 1) + self.fibonacci(n - 2)
        };
        
        self.cache.insert(n, result);
        result
    }
    
    fn get_cache(&self) -> &HashMap<u32, u64> {
        &self.cache
    }
    
    fn get_calculation_count(&self) -> u32 {
        self.calculations
    }
}
```

</details>

---

### 练习 15：引用链

**题目：** 实现一个链式查找函数。

```rust
struct User {
    id: u32,
    name: String,
    email: String,
}

struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }
    
    fn add_user(&mut self, id: u32, name: &str, email: &str) {
        self.users.push(User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        });
    }
    
    fn find_user_by_id(&self, id: u32) -> Option<&User> {
        // 查找用户
    }
    
    fn find_user_by_email(&self, email: &str) -> Option<&User> {
        // 查找用户
    }
    
    fn get_user_name(&self, id: u32) -> Option<&str> {
        // 返回用户名引用
    }
}

fn main() {
    let mut db = Database::new();
    
    db.add_user(1, "Alice", "alice@example.com");
    db.add_user(2, "Bob", "bob@example.com");
    
    if let Some(user) = db.find_user_by_id(1) {
        println!("找到用户：{}", user.name);
    }
    
    if let Some(name) = db.get_user_name(2) {
        println!("用户名：{}", name);
    }
}
```

<details>
<summary>查看答案</summary>

```rust
impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }
    
    fn add_user(&mut self, id: u32, name: &str, email: &str) {
        self.users.push(User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        });
    }
    
    fn find_user_by_id(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    
    fn find_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.iter().find(|u| u.email == email)
    }
    
    fn get_user_name(&self, id: u32) -> Option<&str> {
        self.find_user_by_id(id).map(|u| u.name.as_str())
    }
}
```

</details>

---

## 挑战题（16-20）

### 练习 16：引用与并发安全

**题目：** 分析以下代码为何安全，并解释借用检查器的作用。

```rust
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 分片处理
    let mid = data.len() / 2;
    let (left, right) = data.split_at_mut(mid);
    
    // 想法：在两个线程中分别处理
    // 但这需要什么？为什么 Rust 禁止这样做？
}
```

<details>
<summary>查看答案</summary>

**分析：**

```rust
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    let mid = data.len() / 2;
    let (left, right) = data.split_at_mut(mid);
    
    // ❌ 错误：线程可能超过数据生命周期
    // thread::spawn(|| {
    //     left[0] = 10;  // left 的生命周期问题
    // });
    
    // ✅ 正确：使用作用域线程
    std::thread::scope(|s| {
        s.spawn(|| {
            left[0] = 10;
        });
        s.spawn(|| {
            right[0] = 20;
        });
    });
    
    println!("{:?}", data);  // [10, 20, 3, 4, 5]
}
```

**关键点：**
1. `split_at_mut` 安全地将可变引用分成两部分
2. 借用检查器确保不会有两个可变引用指向同一数据
3. `std::thread::scope` 确保线程在作用域结束前完成

</details>

---

### 练习 17：生命周期挑战

**题目：** 实现一个函数，返回两个字符串切片中较长的那个。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**问题：**
1. 为什么生命周期标注是 `'a`？
2. 如果返回新创建的字符串会怎样？

**任务：** 编译并运行以下代码，观察错误：

```rust
fn main() {
    let string1 = String::from("long string");
    let result;
    
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    
    println!("最长的是：{}", result);
}
```

<details>
<summary>查看答案</summary>

**错误原因：**
- `string2` 在内部作用域结束时就失效了
- `result` 引用了已失效的 `string2`
- 借用检查器检测到生命周期不匹配

**修复方案：**

```rust
fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("xyz");
    
    let result = longest(&string1, &string2);
    println!("最长的是：{}", result);  // ✅ 正确
}
```

**生命周期解释：**
- `<'a>` 表示生命周期参数
- `x: &'a str` 表示 `x` 活至少 `'a` 久
- `y: &'a str` 表示 `y` 活至少 `'a` 久
- `-> &'a str` 表示返回值也活 `'a` 久
- 编译器确保返回值不会比输入活得长

</details>

---

### 练习 18：智能指针与借用

**题目：** 使用 `RefCell` 实现内部可变性。

```rust
use std::cell::RefCell;

struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }
    
    fn increment(&self) {  // 注意：&self 不是 &mut self
        // 如何修改 value？
    }
    
    fn get(&self) -> i32 {
        // 如何读取 value？
    }
}

fn main() {
    let counter = Counter::new();
    
    counter.increment();
    counter.increment();
    
    println!("计数：{}", counter.get());  // 2
}
```

<details>
<summary>查看答案</summary>

```rust
use std::cell::RefCell;

struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }
    
    fn increment(&self) {
        let mut value = self.value.borrow_mut();
        *value += 1;
    }
    
    fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
```

**关键点：**
- `RefCell` 提供内部可变性
- `borrow()` 返回不可变引用（`Ref<T>`）
- `borrow_mut()` 返回可变引用（`RefMut<T>`）
- 运行时检查借用规则，而非编译时

</details>

---

### 练习 19：零拷贝解析器

**题目：** 实现一个简单的命令行参数解析器，使用引用避免拷贝。

```rust
struct CommandParser<'a> {
    args: Vec<&'a str>,
}

impl<'a> CommandParser<'a> {
    fn new(input: &'a str) -> Self {
        CommandParser {
            args: input.split_whitespace().collect(),
        }
    }
    
    fn get_command(&self) -> Option<&str> {
        // 返回命令（第一个参数）
    }
    
    fn get_arg(&self, index: usize) -> Option<&str> {
        // 返回指定位置的参数
    }
    
    fn get_flag(&self, flag: &str) -> Option<&str> {
        // 查找标志参数（如 --name value）
        // 返回标志的值
    }
}

fn main() {
    let input = "git commit -m \"initial commit\" --author Alice";
    let parser = CommandParser::new(input);
    
    if let Some(cmd) = parser.get_command() {
        println!("命令：{}", cmd);
    }
    
    if let Some(msg) = parser.get_flag("-m") {
        println!("消息：{}", msg);
    }
}
```

<details>
<summary>查看答案</summary>

```rust
struct CommandParser<'a> {
    args: Vec<&'a str>,
}

impl<'a> CommandParser<'a> {
    fn new(input: &'a str) -> Self {
        CommandParser {
            args: input.split_whitespace().collect(),
        }
    }
    
    fn get_command(&self) -> Option<&str> {
        self.args.first().copied()
    }
    
    fn get_arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).copied()
    }
    
    fn get_flag(&self, flag: &str) -> Option<&str> {
        let flag_index = self.args.iter().position(|&arg| arg == flag)?;
        self.args.get(flag_index + 1).copied()
    }
}
```

</details>

---

### 练习 20：完整项目 - 学生成绩管理

**题目：** 实现一个学生成绩管理系统，综合运用引用和借用。

**要求：**
1. 添加学生
2. 添加成绩
3. 查询学生信息
4. 计算平均分
5. 列出所有不及格学生

```rust
use std::collections::HashMap;

struct Student {
    id: u32,
    name: String,
    grades: Vec<f64>,
}

class GradeManager {
    students: HashMap<u32, Student>,
}

impl GradeManager {
    fn new() -> Self {
        // 实现
    }
    
    fn add_student(&mut self, id: u32, name: &str) {
        // 实现
    }
    
    fn add_grade(&mut self, student_id: u32, grade: f64) -> Result<(), String> {
        // 实现
    }
    
    fn get_student(&self, id: u32) -> Option<&Student> {
        // 实现
    }
    
    fn get_average(&self, student_id: u32) -> Option<f64> {
        // 实现
    }
    
    fn get_failing_students(&self, passing_grade: f64) -> Vec<&Student> {
        // 返回平均分低于 passing_grade 的学生
    }
}

fn main() {
    let mut manager = GradeManager::new();
    
    manager.add_student(1, "Alice");
    manager.add_student(2, "Bob");
    manager.add_student(3, "Charlie");
    
    manager.add_grade(1, 85.0);
    manager.add_grade(1, 92.0);
    manager.add_grade(2, 55.0);
    manager.add_grade(2, 60.0);
    manager.add_grade(3, 70.0);
    manager.add_grade(3, 65.0);
    
    if let Some(avg) = manager.get_average(1) {
        println!("Alice 平均分：{}", avg);
    }
    
    let failing = manager.get_failing_students(70.0);
    println!("不及格学生：");
    for student in failing {
        println!("- {}", student.name);
    }
}
```

<details>
<summary>查看答案</summary>

```rust
use std::collections::HashMap;

struct Student {
    id: u32,
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            self.grades.iter().sum::<f64>() / self.grades.len() as f64
        }
    }
}

struct GradeManager {
    students: HashMap<u32, Student>,
}

impl GradeManager {
    fn new() -> Self {
        GradeManager {
            students: HashMap::new(),
        }
    }
    
    fn add_student(&mut self, id: u32, name: &str) {
        self.students.insert(id, Student {
            id,
            name: name.to_string(),
            grades: Vec::new(),
        });
    }
    
    fn add_grade(&mut self, student_id: u32, grade: f64) -> Result<(), String> {
        match self.students.get_mut(&student_id) {
            Some(student) => {
                student.grades.push(grade);
                Ok(())
            }
            None => Err(format!("学生 {} 不存在", student_id)),
        }
    }
    
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }
    
    fn get_average(&self, student_id: u32) -> Option<f64> {
        self.students.get(&student_id).map(|s| s.average())
    }
    
    fn get_failing_students(&self, passing_grade: f64) -> Vec<&Student> {
        self.students
            .values()
            .filter(|s| s.average() < passing_grade)
            .collect()
    }
}
```

</details>

---

## 总结

通过这些练习，你应该掌握了：

1. **引用基础**：不可变引用和可变引用的使用
2. **借用规则**：理解借用检查器的工作原理
3. **生命周期**：标注和理解引用的生命周期
4. **实际应用**：在项目中正确使用引用和借用
5. **调试技巧**：解决借用相关的编译错误

继续练习，深入理解 Rust 的所有权和借用系统！