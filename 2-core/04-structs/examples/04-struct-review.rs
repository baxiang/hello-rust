//! # 示例：结构体实战总结
//!
//! 对应章节：04-实战总结
//! 运行：cargo run --example 04-struct-review

#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            grades: Vec::new(),
        }
    }

    fn add_grade(&mut self, grade: f64) {
        self.grades.push(grade);
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.grades.iter().sum();
        sum / self.grades.len() as f64
    }

    fn highest(&self) -> Option<f64> {
        self.grades.iter().cloned().fold(None, |max, x| {
            Some(match max {
                None => x,
                Some(current_max) => {
                    if x > current_max {
                        x
                    } else {
                        current_max
                    }
                }
            })
        })
    }
}

fn main() {
    println!("=== 结构体实战总结 ===\n");

    let mut student = Student::new("张三");
    student.add_grade(95.0);
    student.add_grade(87.5);
    student.add_grade(92.0);
    student.add_grade(88.0);

    println!("学生：{}", student.name);
    println!("成绩：{:?}", student.grades);
    println!("平均分：{:.2}", student.average());
    println!("最高分：{:?}", student.highest());

    // ✅ 结构体更新
    let student2 = Student {
        name: String::from("李四"),
        ..Student::new("李四")
    };
    println!("\n学生 2：{}", student2.name);

    // 尝试修改：
    // 1. 添加 lowest 方法
    // 2. 添加 passed 方法判断是否及格
    // 3. 思考：结构体适合建模什么类型的数据？

    println!("\n结构体章节完成！");
}
