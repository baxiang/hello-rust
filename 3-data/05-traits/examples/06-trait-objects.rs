//! # 示例：Trait 对象
//!
//! 对应章节：06-Trait对象
//! 运行：cargo run --example 06-trait-objects

// ✅ 正确示例：Trait 对象
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮：{}", self.label);
    }
}

struct TextField {
    placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("绘制文本框：{}", self.placeholder);
    }
}

// 使用 Trait 对象
fn render(components: &[&dyn Draw]) {
    for component in components {
        component.draw();
    }
}

fn main() {
    let button = Button {
        label: "点击".into(),
    };
    let text = TextField {
        placeholder: "输入...".into(),
    };

    let components: Vec<&dyn Draw> = vec![&button, &text];
    render(&components);

    // 尝试修改：
    // 1. 添加更多组件
    // 2. 思考：Trait 对象的性能开销？
    // 3. 对比泛型 vs Trait 对象
}
