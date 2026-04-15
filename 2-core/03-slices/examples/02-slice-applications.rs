//! # 示例：切片应用
//!
//! 对应章节：02-切片应用
//! 运行：cargo run --example 02-slice-applications

fn main() {
    // ✅ 正确示例：第一个单词函数
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词：{}", word);

    // ✅ 正确示例：修改字符串后切片
    let mut s2 = String::from("hello rust");
    let word = first_word(&s2);
    println!("修改前：{}", word);
    s2.clear(); // 清空字符串
                // word 现在指向无效内存（编译时保证安全）
                // println!("清空后：{}", word);  // ❌ 借用冲突

    // ✅ 正确示例：切片作为参数
    let a = [1, 2, 3, 4, 5];
    println!("数组和：{}", sum_slice(&a));

    let sub = &a[1..4];
    println!("子数组和：{}", sum_slice(sub));

    // ✅ 正确示例：字符串处理
    let text = "  Hello, World!  ";
    let trimmed = text.trim();
    println!("trim: '{}'", trimmed);

    // 尝试修改：
    // 1. 创建 last_word 函数
    // 2. 尝试使用切片修改原字符串（会失败）
    // 3. 思考：切片如何避免悬垂指针？
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
