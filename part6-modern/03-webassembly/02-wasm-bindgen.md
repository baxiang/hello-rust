# wasm-bindgen

> 掌握 Rust WASM 与 JavaScript 的互操作技术。

## wasm-bindgen 简介

wasm-bindgen 是 Rust 和 JavaScript 之间的桥梁，允许：
- Rust 调用 JavaScript 函数
- JavaScript 调用 Rust 函数
- 共享复杂数据类型
- 操作 DOM

```
┌─────────────────────────────────────────┐
│        wasm-bindgen 互操作架构            │
├─────────────────────────────────────────┤
│                                         │
│    JavaScript                    Rust    │
│  ┌──────────┐               ┌─────────┐│
│  │ function │               │ #[wasm_ ││
│  │          │←─────────────→│ bindgen]││
│  └──────────┘               └─────────┘│
│                                         │
│    类型转换                              │
│  ┌─────────────────────────────────┐  │
│  │ String ↔ String                  │  │
│  │ Number ↔ i32, f64               │  │
│  │ Object ↔ JsValue                │  │
│  │ Array ↔ Vec                     │  │
│  └─────────────────────────────────┘  │
│                                         │
└─────────────────────────────────────────┘
```

## 基础用法

### 导出 Rust 函数

```rust
use wasm_bindgen::prelude::*;

// 简单函数
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 返回 String
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 使用 Result（错误转为 JavaScript 异常）
#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> Result<f64, JsValue> {
    if b == 0.0 {
        Err(JsValue::from_str("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

**JavaScript 调用：**

```javascript
import init, { add, greet, divide } from './pkg/my_wasm.js';

await init();

console.log(add(2, 3));           // 5
console.log(greet('World'));      // "Hello, World!"

try {
    divide(10, 0);                // 抛出异常
} catch (e) {
    console.error(e);
}
```

### 导入 JavaScript 函数

```rust
use wasm_bindgen::prelude::*;

// 导入 JavaScript 函数
#[wasm_bindgen]
extern "C" {
    // alert 函数
    pub fn alert(s: &str);
    
    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    // 自定义 JavaScript 函数
    fn custom_function(x: i32) -> i32;
}

#[wasm_bindgen]
pub fn call_js() {
    alert("Hello from WASM!");
    log("Logging from Rust");
    let result = custom_function(42);
    log(&format!("Custom function returned {}", result));
}
```

**JavaScript 实现：**

```javascript
// 提供自定义函数
window.custom_function = function(x) {
    return x * 2;
};
```

## 类型转换

### 基本类型

| Rust 类型 | JavaScript 类型 | 说明 |
|----------|----------------|------|
| `i32`, `u32` | Number | 整数 |
| `f64` | Number | 浮点数 |
| `bool` | Boolean | 布尔值 |
| `String` | String | 字符串 |
| `&str` | String | 字符串切片 |
| `JsValue` | Any | 任意 JS 值 |

### 示例

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_types(
    integer: i32,
    float: f64,
    boolean: bool,
    text: String,
) -> String {
    format!(
        "Received: int={}, float={}, bool={}, text={}",
        integer, float, boolean, text
    )
}
```

### 数组和集合

```rust
use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array};

// Vec 转换
#[wasm_bindgen]
pub fn sum_array(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

#[wasm_bindgen]
pub fn create_array() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

// Uint8Array（二进制数据）
#[wasm_bindgen]
pub fn process_bytes(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ 0xFF).collect()
}

// JavaScript Array
#[wasm_bindgen]
pub fn js_array_length(arr: Array) -> u32 {
    arr.length()
}
```

**JavaScript 调用：**

```javascript
await init();

const sum = sum_array([1, 2, 3, 4, 5]);  // 15
const arr = create_array();               // [1, 2, 3, 4, 5]

// 二进制数据
const bytes = new Uint8Array([0, 1, 2]);
const processed = process_bytes(bytes);  // Uint8Array

// JS Array
const jsArr = new Array(1, 2, 3);
const len = js_array_length(jsArr);      // 3
```

## 结构体和类

### 导出结构体

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    // 构造函数
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
    // getter
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }
    
    // setter
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, value: f64) {
        self.x = value;
    }
    
    // 方法
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

**JavaScript 使用：**

