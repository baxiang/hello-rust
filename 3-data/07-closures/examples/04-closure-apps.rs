//! # 示例：闭包应用
//!
//! 对应章节：04-闭包应用
//! 运行：cargo run --example 04-closure-apps

fn main() {
    // ✅ 缓存计算结果
    struct Cacher<T, F>
    where
        T: Eq + std::hash::Hash + Clone,
        F: Fn(T) -> i32,
    {
        calculation: F,
        values: std::collections::HashMap<T, i32>,
    }

    impl<T, F> Cacher<T, F>
    where
        T: Eq + std::hash::Hash + Clone,
        F: Fn(T) -> i32,
    {
        fn new(calculation: F) -> Self {
            Cacher {
                calculation,
                values: std::collections::HashMap::new(),
            }
        }

        fn value(&mut self, arg: T) -> i32 {
            let calc = &self.calculation;
            *self.values.entry(arg.clone()).or_insert_with(|| calc(arg))
        }
    }

    let mut cacher = Cacher::new(|x| {
        println!("计算中...");
        x * 2
    });

    println!("第一次：{}", cacher.value(5));
    println!("第二次：{}", cacher.value(5)); // 缓存
    println!("不同参数：{}", cacher.value(10));

    // ✅ 迭代器中的闭包
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    let sum: i32 = numbers.iter().sum();
    println!("\n翻倍：{:?}", doubled);
    println!("求和：{}", sum);

    // 尝试修改：
    // 1. 实现自己的 Cacher
    // 2. 使用闭包处理集合
}
