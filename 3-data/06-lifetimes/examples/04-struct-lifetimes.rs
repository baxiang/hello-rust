//! # 示例：结构体生命周期
//!
//! 对应章节：04-结构体生命周期
//! 运行：cargo run --example 04-struct-lifetimes

// ✅ 正确示例：带生命周期的结构体
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.position += c.len_utf8();
        }
    }
}

// ✅ 正确示例：多个生命周期
struct Context<'a, 'b> {
    name: &'a str,
    value: &'b str,
}

fn main() {
    let input = "hello world";
    let mut parser = Parser::new(input);

    while let Some(c) = parser.current_char() {
        print!("{}", c);
        parser.advance();
    }
    println!();

    let name = "配置";
    let value = "值";
    let ctx = Context { name, value };
    println!("{}: {}", ctx.name, ctx.value);

    // 尝试修改：
    // 1. 创建自己的解析器
    // 2. 思考：结构体为什么需要生命周期？
}
