## 11.5 match 控制流

### 基本用法

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));   // 1
    println!("{}", value_in_cents(Coin::Quarter)); // 25
}
```

### match 是表达式

```rust
fn main() {
    let number = 5;

    // match 可以返回值
    let description = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "其他",
    };

    println!("{}", description);
}
```

### 绑定值

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运的一分钱！");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("州纪念币：{:?}", state);
            25
        }
    }
}
```




