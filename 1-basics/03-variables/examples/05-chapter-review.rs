//! # 示例：章节综合练习
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 05-chapter-review

// 练习：实现一个简单的计算器变量管理

const TAX_RATE: f64 = 0.08; // 税率 8%

fn main() {
    // 1. 声明商品价格（不可变）
    let price: f64 = 99.99;
    println!("商品价格：¥{:.2}", price);

    // 2. 计算税额（使用常量）
    let tax = price * TAX_RATE;
    println!("税额：¥{:.2}", tax);

    // 3. 总价（可变，用于后续调整）
    let mut total = price + tax;
    println!("总价：¥{:.2}", total);

    // 4. 应用折扣
    let discount = 0.9; // 9 折
    total *= discount;
    println!("折扣后：¥{:.2}", total);

    // 5. 变量遮蔽：改变类型
    let total = total.round(); // 从 f64 变为整数
    println!("取整后：¥{:.0}", total);

    // 练习题：
    // 1. 尝试修改 TAX_RATE 为 0.1，观察结果
    // 2. 将 price 改为 mut，尝试在运行时修改价格
    // 3. 添加一个 static 变量存储店铺名称

    println!("\n练习完成！尝试修改代码实验不同的结果。");
}
