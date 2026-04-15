//! # 示例：并发基础
//!
//! 对应章节：01-并发基础
//! 运行：cargo run --example 01-concurrency-basics

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 并发基础 ===\n");

    // ✅ 正确示例：创建线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程：{}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=3 {
        println!("主线程：{}", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();

    // ✅ 正确示例：move 闭包
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("线程中的向量：{:?}", v);
    });
    handle.join().unwrap();

    println!("\n线程完成！");
}
