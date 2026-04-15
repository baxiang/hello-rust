//! # 示例：测试基础
//!
//! 对应章节：01-测试基础
//! 运行：cargo run --example 01-test-basics

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert!(divide(10, 0).is_err());
    }

    #[test]
    #[should_panic(expected = "assertion")]
    fn test_panic() {
        assert!(false);
    }
}

fn main() {
    println!("=== 测试基础 ===\n");
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 2) = {:?}", divide(10, 2));
    println!("\n运行测试：cargo test");
}
