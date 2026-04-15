//! # 示例：Vec 详解
//!
//! 对应章节：02-Vec详解
//! 运行：cargo run --example 02-vec-details

fn main() {
    // ✅ 正确示例：创建 Vec
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v = {:?}", v);

    // ✅ 正确示例：宏创建
    let v2 = vec![1, 2, 3];
    println!("v2 = {:?}", v2);

    // ✅ 正确示例：访问元素
    let third = &v2[2];
    println!("第三个元素：{}", third);

    if let Some(fourth) = v2.get(3) {
        println!("第四个元素：{}", fourth);
    } else {
        println!("第四个元素不存在");
    }

    // ✅ 正确示例：修改 Vec
    let mut v3 = vec![10, 20, 30];
    v3[0] = 100;
    println!("修改后：{:?}", v3);

    // ✅ 正确示例：常用方法
    println!("长度：{}", v3.len());
    println!("是否为空：{}", v3.is_empty());
    v3.pop();
    println!("pop 后：{:?}", v3);

    // 尝试修改：
    // 1. 创建一个字符串 Vec
    // 2. 尝试在遍历时修改 Vec（会失败）
    // 3. 思考：Vec 如何实现动态扩容？
}
