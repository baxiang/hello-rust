## 路径：绝对路径 vs 相对路径

### 绝对路径

```rust
// 从 crate 根开始
crate::front_of_house::hosting::add_to_waitlist();

// 或者省略 crate（推荐）
front_of_house::hosting::add_to_waitlist();
```

### 相对路径

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve() {
    // 相对路径：从当前模块开始
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    fn fix_order() {
        // super 指向父模块
        super::front_of_house::hosting::add_to_waitlist();
    }
}
```

### 路径选择建议

```
┌─────────────────────────────────────────────────────┐
│              路径选择指南                            │
├─────────────────────────────────────────────────────┤
│                                                     │
│  使用绝对路径（crate::...）当：                      │
│  • 引用顶层公共 API                                  │
│  • 希望路径清晰明确                                  │
│  • 模块层级较深                                      │
│                                                     │
│  使用相对路径（super::...）当：                      │
│  • 引用兄弟模块或父模块                              │
│  • 模块层级较浅                                      │
│  • 路径很短（如 self::func）                         │
│                                                     │
│  实际项目中，推荐使用绝对路径                        │
│                                                     │
└─────────────────────────────────────────────────────┘
```




