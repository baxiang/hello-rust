## 16.10 Trait 对象（动态分发）

### 什么是 Trait 对象

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("绘制圆，半径 = {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("绘制矩形，{}x{}", self.width, self.height);
    }
}

// Trait 对象：dyn Draw
fn main() {
    // 存储不同类型的 Trait 对象
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];

    // 动态调用各自的方法
    for shape in shapes {
        shape.draw();
    }
}
```

### 动态 vs 静态分发

```
┌─────────────────────────────────────────────────────┐
│          动态分发 vs 静态分发                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  静态分发（泛型/impl Trait）                        │
│  ├── 编译时确定调用哪个方法                        │
│  ├── 直接函数调用，无开销                          │
│  ├── 代码可能膨胀（单态化）                        │
│  └── 类型必须在编译时已知                          │
│                                                     │
│  动态分发（Trait 对象/dyn Trait）                    │
│  ├── 运行时通过虚函数表查找方法                    │
│  ├── 有小开销（间接调用）                          │
│  ├── 代码不膨胀                                    │
│  └── 可以在运行时存储不同类型                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```




