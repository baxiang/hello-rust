//! # 示例：迭代器实战总结
//!
//! 对应章节：05-实战总结
//! 运行：cargo run --example 05-iterator-review

fn main() {
    println!("=== 迭代器实战总结 ===\n");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 1. 数据处理流水线
    let result: i32 = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .take(3)
        .sum();
    println!("偶数平方前3个之和：{}", result);

    // 2. 字符串处理
    let text = "hello world from rust";
    let capitalized: String = text
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("大写：{}", capitalized);

    // 3. 分组统计
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let odds: Vec<i32> = numbers.iter().filter(|&&x| x % 2 != 0).copied().collect();
    println!("偶数：{:?}", evens);
    println!("奇数：{:?}", odds);

    // 4. 自定义迭代器
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter { count: 0, max: 5 };
    let sum: u32 = counter.sum();
    println!("1+2+3+4+5 = {}", sum);

    println!("\n迭代器章节完成！");
}