```javascript
const p1 = new Point(0, 0);
const p2 = new Point(3, 4);

console.log(p1.x);              // getter: 0
p1.x = 5;                       // setter

console.log(p1.distance(p2));   // 方法: 5.0
```

### 复杂结构体

```rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// 使用 serde 序列化复杂类型
#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u32,
    email: String,
}

#[wasm_bindgen]
pub fn parse_user(json: &str) -> Result<User, JsValue> {
    let user: User = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(user)
}

#[wasm_bindgen]
pub fn user_to_json(user: &User) -> Result<String, JsValue> {
    serde_json::to_string(user)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
```

## DOM 操作

### 基础 DOM 操作

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn getElementById(id: &str) -> Option<Element>;
    
    #[wasm_bindgen(js_namespace = document)]
    fn createElement(tag: &str) -> Element;
}

#[wasm_bindgen]
pub fn create_element() {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    
    // 创建元素
    let div = document.create_element("div")
        .expect("failed to create div");
    
    div.set_inner_html("Hello from WASM!");
    
    // 添加到 body
    document.body()
        .expect("no body")
        .append_child(&div)
        .expect("failed to append");
}
```

### 使用 web-sys

```toml
# Cargo.toml
[dependencies]
web-sys = { version = "0.3", features = [
    "Document",
    "Element",
    "Window",
    "HtmlElement",
    "console",
] }
```

```rust
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn modify_dom() {
    let document = window()
        .expect("no window")
        .document()
        .expect("no document");
    
    // 获取元素
    let element = document.get_element_by_id("target")
        .expect("element not found");
    
    // 设置内容
    element.set_inner_html("Modified by WASM!");
    
    // 设置样式
    element.set_attribute("style", "color: red; font-size: 20px;")
        .expect("failed to set style");
}
```

### 事件处理

```rust
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Event};

#[wasm_bindgen]
pub fn setup_click_handler() {
    let document = window()
        .expect("no window")
        .document()
        .expect("no document");
    
    let button = document.get_element_by_id("myButton")
        .expect("button not found")
        .dyn_into::<HtmlElement>()
        .expect("not an HtmlElement");
    
    // 创建闭包
    let closure = Closure::<dyn Fn(Event)>::new(|event: Event| {
        console::log_1(&JsValue::from_str("Button clicked!"));
    });
    
    // 添加事件监听器
    button.add_event_listener_with_callback(
        "click",
        closure.as_ref().unchecked_ref()
    ).expect("failed to add listener");
    
    // 必须忘记闭包，否则会被释放
    closure.forget();
}
```

## 异步操作

### wasm-bindgen-futures

```toml
# Cargo.toml
[dependencies]
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["fetch", "Request", "Response"] }
```

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, JSON};
use web_sys::{Request, Response};

#[wasm_bindgen]
pub async fn fetch_data(url: &str) -> Result<String, JsValue> {
    // 创建请求
    let request = Request::new_with_str(url)?;
    
    // 发送请求
    let promise = window()
        .expect("no window")
        .fetch_with_request(&request);
    
    // 等待响应
    let response: Response = JsFuture::from(promise)
        .await?
        .dyn_into()?;
    
    // 获取文本
    let text_promise = response.text()?;
    let text: String = JsFuture::from(text_promise)
        .await?
        .as_string()
        .expect("response not string");
    
    Ok(text)
}
```

### setTimeout

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub fn delayed_greeting(seconds: f64) -> Promise {
    future_to_promise(async move {
        // 等待
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            web_sys::window()
                .expect("no window")
                .set_timeout_with_callback_and_timeout_arguments(
                    &resolve,
                    (seconds * 1000.0) as i32,
                    &js_sys::Array::new()
                )
                .expect("failed to set timeout");
        });
        
        JsFuture::from(promise).await?;
        
        Ok(JsValue::from_str("Hello after delay!"))
    })
}
```

## JavaScript 对象

### JsValue 操作

```rust
use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect, Array};

#[wasm_bindgen]
pub fn create_object() -> Object {
    let obj = Object::new();
    
    // 设置属性
    Reflect::set(
        &obj,
        &JsValue::from_str("name"),
        &JsValue::from_str("Alice")
    ).expect("failed to set");
    
    Reflect::set(
        &obj,
        &JsValue::from_str("age"),
        &JsValue::from_f64(30.0)
    ).expect("failed to set");
    
    obj
}

