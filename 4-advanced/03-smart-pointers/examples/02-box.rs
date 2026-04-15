//! # 示例：Box 智能指针
//!
//! 对应章节：02-Box
//! 运行：cargo run --example 02-box

// ✅ 正确示例：Box 在堆上分配
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // ✅ 递归类型
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    match &list {
        List::Cons(val, next) => println!("第一个值：{}，还有更多", val),
        List::Nil => println!("空列表"),
    }

    // ✅ Box 用于 Trait 对象
    trait Draw {
        fn draw(&self);
    }

    struct Button;
    impl Draw for Button {
        fn draw(&self) {
            println!("绘制按钮");
        }
    }

    let components: Vec<Box<dyn Draw>> = vec![Box::new(Button)];
    for c in &components {
        c.draw();
    }

    // 尝试修改：
    // 1. 创建自己的递归类型
    // 2. 思考：什么时候使用 Box？
}
