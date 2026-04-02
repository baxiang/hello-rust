## 12.4 Vec 常用方法大全

```rust
fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // ========== 查询方法 ==========
    println!("长度：{}", v.len());           // 8
    println!("容量：{}", v.capacity());      // >= 8
    println!("是否为空：{}", v.is_empty());  // false
    println!("包含 5: {}", v.contains(&5));  // true
    println!("第一个：{:?}", v.first());     // Some(3)
    println!("最后一个：{:?}", v.last());    // Some(6)

    // ========== 修改方法 ==========
    v.push(7);              // 末尾添加
    v.pop();                // 移除末尾
    v.insert(0, 0);         // 索引 0 插入 0
    v.remove(0);            // 移除索引 0
    v[0] = 10;              // 修改索引 0

    // ========== 批量操作 ==========
    v.extend([11, 12, 13]);           // 批量添加
    v.truncate(5);                    // 截断
    v.clear();                        // 清空

    // ========== 排序 ==========
    v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort();                         // 原地排序
    println!("排序后：{:?}", v);       // [1, 1, 2, 3, 4, 5, 6, 9]

    v.sort_by(|a, b| b.cmp(a));       // 降序排序
    println!("降序：{:?}", v);         // [9, 6, 5, 4, 3, 2, 1, 1]

    v.reverse();                      // 反转
    println!("反转：{:?}", v);         // [1, 1, 2, 3, 4, 5, 6, 9]

    // ========== 查找 ==========
    println!("3 的索引：{:?}", v.iter().position(|&x| x == 3));  // Some(3)
    println!("是否有偶数：{}", v.iter().any(|&x| x % 2 == 0));   // true
    println!("是否全为正：{}", v.iter().all(|&x| x > 0));        // true

    // ========== 去重 ==========
    v.sort();
    v.dedup();  // 移除连续重复元素
    println!("去重后：{:?}", v);  // [1, 2, 3, 4, 5, 6, 9]

    // ========== 迭代 ==========
    for (i, val) in v.iter().enumerate() {
        println!("v[{}] = {}", i, val);
    }
}
```