#[wasm_bindgen]
pub fn read_object(obj: &Object) -> String {
    let name = Reflect::get(
        &obj,
        &JsValue::from_str("name")
    ).expect("failed to get name")
     .as_string()
     .expect("name not string");
    
    let age = Reflect::get(
        &obj,
        &JsValue::from_str("age")
    ).expect("failed to get age")
     .as_f64()
     .expect("age not number") as u32;
    
    format!("Name: {}, Age: {}", name, age)
}
```

### JSON 处理

```rust
use js_sys::JSON;

#[wasm_bindgen]
pub fn parse_json(json_str: &str) -> Result<Object, JsValue> {
    let value = JSON::parse(json_str)?;
    value.dyn_into()
}

#[wasm_bindgen]
pub fn stringify_object(obj: &Object) -> Result<String, JsValue> {
    let value = JSON::stringify(&obj)?;
    value.as_string()
        .ok_or_else(|| JsValue::from_str("Failed to stringify"))
}
```

## 内存管理

### 避免频繁复制

```rust
// ❌ 低效：频繁复制
#[wasm_bindgen]
pub fn process_string(s: String) -> String {
    s.to_uppercase()  // 复制两次
}

// ✅ 高效：使用引用
#[wasm_bindgen]
pub fn process_string_ref(s: &str) -> String {
    s.to_uppercase()  // 只复制一次
}

// ✅ 高效：批量处理
#[wasm_bindgen]
pub fn process_batch(data: Vec<u8>) -> Vec<u8> {
    // 一次性处理大量数据
    data.iter().map(|b| b * 2).collect()
}
```

### 预分配内存

```rust
#[wasm_bindgen]
pub fn allocate_buffer() -> Vec<u8> {
    // 预分配大小
    Vec::with_capacity(1024)
}

#[wasm_bindgen]
pub fn fill_buffer(mut buffer: Vec<u8>, data: &[u8]) {
    buffer.clear();
    buffer.extend_from_slice(data);
}
```

## 错误处理

### Result 与异常

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn may_fail(input: i32) -> Result<i32, JsValue> {
    if input < 0 {
        Err(JsValue::from_str("Input must be positive"))
    } else {
        Ok(input * 2)
    }
}

// JavaScript 中捕获异常
try {
    may_fail(-1);
} catch (e) {
    console.error(e);  // "Input must be positive"
}
```

### 自定义错误类型

```rust
use wasm_bindgen::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasmError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Processing failed")]
    ProcessingFailed,
}

impl From<WasmError> for JsValue {
    fn from(error: WasmError) -> JsValue {
        JsValue::from_str(&error.to_string())
    }
}

#[wasm_bindgen]
pub fn validate(input: &str) -> Result<String, WasmError> {
    if input.is_empty() {
        Err(WasmError::InvalidInput("empty string".to_string()))
    } else {
        Ok(input.to_uppercase())
    }
}
```

## 小结

**wasm-bindgen 核心功能：**

| 功能 | Rust | JavaScript |
|------|------|-----------|
| 导出函数 | `#[wasm_bindgen]` | 直接调用 |
| 导入函数 | `extern "C"` | 实现 JS 函数 |
| 结构体 | `#[wasm_bindgen] struct` | JS 类 |
| DOM 操作 | web-sys | DOM API |
| 异步 | wasm-bindgen-futures | Promise |

**类型转换：**
- 基本类型：自动转换
- 字符串：String ↔ String
- 数组：Vec ↔ Array/TypedArray
- 对象：JsValue/Object ↔ JS Object

**最佳实践：**
- 使用引用避免频繁复制
- 批量处理减少跨边界调用
- 预分配内存优化性能
- 正确处理错误和异常

**下一步：**
下一节我们将通过实战项目巩固 WASM 开发技能。

## 练习

### 练习 1：导出结构体

创建一个 WASM 结构体：

```rust
// TODO: 创建 Counter 结构体
// TODO: 实现构造函数、getter、方法
// TODO: 在 JavaScript 中使用
```

### 练习 2：DOM 操作

使用 WASM 操作 DOM：

```rust
// TODO: 创建 DOM 元素
// TODO: 设置内容、样式
// TODO: 添加事件监听器
```

### 练习 3：异步操作

实现异步 WASM 函数：

```rust
// TODO: 使用 wasm-bindgen-futures
// TODO: fetch 远程数据
// TODO: 处理 Promise
```