## 8.3 可变引用（&mut T）

### 基本语法

```rust
fn main() {
    // 必须声明为 mut
    let mut s = String::from("hello");

    // 创建可变引用
    let r = &mut s;

    // 通过引用修改数据
    r.push_str(", world!");

    println!("{}", s);  // "hello, world!"
}
```

### 函数中修改数据

```rust
fn main() {
    let mut greeting = String::from("hello");

    make_friendly(&mut greeting);

    println!("{}", greeting);  // "hello, nice to meet you!"
}

fn make_friendly(s: &mut String) {
    s.push_str(", nice to meet you!");
}
```

### 可变引用的限制

```rust
// 限制 1：同一作用域内只能有一个可变引用
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;  // ❌ 错误！

    r1.push_str(" world");
    println!("{}", r1);
}
```

**错误信息：**
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;  // ❌ 错误！
  |              ^^^^^^ second mutable borrow occurs here
6 |     r1.push_str(" world");
  |     --------------------- first borrow later used here
```

```rust
// 限制 2：不能同时存在可变引用和不可变引用
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // 不可变引用
    let r2 = &mut s;  // ❌ 错误！

    println!("{}, {}", r1, r2);
}
```

**错误信息：**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:14
  |
4 |     let r1 = &s;      // 不可变引用
  |              -- immutable borrow occurs here
5 |     let r2 = &mut s;  // ❌ 错误！
  |              ^^^^^^ mutable borrow occurs here
6 |     println!("{}, {}", r1, r2);
  |                        ------ immutable borrow later used here
```

### 正确的使用方式

```rust
// 方案 1：分离作用域
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(", world");
    }  // r1 作用域结束，借用结束

    let r2 = &s;  // ✅ 现在可以了
    println!("{}", r2);
}

// 方案 2：按顺序使用
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    println!("{}", r1);  // 使用完 r1

    let r2 = &mut s;  // ✅ 现在可以借用了
    r2.push_str(" world");
    println!("{}", r2);
}
```




