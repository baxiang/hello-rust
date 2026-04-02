## 4.6 元组（Tuple）

### 创建和解构元组

```rust
fn main() {
    // 创建元组
    let point: (i32, i32, i32) = (1, 2, 3);
    let mixed = (42, "hello", true, 3.14);

    println!("point = {:?}", point);
    println!("mixed = {:?}", mixed);

    // 方法 1：模式匹配解构
    let (x, y, z) = point;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // 只取部分值（用_忽略其他）
    let (first, _, _) = point;
    println!("first = {}", first);

    // 方法 2：索引访问
    println!("point.0 = {}", point.0);
    println!("point.1 = {}", point.1);
    println!("point.2 = {}", point.2);

    // 嵌套元组
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("a={}, b={}, c={}, d={}", a, b, c, d);
}
```

### 元组作为函数返回值

```rust
// 返回多个值的函数
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];

    for &n in numbers.iter() {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
    }

    (min, max)
}

// 返回命名信息
fn get_person() -> (String, u8, &str) {
    (String::from("张三"), 25, "北京")
}

fn main() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&numbers);
    println!("最小值：{}，最大值：{}", min, max);

    let (name, age, city) = get_person();
    println!("姓名：{}，年龄：{}，城市：{}", name, age, city);

    // 交换两个值（不用临时变量）
    let mut a = 10;
    let mut b = 20;
    (a, b) = (b, a);
    println!("交换后：a = {}, b = {}", a, b);
}
```

### 单元类型（Unit Type）

```rust
fn main() {
    // 空元组 () 称为单元类型
    let unit: () = ();

    // 没有返回值的函数实际上返回 ()
    fn say_hello() {
        println!("Hello!");
        // 隐式返回 ()
    }

    let result = say_hello();
    println!("返回值：{:?}", result);  // ()

    // 显式返回 ()
    fn explicit_unit() -> () {
        println!("显式返回 ()");
    }

    explicit_unit();
}
```

---

---

## 下一步

[数组（Array）](../7-数组（Array）.md)