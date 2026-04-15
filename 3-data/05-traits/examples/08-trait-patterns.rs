//! # 示例：Trait 设计模式
//!
//! 对应章节：08-Trait设计模式
//! 运行：cargo run --example 08-trait-patterns

// ✅ 正确示例：Builder 模式
struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl Database {
    fn builder() -> DatabaseBuilder {
        DatabaseBuilder::default()
    }
}

#[derive(Default)]
struct DatabaseBuilder {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl DatabaseBuilder {
    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn credentials(mut self, username: &str, password: &str) -> Self {
        self.username = username.to_string();
        self.password = password.to_string();
        self
    }

    fn database(mut self, db: &str) -> Self {
        self.database = db.to_string();
        self
    }

    fn build(self) -> Database {
        Database {
            host: self.host,
            port: self.port,
            username: self.username,
            password: self.password,
            database: self.database,
        }
    }
}

fn main() {
    let db = Database::builder()
        .host("localhost")
        .port(5432)
        .credentials("admin", "secret")
        .database("myapp")
        .build();

    println!("数据库配置：");
    println!("  主机：{}", db.host);
    println!("  端口：{}", db.port);
    println!("  用户：{}", db.username);
    println!("  数据库：{}", db.database);

    // 尝试修改：
    // 1. 添加更多 Builder 方法
    // 2. 实现自己的设计模式
}
