## macro_rules! 基础

### 宏语法

**概念名称：** 声明宏在编译时生成代码，使用模式匹配。

```
语法结构：
┌──────────────────────────────────────┐
│  macro_rules! 宏名 {                  │
│      (模式) => { 展开代码 };          │
│      (模式2) => { 展开代码2 };        │
│  }                                    │
│                                       │
│  macro_rules! say_hello {            │
│      () => {                         │
│          println!("Hello!");         │
│      };                               │
│  }                                    │
│                                       │
│  带参数：                              │
│  ($name:ident) => { ... }            │
│                                       │
│  调用：宏名!(参数);                   │
└──────────────────────────────────────┘
```

### 最简示例

```rust
macro_rules! say_hello {
    () => { println!("Hello!"); };
}

fn main() {
    say_hello!();
}
```

### 基本语法

```rust
// 定义宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();  // 调用宏
}
```

### 带参数的宏

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("函数 {:?} 被调用", stringify!($func_name));
        }
    };
}

// 使用宏生成函数
create_function!(foo);
create_function!(bar);

fn main() {
    foo();  // 函数 "foo" 被调用
    bar();  // 函数 "bar" 被调用
}
```

### 宏展开

```rust
// 宏调用
say_hello!();

// 展开后（概念上）
{
    println!("Hello!");
}
```






## 宏片段说明符

### 常见片段类型

```
┌─────────────┬────────────────────────────────────┐
│ 说明符      │ 匹配内容                           │
├─────────────┼────────────────────────────────────┤
│ item        │ 项（fn, struct, mod 等）            │
│ block       │ 代码块 { ... }                      │
│ stmt        │ 语句                                │
│ pat         │ 模式（pattern）                     │
│ expr        │ 表达式                              │
│ ty          │ 类型                                │
│ ident       │ 标识符（变量名、函数名等）          │
│ literal     │ 字面量（数字、字符串等）            │
│ tt          │ token 树（任意 tokens）             │
│ meta        │ 元属性（derive 参数等）             │
│ path        │ 路径（如 std::mem::size_of）        │
│ vis         │ 可见性限定符（pub, pub(crate) 等）  │
└─────────────┴────────────────────────────────────┘
```

### 片段示例

```rust
macro_rules! demo {
    // 标识符
    ($i:ident) => { /* my_var */ };

    // 代码块
    ($b:block) => { /* { ... } */ };

    // 表达式
    ($e:expr) => { /* 1 + 2 */ };

    // 类型
    ($t:ty) => { /* i32, String */ };

    // 模式
    ($p:pat) => { /* Some(x) */ };

    // 字面量
    ($l:literal) => { /* 42, "hello" */ };

    // token 树
    ($tt:tt) => { /* 任意 tokens */ };

    // 路径
    ($path:path) => { /* std::mem::size_of */ };

    // 可见性
    ($vis:vis) => { /* pub, pub(crate) */ };
}
```






## 重复匹配

### * 和 + 重复

```rust
// 匹配零次或多次（*）
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$(String::from($x)),*]
    };
}

// 匹配一次或多次（+）
macro_rules! one_or_more {
    ($($x:expr),+) => {
        vec![$($x),+]
    };
}

fn main() {
    let v = vec_of_strings!["hello", "world", "rust"];
    println!("{:?}", v);

    let v2 = one_or_more![1, 2, 3];
    println!("{:?}", v2);
}
```

### 分隔符

```rust
macro_rules! multi_match {
    // 逗号分隔
    ($($x:expr),*) => { /* 1, 2, 3 */ };

    // 分号分隔
    ($($x:expr);*) => { /* 1; 2; 3 */ };

    // 空格分隔
    ($($x:expr) *) => { /* 1 2 3 */ };

    // 无分隔符
    ($($x:tt)*) => { /* 任意 tokens */ };
}
```

### 重复嵌套

```rust
macro_rules! matrix {
    ($($( $val:expr ),*);*) => {
        {
            println!("矩阵：");
            $(
                $(
                    print!("{} ", $val);
                )*
                println!();
            )*
        }
    };
}

fn main() {
    matrix! {
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    }
}
```






## 多个匹配规则

### 规则匹配

```rust
macro_rules! print_type {
    // 规则 1：只有表达式
    ($e:expr) => {
        println!("表达式：{:?} = {:?}", stringify!($e), $e);
    };

    // 规则 2：表达式 + 消息
    ($e:expr, $msg:expr) => {
        println!("{} = {:?}", $msg, $e);
    };

    // 规则 3：表达式 + 消息 + 前缀
    ($e:expr, $msg:expr, $prefix:expr) => {
        println!("{}: {} = {:?}", $prefix, $msg, $e);
    };
}

fn main() {
    print_type!(42);                    // 规则 1
    print_type!(42, "答案");            // 规则 2
    print_type!(42, "答案", "数学");    // 规则 3
}
```

### 可选参数

```rust
macro_rules! myprintln {
    ($e:expr) => {
        println!("{}", $e);
    };
    ($e:expr, $prefix:expr) => {
        println!("{}: {}", $prefix, $e);
    };
}

fn main() {
    myprintln!("Hello");              // 无 prefix
    myprintln!("World", "Greeting");  // 有 prefix
}
```







