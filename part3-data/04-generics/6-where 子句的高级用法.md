## 15.6 where 子句的高级用法

### 约束关联类型

```rust
use std::fmt::Display;

// 约束 Iterator 的 Item 类型
fn print_all<T>(iter: T)
where
    T: IntoIterator,
    T::Item: Display,
{
    for item in iter {
        println!("{}", item);
    }
}

fn main() {
    print_all(vec![1, 2, 3]);
    print_all(vec!["a", "b", "c"]);
}
```

### 约束生命周期

```rust
// 约束引用生命周期
fn get_longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: PartialOrd,
{
    if x > y { x } else { y }
}

fn main() {
    let a = 5;
    let b = 10;
    println!("{}", get_longest(&a, &b));
}
```

### 复杂约束组合

```rust
use std::fmt::Display;
use std::hash::Hash;
use std::collections::HashMap;

// 多约束组合
fn count_occurrences<T>(items: &[T]) -> HashMap<&T, usize>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

// 约束借用
fn process<'a, T>(items: &'a [T]) -> Vec<&'a T>
where
    T: Clone,
{
    items.iter().collect()
}

fn main() {
    let items = vec![1, 2, 2, 3, 3, 3];
    let counts = count_occurrences(&items);
    println!("{:?}", counts);
}
```

---

---

## 下一步

[泛型与性能：单态化](../7-泛型与性能：单态化.md)