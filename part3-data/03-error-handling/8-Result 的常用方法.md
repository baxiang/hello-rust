## 14.8 Result 的常用方法

```rust
fn main() {
    // ========== 基本查询 ==========
    let ok_result: Result<i32, &str> = Ok(5);
    let err_result: Result<i32, &str> = Err("错误");

    // is_ok / is_err
    println!("{}", ok_result.is_ok());    // true
    println!("{}", err_result.is_err());  // true

    // ok / err - 转换为 Option
    println!("{:?}", ok_result.ok());    // Some(5)
    println!("{:?}", err_result.ok());   // None
    println!("{:?}", ok_result.err());   // None
    println!("{:?}", err_result.err());  // Some("错误")

    // ========== 获取值 ==========
    // unwrap - 获取值，错误则 panic
    println!("{}", ok_result.unwrap());  // 5
    // println!("{}", err_result.unwrap());  // panic!

    // unwrap_or - 提供默认值
    println!("{}", ok_result.unwrap_or(0));   // 5
    println!("{}", err_result.unwrap_or(0));  // 0

    // unwrap_or_else - 使用闭包
    println!("{}", ok_result.unwrap_or_else(|_| 42));
    println!("{}", err_result.unwrap_or_else(|e| {
        println!("错误：{}", e);
        42
    }));

    // unwrap_or_default - 使用默认值
    println!("{:?}", err_result.unwrap_or_default());  // 0

    // expect / expect_err - 自定义 panic 消息
    // err_result.expect("自定义错误消息");

    // ========== 转换值 ==========
    // map - 转换 Ok 值
    let squared = ok_result.map(|x| x * x);
    println!("{:?}", squared);  // Ok(25)

    // map_or / map_or_else - 映射或默认值
    println!("{}", ok_result.map_or(0, |x| x * 2));   // 10
    println!("{}", err_result.map_or(0, |x| x * 2));  // 0

    // map_err - 转换错误
    let mapped = err_result.map_err(|e| format!("错误：{}", e));
    println!("{:?}", mapped);  // Err("错误：错误")

    // ========== 链式操作 ==========
    // and - 两个都 Ok 才返回第二个
    let r1: Result<i32, &str> = Ok(5);
    let r2: Result<i32, &str> = Ok(10);
    println!("{:?}", r1.and(r2));  // Ok(10)

    // and_then - 链式绑定
    let result: Result<i32, &str> = Ok(5);
    let chained = result.and_then(|x| Ok(x * 2));
    println!("{:?}", chained);  // Ok(10)

    // or - 第一个 Err 才返回第二个
    let r1: Result<i32, &str> = Err("错误 1");
    let r2: Result<i32, &str> = Ok(10);
    println!("{:?}", r1.or(r2));  // Ok(10)

    // or_else - 错误时调用闭包
    let result: Result<i32, &str> = Err("错误");
    let fallback = result.or_else(|e| Ok(0));
    println!("{:?}", fallback);  // Ok(0)

    // ========== 迭代器方法 ==========
    let results = vec![Ok(1), Err("错"), Ok(3), Err("误")];

    // collect - 收集 Result
    let collected: Result<Vec<_>, _> = results.iter().cloned().collect();
    println!("{:?}", collected);  // Err("错")

    // filter_map
    let values: Vec<i32> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    println!("{:?}", values);  // [1, 3]（只保留 Ok）
}
```

---

---

## 下一步

[错误处理最佳实践](../9-错误处理最佳实践.md)