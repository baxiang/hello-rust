## 10.8 多个 impl 块

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 第一个 impl 块
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 第二个 impl 块
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// 第三个 impl 块（可以用于条件编译）
#[cfg(debug_assertions)]
impl Rectangle {
    fn debug_print(&self) {
        println!("[DEBUG] Rectangle: {}x{}", self.width, self.height);
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };

    println!("面积：{}", rect.area());
    println!("周长：{}", rect.perimeter());
    println!("是正方形：{}", rect.is_square());

    #[cfg(debug_assertions)]
    rect.debug_print();
}
```




