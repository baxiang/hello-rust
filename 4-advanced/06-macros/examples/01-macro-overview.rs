//! # 示例：宏概述
//!
//! 对应章节：01-宏概述
//! 运行：cargo run --example 01-macro-overview

// ✅ 正确示例：声明宏
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

// ✅ 正确示例：带参数宏
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("函数 {:?} 被调用", stringify!($func_name));
        }
    };
}

// ✅ 正确示例：重复宏
macro_rules! vec_of {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    say_hello!();
    foo();
    bar();

    let v = vec_of![1, 2, 3, 4, 5];
    println!("vec_of! 创建：{:?}", v);

    // ✅ 内置宏
    println!("line: {}", line!());
    println!("file: {}", file!());
    println!("column: {}", column!());

    // 尝试修改：
    // 1. 创建自己的宏
    // 2. 思考：宏和函数的区别？
}
