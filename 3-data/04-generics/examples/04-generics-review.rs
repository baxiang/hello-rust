//! # 示例：泛型实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-generics-review

use std::fmt::Display;

// ✅ 正确示例：综合泛型应用
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x >= y: {} >= {}", self.x, self.y);
        } else {
            println!("x < y: {} < {}", self.x, self.y);
        }
    }
}

// ✅ 正确示例：泛型工厂
fn create_vec<T>(item: T, count: usize) -> Vec<T>
where
    T: Clone,
{
    vec![item; count]
}

fn main() {
    println!("=== 泛型实战总结 ===\n");

    // 1. 泛型结构体
    let pair = Pair::new(10, 20);
    pair.cmp_display();

    // 2. 泛型工厂
    let strings = create_vec("hello", 3);
    let numbers = create_vec(42, 5);
    println!("\nstrings: {:?}", strings);
    println!("numbers: {:?}", numbers);

    // 3. 泛型函数组合
    fn process<T: Display + Clone>(items: &[T]) -> String {
        items
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }

    let data = vec![1, 2, 3, 4];
    println!("\nprocess: {}", process(&data));

    // 4. 总结
    println!("\n泛型优势：");
    println!("  - 避免代码重复");
    println!("  - 编译时类型检查");
    println!("  - 零运行时开销");
    println!("  - 与 Trait 配合提供灵活性");

    println!("\n泛型章节完成！");
}
