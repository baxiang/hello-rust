# 第 16 章：Trait 练习题

## 基础练习（1-5）

### 练习 1：定义和实现 Trait

定义一个 `Describable` Trait，并为多个类型实现：

```rust
// 要求：
trait Describable {
    fn describe(&self) -> String;
}

// 为以下类型实现：
struct Product {
    name: String,
    price: f64,
}

struct User {
    id: u32,
    username: String,
}

struct File {
    path: String,
    size: usize,
}

// 测试代码
fn main() {
    let product = Product { name: "Laptop", price: 999.99 };
    let user = User { id: 1, username: "alice" };
    let file = File { path: "/tmp/data.txt", size: 1024 };
    
    println!("{}", product.describe());
    println!("{}", user.describe());
    println!("{}", file.describe());
}
```

---

### 练习 2：默认实现

使用默认实现减少重复代码：

```rust
// 要求：
trait Printable {
    // 提供默认实现
    fn print(&self) {
        println!("{}", self.to_string());
    }
    
    // 必须实现
    fn to_string(&self) -> String;
}

// 为以下类型实现：
struct Point { x: i32, y: i32 }
struct Color { r: u8, g: u8, b: u8 }

// Point 使用默认 print
// Color 自定义 print（带颜色格式）

fn main() {
    let point = Point { x: 5, y: 10 };
    let color = Color { r: 255, g: 0, b: 128 };
    
    point.print();  // 使用默认实现
    color.print();  // 使用自定义实现
}
```

---

### 练习 3：Trait 作为参数

实现通用函数处理不同类型：

```rust
// 要求：
trait Area {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

// 为所有形状实现 Area

// 实现以下通用函数：
fn print_area(shape: &impl Area) {
    // 打印面积
}

fn total_area(shapes: &[&dyn Area]) -> f64 {
    // 计算总面积
}

fn largest_shape<'a>(shapes: &'a [&'a dyn Area]) -> &'a dyn Area {
    // 返回面积最大的形状
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    let tri = Triangle { base: 6.0, height: 8.0 };
    
    print_area(&circle);
    
    let shapes: Vec<&dyn Area> = vec![&circle, &rect, &tri];
    println!("Total area: {}", total_area(&shapes));
}
```

---

### 练习 4：多个 Trait 约束

实现需要多个 Trait 的函数：

```rust
// 要求：
trait Summary {
    fn summarize(&self) -> String;
}

trait Identify {
    fn id(&self) -> u32;
}

struct Article {
    id: u32,
    title: String,
    author: String,
}

struct Comment {
    id: u32,
    user: String,
    content: String,
}

// 为两个类型实现 Summary 和 Identify

// 实现函数：
fn print_item<T: Summary + Identify>(item: &T) {
    // 打印 ID 和摘要
}

fn compare_ids<T: Identify, U: Identify>(a: &T, b: &U) -> bool {
    // 比较两个类型的 ID
}

// 使用 where 子句提高可读性
fn process_items<T, U>(a: &T, b: &U) -> String
where
    T: Summary + Identify,
    U: Summary + Identify,
{
    // 处理两个项目
}
```

---

### 练习 5：孤儿规则

理解孤儿规则的限制：

```rust
// ❌ 尝试为标准库类型实现自定义 Trait（会被拒绝）
// trait MyTrait {
//     fn my_method(&self);
// }
// 
// impl MyTrait for String { }  // ❌ 孤儿规则

// ✅ 正确方式 1：为自定义类型实现标准库 Trait
struct Wrapper {
    value: String,
}

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrapper({})", self.value)
    }
}

// ✅ 正确方式 2：定义本地 Trait 为本地类型
trait LocalTrait {
    fn method(&self);
}

impl LocalTrait for Wrapper {
    fn method(&self) {
        println!("{}", self.value);
    }
}

// ✅ 正确方式 3：使用 newtype 模式
struct MyString(String);

impl MyString {
    fn new(s: String) -> Self {
        MyString(s)
    }
}

impl MyTrait for MyString {
    fn my_method(&self) {
        println!("{}", self.0);
    }
}

fn main() {
    let wrapped = Wrapper { value: String::from("hello") };
    println!("{}", wrapped);  // 使用 Display
    wrapped.method();          // 使用 LocalTrait
}
```

---

## 中级练习（6-10）

### 练习 6：关联类型

实现带关联类型的 Trait：

```rust
// 要求：
trait Container {
    type Item;
    
    fn new() -> Self;
    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

// 实现以下容器：
struct VecContainer<T> {
    data: Vec<T>,
}

struct LinkedList<T> {
    // 简化的链表
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// 使用关联类型
impl Container for VecContainer<i32> {
    type Item = i32;
    
    // 实现方法
}

fn main() {
    let mut container = VecContainer::<i32>::new();
    container.add(1);
    container.add(2);
    
    println!("Length: {}", container.len());
    println!("First: {}", container.get(0).unwrap());
}
```

---

### 练习 7：Iterator 实现

实现自定义迭代器：

