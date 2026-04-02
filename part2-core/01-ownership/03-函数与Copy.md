# 函数与所有权

> 所有权在函数调用中的传递


## 参数传递：移动所有权

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s 的所有权被移动

    // println!("{}", s);  // ❌ 错误：s 已无效

    let x = 5;
    makes_copy(x);  // x 被复制（i32 实现 Copy）

    println!("x = {}", x);  // ✅ 正确：x 仍然有效
}

fn takes_ownership(some_string: String) {
    // some_string 获得所有权
    println!("{}", some_string);
}  // some_string 离开作用域，数据被释放

fn makes_copy(some_integer: i32) {
    // some_integer 是 x 的副本
    println!("{}", some_integer);
}  // 没有堆数据，无需释放
```



```rust
fn main() {
    // 函数返回所有权
    let s1 = gives_ownership();
    // s1 现在拥有 "hello" 的所有权

    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    // s2 的所有权移入函数，然后 s3 获得返回的所有权
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string  // 返回，所有权转移给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 获得所有权
    a_string  // 返回，所有权转移给调用者
}
```


## 所有权流动图解

```
fn gives_ownership() -> String {
    let s = String::from("hello");
    s  // ← 返回
}

fn main() {
    let s1 = gives_ownership();
    //     ↑
    //     所有权从函数流向 main
}
```


## 实际模式

### ❌ 不好：每次都移动

```rust
fn main() {
    let s = String::from("hello");
    let s = take_and_return(s);  // 取回所有权
    let s = take_and_return(s);  // 再次取回
}

fn take_and_return(s: String) -> String {
    println!("{}", s);
    s  // 返回
}
```

### ✅ 更好：使用引用

```rust
fn main() {
    let s = String::from("hello");
    just_borrow(&s);  // 不移动
    just_borrow(&s);  // 可以再次使用
}

fn just_borrow(s: &String) {
    println!("{}", s);
}
```

### ✅ 最佳：接受 &str

```rust
fn main() {
    let s = String::from("hello");
    print_str(&s);  // &String → &str
    print_str("literal");  // &'static str
}

fn print_str(s: &str) {
    println!("{}", s);
}
```



[Copy trait](07-Copy-trait.md)


# Copy trait

> 自动复制的类型


## 什么是 Copy trait？

实现了 `Copy` trait 的类型，赋值时自动复制而不是移动：

```rust
fn main() {
    let x = 5;
    let y = x;  // 自动复制，不是移动

    println!("x = {}, y = {}", x, y);  // x 仍然有效
}
```


## Copy 类型列表

```rust
fn main() {
    // 所有整数类型
    let a: i8 = 1;
    let b: i16 = 2;
    let c: i32 = 3;
    let d: i64 = 4;
    let e: i128 = 5;
    let f: isize = 6;
    let g: u8 = 7;
    let h: u16 = 8;
    let i: u32 = 9;
    let j: u64 = 10;
    let k: u128 = 11;
    let l: usize = 12;

    // 所有浮点类型
    let m: f32 = 3.14;
    let n: f64 = 2.71;

    // 布尔类型
    let o: bool = true;

    // 字符类型
    let p: char = 'A';

    // 所有元素都 Copy 的元组
    let q: (i32, i32) = (1, 2);
    let r = q;
    println!("q = {:?}", q);  // ✅ q 仍然有效

    // 所有元素都 Copy 的数组
    let s: [i32; 3] = [1, 2, 3];
    let t = s;
    println!("s = {:?}", s);  // ✅ s 仍然有效
}
```


## 元组和数组的 Copy 规则

### 元组

```rust
// ✅ Copy：所有元素都是 Copy
let t1 = (1, 2, 3);
let t2 = t1;
println!("{:?}", t1);  // 有效

// ❌ 非 Copy：包含非 Copy 元素
let t3 = (String::from("hello"), 5);
let t4 = t3;
// println!("{:?}", t3);  // 错误：t3 已移动
```

### 数组

```rust
// ✅ Copy：元素是 Copy
let a1 = [1, 2, 3];
let a2 = a1;
println!("{:?}", a1);  // 有效

// ❌ 非 Copy：元素不是 Copy
let a3 = [String::from("a"), String::from("b")];
let a4 = a3;
// println!("{:?}", a3);  // 错误：a3 已移动
```


## Copy 类型总结表

| 类型 | 是否 Copy | 条件 |
|------|----------|------|
| `(i32, i32)` | ✅ | 所有元素 Copy |
| `(String, i32)` | ❌ | String 不是 Copy |
| `[i32; 5]` | ✅ | 元素 i32 是 Copy |
| `[String; 3]` | ❌ | String 不是 Copy |


## Copy vs Clone

```
┌─────────────────────────────────────────────────────┐
│              Copy vs Clone                          │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Copy                                               │
│  ├── 编译时自动复制                                 │
│  ├── 只复制栈数据（位复制）                         │
│  ├── 零开销                                         │
│  └── 适用于简单类型（整数、浮点、bool、char）        │
│                                                     │
│  Clone                                              │
│  ├── 显式调用 .clone()                              │
│  ├── 可能复制堆数据（深拷贝）                       │
│  ├── 可能有性能开销                                 │
│  └── 适用于复杂类型（String、Vec、自定义类型）       │
│                                                     │
└─────────────────────────────────────────────────────┘
```



[常见错误](08-常见错误.md)



