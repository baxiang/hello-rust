## 24.7 Union

### 定义和使用

```rust
#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
    bytes: [u8; 4],
}

fn main() {
    // 创建 union
    let u = MyUnion { i: 42 };

    // 访问 union 字段需要 unsafe
    unsafe {
        println!("i: {}", u.i);
        println!("f: {}", u.f);  // 同样的位，解释为浮点数
        println!("bytes: {:?}", u.bytes);
    }
}
```

### Union 应用

```rust
#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process(data: IntOrFloat, is_int: bool) {
    unsafe {
        if is_int {
            println!("整数：{}", data.i);
        } else {
            println!("浮点数：{}", data.f);
        }
    }
}
```

---

---

## 下一步

[完整示例](../8-完整示例.md)