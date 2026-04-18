# Fn Trait

> 掌握 Fn、FnMut、FnOnce 三种 Trait 的区别和用法，学会作为参数和返回值使用闭包。

## Fn、FnMut、FnOnce Trait

### 三种 Trait 详解

```
┌─────────────────────────────────────────────────────┐
│        闭包的三种 Trait（继承关系）                  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  FnOnce（基类）                                     │
│  ├── fn call_once(self, args) -> Output            │
│  ├── 消耗 self（获取所有权）                        │
│  └── 所有闭包都实现 FnOnce                         │
│                                                     │
│  FnMut: FnOnce                                      │
│  ├── fn call_mut(&mut self, args) -> Output        │
│  ├── 可变借用 self                                  │
│  └── 可以多次调用，可能修改捕获的变量              │
│                                                     │
│  Fn: FnMut                                          │
│  ├── fn call(&self, args) -> Output                │
│  ├── 不可变借用 self                                │
│  └── 可以多次调用，不修改捕获的变量                │
│                                                     │
│  继承关系：Fn ⊂ FnMut ⊂ FnOnce                       │
│  • 实现 Fn 的自动实现 FnMut 和 FnOnce               │
│  • 实现 FnMut 的自动实现 FnOnce                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 判断闭包类型

```rust
fn main() {
    // ========== Fn（不可变借用）==========
    let x = 5;
    let f = |y| x + y;
    // f 只读取 x，不修改
    // 可以调用多次
    println!("{}", f(1));  // 6
    println!("{}", f(2));  // 7

    // ========== FnMut（可变借用）==========
    let mut counter = 0;
    let mut f = || {
        counter += 1;  // 修改 counter
        counter
    };
    // f 修改了 counter，需要 mut
    println!("{}", f());  // 1
    println!("{}", f());  // 2

    // ========== FnOnce（获取所有权）==========
    let s = String::from("hello");
    let f = move || {
        println!("{}", s);  // 消耗 s
    };
    // f 消耗了 s，只能调用一次
    f();
    // f();  // ❌ 错误：f 已被消耗
}
```

### 作为函数参数

```rust
// 接受 FnOnce（最宽松）
fn call_once<F: FnOnce()>(f: F) {
    f();
}

// 接受 FnMut
fn call_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

// 接受 Fn（最严格）
fn call_fn<F: Fn()>(f: F) {
    f();
    f();
    f();
}

fn main() {
    // Fn 闭包可以传递给任何函数
    let f = || println!("Fn");
    call_fn(|| println!("Fn"));
    call_mut(|| println!("Fn"));
    call_once(|| println!("Fn"));

    // FnMut 闭包只能传递给 FnMut 和 FnOnce
    let mut count = 0;
    call_mut(|| {
        count += 1;
        println!("count = {}", count);
    });
    call_once(|| println!("FnMut"));

    // FnOnce 闭包只能传递给 FnOnce
    let s = String::from("hello");
    call_once(move || println!("{}", s));
}
```

### 返回闭包

```rust
// 返回 impl Trait（推荐）
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 返回 Box<dyn Trait>（需要动态分发时）
fn make_adder_boxed(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn main() {
    let add_5 = make_adder(5);
    let add_10 = make_adder(10);

    println!("{}", add_5(3));   // 8
    println!("{}", add_10(3));  // 13
}




---

## 小结

- Fn：不可变借用，可多次调用，不修改捕获变量
- FnMut：可变借用，可多次调用，可能修改捕获变量
- FnOnce：获取所有权，只能调用一次
- Trait 继承关系：Fn ⊂ FnMut ⊂ FnOnce
- 使用 `impl Trait` 或 `Box<dyn Trait>` 返回闭包

## 练习题

详见：[练习题](../../exercises/07-closures.md)
```







