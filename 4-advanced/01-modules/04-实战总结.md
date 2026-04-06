## 完整示例：餐厅系统

### 项目结构

```bash
src/
├── lib.rs
├── front_of_house/
│   ├── mod.rs
│   ├── hosting.rs
│   └── serving.rs
├── back_of_house/
│   ├── mod.rs
│   └── cooking.rs
└── main.rs
```

### 代码实现

```rust
// src/lib.rs
pub mod front_of_house;
pub mod back_of_house;

pub fn serve_dinner() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    back_of_house::cooking::cook_dish();
    front_of_house::serving::serve_order();
}
```

```rust
// src/front_of_house/mod.rs
pub mod hosting;
pub mod serving;
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {
    println!("Added to waitlist");
}

pub fn seat_at_table(table_id: usize) -> usize {
    println!("Seating at table {}", table_id);
    table_id
}
```

```rust
// src/front_of_house/serving.rs
pub fn take_order() {
    println!("Taking order");
}

pub fn serve_order() {
    println!("Serving order");
}
```

```rust
// src/back_of_house/mod.rs
pub mod cooking;
```

```rust
// src/back_of_house/cooking.rs
pub fn cook_dish() {
    println!("Cooking dish");
}

pub struct Chef {
    pub name: String,
}

impl Chef {
    pub fn new(name: String) -> Self {
        Chef { name }
    }

    pub fn cook(&self) {
        println!("{} is cooking", self.name);
    }
}
```

```rust
// src/main.rs
use restaurant::front_of_house::hosting;
use restaurant::back_of_house::Chef;

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table(5);

    let chef = Chef::new(String::from("Gordon"));
    chef.cook();

    restaurant::serve_dinner();
}
```




