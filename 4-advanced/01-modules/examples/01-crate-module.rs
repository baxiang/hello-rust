//! # 示例：Crate 与模块
//!
//! 对应章节：01-Crate与模块
//! 运行：cargo run --example 01-crate-module

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等候名单");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("接受订单");
        }
    }
}

fn main() {
    // ✅ 正确示例：使用绝对路径
    front_of_house::hosting::add_to_waitlist();

    // ✅ 正确示例：使用 use 引入
    use front_of_house::serving;
    serving::take_order();

    // 尝试修改：
    // 1. 添加更多模块
    // 2. 思考：pub 关键字的作用？
}
