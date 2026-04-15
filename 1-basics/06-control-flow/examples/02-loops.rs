//! # 示例：循环
//!
//! 对应章节：02-循环
//! 运行：cargo run --example 02-loops

fn main() {
    // ✅ 正确示例：loop 无限循环
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2;
        }
    };
    println!("loop 结果：{}", result);

    // ✅ 正确示例：while 循环
    let mut number = 3;
    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射！");

    // ✅ 正确示例：for 循环
    let numbers = [10, 20, 30, 40, 50];
    println!("\nfor 循环遍历数组：");
    for num in numbers {
        println!("{}", num);
    }

    // ✅ 正确示例：范围循环
    println!("\n范围循环：");
    for i in 1..=5 {
        println!("{}", i);
    }

    // ✅ 正确示例：带标签的循环
    'outer: for x in 1..=3 {
        for y in 1..=3 {
            if x == 2 && y == 2 {
                break 'outer;
            }
            println!("x={}, y={}", x, y);
        }
    }

    // 尝试修改：
    // 1. 使用 continue 跳过某些迭代
    // 2. 创建一个嵌套循环
    // 3. 思考：什么时候用 loop，什么时候用 for？
}
