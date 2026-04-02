## 19.2 Iterator Trait

### Trait 定义

```rust
// 标准库中的定义
pub trait Iterator {
    type Item;  // 关联类型：迭代的元素类型

    // 唯一必需的方法
    fn next(&mut self) -> Option<Self::Item>;

    // 大量默认实现的方法（适配器）
    fn map<F, B>(self, f: F) -> Map<Self, F> { ... }
    fn filter<P>(self, predicate: P) -> Filter<Self, P> { ... }
    fn collect<B>(self) -> B { ... }
    // ... 等等
}
```

### next() 方法详解

```
next() 的工作流程：

┌─────────────────────────────────────────────────────┐
│                 迭代器状态机                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  每次调用 next()：                                   │
│                                                     │
│  有下一个元素  ──────→  Some(element)               │
│                      │                              │
│  没有元素了   ──────→  None（迭代结束）              │
│                                                     │
│  示意图：                                            │
│  ┌─────┬─────┬─────┬─────┬─────┐                   │
│  │  A  │  B  │  C  │  D  │     │                   │
│  └─────┴─────┴─────┴─────┴─────┘                   │
│     ↑                                               │
│     当前位置                                        │
│                                                     │
│  next() → Some(A)，位置前进                         │
│  next() → Some(B)，位置前进                         │
│  next() → Some(C)，位置前进                         │
│  next() → Some(D)，位置前进                         │
│  next() → None，结束                                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 手动迭代示例

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    // 手动调用 next()
    println!("{:?}", iter.next());  // Some(&1)
    println!("{:?}", iter.next());  // Some(&2)
    println!("{:?}", iter.next());  // Some(&3)
    println!("{:?}", iter.next());  // None
    println!("{:?}", iter.next());  // None（已经结束）
}
```

---

---

## 下一步

[创建迭代器](../3-创建迭代器.md)