```rust
// 要求：实现迭代斐波那契数列
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        // 实现迭代逻辑
    }
}

// 实现迭代器适配器
struct TakeWhile<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Iterator for TakeWhile<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        // 实现
    }
}

fn main() {
    let fib = Fibonacci::new();
    let first_ten: Vec<u64> = fib.take(10).collect();
    println!("{:?}", first_ten);
    
    let fib2 = Fibonacci::new();
    let less_than_100: Vec<u64> = fib2
        .take_while(|x| *x < 100)
        .collect();
    println!("{:?}", less_than_100);
}
```

---

### 练习 8：From/Into 转换

实现类型转换系统：

```rust
// 要求：实现货币转换
struct USD(f64);
struct EUR(f64);
struct GBP(f64);

// 实现双向转换
impl From<EUR> for USD {
    fn from(eur: EUR) -> Self {
        USD(eur.0 * 1.1)  // 简化汇率
    }
}

impl From<USD> for EUR {
    fn from(usd: USD) -> Self {
        EUR(usd.0 / 1.1)
    }
}

// 实现其他转换

fn convert<T, U>(value: T) -> U
where
    U: From<T>,
{
    value.into()
}

fn main() {
    let usd = USD(100.0);
    let eur: EUR = usd.into();
    let gbp: GBP = eur.into();
    
    println!("USD: {}", usd.0);
    println!("EUR: {}", eur.0);
    println!("GBP: {}", gbp.0);
}
```

---

### 练习 9：Deref 智能指针

实现自定义智能指针：

```rust
// 要求：
struct SmartBox<T> {
    value: T,
}

impl<T> SmartBox<T> {
    fn new(value: T) -> Self {
        SmartBox { value }
    }
}

impl<T> Deref for SmartBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        // 实现
    }
}

impl<T> DerefMut for SmartBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // 实现
    }
}

impl<T> Drop for SmartBox<T> {
    fn drop(&mut self) {
        println!("Dropping SmartBox");
    }
}

fn main() {
    let box = SmartBox::new(String::from("hello"));
    
    // Deref 强制转换
    println!("{}", box.len());  // 自动转换为 &String
    println!("{}", &box as &str);  // 多级 Deref
}
```

---

### 练习 10：Trait 对象

实现动态分发系统：

```rust
// 要求：插件系统
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, data: &mut String);
}

struct TrimPlugin;
struct UppercasePlugin;
struct ReversePlugin;

// 实现所有插件

struct PluginChain {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginChain {
    fn new() -> Self {
        PluginChain { plugins: Vec::new() }
    }
    
    fn add(&mut self, plugin: impl Plugin + 'static) {
        self.plugins.push(Box::new(plugin));
    }
    
    fn process(&self, input: String) -> String {
        // 按顺序执行所有插件
    }
}

fn main() {
    let mut chain = PluginChain::new();
    chain.add(TrimPlugin);
    chain.add(UppercasePlugin);
    chain.add(ReversePlugin);
    
    let result = chain.process("  hello world  ".to_string());
    println!("{}", result);  // "DLROW OLLEH"
}
```

---

## 高级练习（11-15）

### 练习 11：生命周期 + Trait

结合生命周期和 Trait：

```rust
// 要求：引用解析器
trait Parser<'a> {
    fn parse(&self, input: &'a str) -> &'a str;
}

struct WordParser;
struct NumberParser;
struct TrimParser;

impl<'a> Parser<'a> for WordParser {
    fn parse(&self, input: &'a str) -> &'a str {
        // 返回第一个单词
    }
}

fn apply_parser<'a>(parser: &dyn Parser<'a>, input: &'a str) -> &'a str {
    parser.parse(input)
}

fn main() {
    let input = "hello world 123";
    
    let parsers: Vec<Box<dyn for<'a> Parser<'a>>> = vec![
        Box::new(WordParser),
        Box::new(NumberParser),
    ];
    
    for parser in &parsers {
        println!("{}", parser.parse(input));
    }
}
```

---

### 练习 12：条件实现

使用 Trait 边界实现条件方法：

```rust
// 要求：
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

// 只有当 T: Display 时才有此方法
impl<T: Display> Pair<T> {
    fn display(&self) {
        println!("({}, {})", self.first, self.second);
    }
}

// 只有当 T: PartialOrd 时才有此方法
impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

// 只有当 T: Clone 时才有此方法
impl<T: Clone> Pair<T> {
    fn clone_pair(&self) -> Pair<T> {
        Pair {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }
}

fn main() {
    let int_pair = Pair::new(5, 10);
    int_pair.display();      // ✅ i32: Display
    println!("{}", int_pair.larger());  // ✅ i32: PartialOrd
    
    let str_pair = Pair::new("hello", "world");
    str_pair.display();      // ✅ &str: Display
    // str_pair.larger();    // ❌ &str 不实现 PartialOrd
}
```

---

### 练习 13：Blanket Implementation

实现泛型 Trait 实现：

