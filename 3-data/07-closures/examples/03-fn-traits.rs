//! # 示例：Fn Trait
//!
//! 对应章节：03-FnTrait
//! 运行：cargo run --example 03-fn-traits

// ✅ Fn: 不可变借用，可多次调用
fn call_fn<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// ✅ FnMut: 可变借用
fn call_fn_mut<F>(mut f: F, times: usize)
where
    F: FnMut(),
{
    for _ in 0..times {
        f();
    }
}

// ✅ FnOnce: 获取所有权
fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let double = |x| x * 2;
    println!("call_fn(double, 5) = {}", call_fn(double, 5));

    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("count = {}", count);
    };
    call_fn_mut(increment, 3);

    let msg = String::from("hello");
    let once = || println!("{}", msg);
    call_fn_once(once);

    // 尝试修改：
    // 1. 创建自己的 Fn Trait 函数
    // 2. 思考：三种 Trait 的层次关系？
}
