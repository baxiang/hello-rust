## 17.9 Trait 中的生命周期

### 定义带生命周期的 Trait

```rust
// Trait 可以有生命周期参数
trait Drawable<'a> {
    fn draw(&'a self) -> &'a str;
    fn get_name(&'a self) -> &'a str;
}

// 实现
struct Circle {
    radius: f64,
}

impl<'a> Drawable<'a> for Circle {
    fn draw(&'a self) -> &'a str {
        "○"
    }

    fn get_name(&'a self) -> &'a str {
        "Circle"
    }
}
```

### Trait 对象中的生命周期

```rust
trait Draw {
    fn draw(&self);
}

// 结构体持有 Trait 对象的引用
struct Screen<'a> {
    components: Vec<&'a dyn Draw>,
}

impl<'a> Screen<'a> {
    fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

// 或者使用 Box
struct OwnedScreen {
    components: Vec<Box<dyn Draw>>,
}
```

---

---

## 下一步

[完整示例](../10-完整示例.md)