```rust
// 要求：
trait ToStringCustom {
    fn to_string_custom(&self) -> String;
}

// 为所有实现 Display 的类型自动实现
impl<T: Display> ToStringCustom for T {
    fn to_string_custom(&self) -> String {
        format!("Custom: {}", self)
    }
}

trait Numeric {
    fn is_positive(&self) -> bool;
}

// 为所有数值类型自动实现
impl Numeric for i32 {
    fn is_positive(&self) -> bool {
        *self > 0
    }
}

impl Numeric for f64 {
    fn is_positive(&self) -> bool {
        *self > 0.0
    }
}

// 为所有实现 Numeric 的类型扩展方法
trait ExtendedNumeric: Numeric {
    fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}

impl<T: Numeric> ExtendedNumeric for T {}

fn main() {
    let num = 42;
    println!("{}", num.to_string_custom());  // Custom: 42
    println!("{}", num.is_positive());       // true
    println!("{}", num.is_negative());       // false
}
```

---

### 练习 14：对象安全 Trait

设计对象安全的 Trait：

```rust
// ❌ 不对象安全的设计
trait NotObjectSafe {
    fn clone_self(&self) -> Self;  // 返回 Self
    fn compare(&self, other: &Self) -> bool;  // Self 在参数
}

// ✅ 对象安全的设计
trait ObjectSafe {
    fn do_something(&self) -> String;
    
    // 使用 where Self: Sized 限制不安全方法
    fn clone_self(&self) -> Self where Self: Sized;
}

struct MyType {
    value: i32,
}

impl ObjectSafe for MyType {
    fn do_something(&self) -> String {
        format!("Value: {}", self.value)
    }
    
    fn clone_self(&self) -> Self where Self: Sized {
        MyType { value: self.value }
    }
}

fn main() {
    let items: Vec<Box<dyn ObjectSafe>> = vec![
        Box::new(MyType { value: 1 }),
        Box::new(MyType { value: 2 }),
    ];
    
    for item in &items {
        println!("{}", item.do_something());  // ✅ 调用对象安全方法
    }
}
```

---

### 练习 15：综合案例 - ORM 系统

实现简化的 ORM 系统：

```rust
// 要求：
trait Model {
    type PrimaryKey;
    
    fn table_name() -> &'static str;
    fn primary_key(&self) -> Self::PrimaryKey;
    fn to_insert_sql(&self) -> String;
    fn from_row(row: &Row) -> Self;
}

struct Row {
    data: HashMap<String, String>,
}

impl Row {
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

struct User {
    id: u32,
    username: String,
    email: String,
}

impl Model for User {
    type PrimaryKey = u32;
    
    fn table_name() -> &'static str {
        "users"
    }
    
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
    
    fn to_insert_sql(&self) -> String {
        format!(
            "INSERT INTO users (username, email) VALUES ('{}', '{}')",
            self.username, self.email
        )
    }
    
    fn from_row(row: &Row) -> Self {
        User {
            id: row.get("id").unwrap().parse().unwrap(),
            username: row.get("username").unwrap().clone(),
            email: row.get("email").unwrap().clone(),
        }
    }
}

struct Database {
    tables: HashMap<String, Vec<Row>>,
}

impl Database {
    fn insert<M: Model>(&mut self, model: &M) {
        // 执行插入
    }
    
    fn find<M: Model>(&self, id: M::PrimaryKey) -> Option<M> {
        // 查找记录
    }
    
    fn query<M: Model>(&self, condition: &str) -> Vec<M> {
        // 查询多条记录
    }
}

fn main() {
    let mut db = Database::new();
    
    let user = User {
        id: 1,
        username: "alice",
        email: "alice@example.com",
    };
    
    db.insert(&user);
    println!("SQL: {}", user.to_insert_sql());
    
    let found: Option<User> = db.find(1);
    println!("Found: {:?}", found);
}
```

---

## 练习题答案要点

### 答案 1：Describable 实现

```rust
impl Describable for Product {
    fn describe(&self) -> String {
        format!("Product: {} (${})", self.name, self.price)
    }
}

impl Describable for User {
    fn describe(&self) -> String {
        format!("User #{}: {}", self.id, self.username)
    }
}

impl Describable for File {
    fn describe(&self) -> String {
        format!("File: {} ({} bytes)", self.path, self.size)
    }
}
```

### 答案 7：Fibonacci Iterator

```rust
impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}
```

---

## 自我评估标准

完成所有练习后，你应该能够：

1. ✅ 定义和实现 Trait
2. ✅ 使用默认实现减少重复代码
3. ✅ 理解 Trait 作为参数和返回值
4. ✅ 掌握孤儿规则的限制
5. ✅ 实现带关联类型的 Trait
6. ✅ 自定义 Iterator
7. ✅ 实现 From/Into 转换
8. ✅ 创建智能指针（Deref/Drop）
9. ✅ 使用 Trait 对象动态分发
10. ✅ 设计对象安全的 Trait
11. ✅ 结合生命周期和 Trait
12. ✅ 实现条件方法和 blanket implementation

---

## 下一步学习

完成本章练习后，继续学习：
- 第 17 章：生命周期
- 第 18 章：闭包
- 实战项目：插件系统设计