//! # 示例：数组与 Vec 综合
//!
//! 对应章节：03-数组与Vec
//! 运行：cargo run --example 03-array-vec

fn main() {
    // ✅ 数组 vs Vec 对比
    let array = [1, 2, 3]; // 固定大小
    let vec = vec![1, 2, 3]; // 动态大小

    println!("数组：{:?}，长度：{}", array, array.len());
    println!("Vec：{:?}，长度：{}", vec, vec.len());

    // ✅ 数组转 Vec
    let v = array.to_vec();
    println!("数组转 Vec：{:?}", v);

    // ✅ Vec 转数组
    let arr: [i32; 3] = vec.clone().try_into().unwrap();
    println!("Vec 转数组：{:?}", arr);

    // ✅ 切片共享
    let slice = &vec[1..];
    println!("切片：{:?}", slice);

    // ✅ 迭代
    println!("迭代数组：");
    for item in &array {
        println!("  {}", item);
    }

    // 尝试修改：
    // 1. 什么时候用数组，什么时候用 Vec？
    // 2. 尝试使用迭代器方法
    // 3. 思考：性能差异在哪里？
}
