//! # 示例：迭代器适配器
//!
//! 对应章节：03-迭代器适配器
//! 运行：cargo run --example 03-iterator-adapters

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ✅ map: 转换
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("squared: {:?}", squared);

    // ✅ filter: 过滤
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // ✅ zip: 组合
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let paired: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("paired: {:?}", paired);

    // ✅ enumerate: 索引
    for (i, val) in numbers.iter().enumerate() {
        if i >= 3 {
            break;
        }
        println!("  [{}] = {}", i, val);
    }

    // ✅ chain: 链式连接
    let a = vec![1, 2];
    let b = vec![3, 4];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chained: {:?}", chained);

    // ✅ take/skip: 切片
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_two: Vec<&i32> = numbers.iter().skip(2).collect();
    println!("first 3: {:?}", first_three);
    println!("skip 2: {:?}", skip_two);

    // ✅ flat_map: 扁平化
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flat_map(|v| v).collect();
    println!("flat: {:?}", flat);

    // 尝试修改：
    // 1. 组合多个适配器
    // 2. 思考：lazy evaluation 的优势？
}
