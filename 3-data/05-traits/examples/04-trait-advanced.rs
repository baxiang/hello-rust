//! # 示例：高级特性
//!
//! 对应章节：04-高级特性
//! 运行：cargo run --example 04-trait-advanced

// ✅ 正确示例：关联类型
trait Graph {
    type Node;
    type Edge;

    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self) -> Vec<Self::Edge>;
}

struct SimpleGraph;

impl Graph for SimpleGraph {
    type Node = i32;
    type Edge = (i32, i32);

    fn nodes(&self) -> Vec<Self::Node> {
        vec![1, 2, 3]
    }

    fn edges(&self) -> Vec<Self::Edge> {
        vec![(1, 2), (2, 3)]
    }
}

// ✅ 正确示例：默认类型参数
trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

// ✅ 正确示例：父 Trait
trait Describe: std::fmt::Display {
    fn describe(&self) -> String {
        format!("Item: {}", self)
    }
}

impl Describe for i32 {}

fn main() {
    let graph = SimpleGraph;
    println!("节点：{:?}", graph.nodes());
    println!("边：{:?}", graph.edges());

    // 父 Trait
    let num = 42;
    println!("描述：{}", num.describe());

    // 尝试修改：
    // 1. 实现自己的关联类型 Trait
    // 2. 思考：关联类型 vs 泛型参数？
}
