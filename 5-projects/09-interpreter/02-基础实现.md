## 第一步：创建项目

```bash
cargo new mini-interpreter
cd mini-interpreter
```

### Cargo.toml

```toml
[package]
name = "mini-interpreter"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
thiserror = "1"
rustyline = "13"  # REPL 支持

[dev-dependencies]
pretty_assertions = "1"
```






## 第二步：Token 定义

### src/lexer/token.rs

```rust
use std::fmt;

/// Token 类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // 字面量
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // 关键字
    Let,
    Fn,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    
    // 运算符
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Equal,      // =
    EqualEqual, // ==
    BangEqual,  // !=
    Less,       // <
    LessEqual,  // <=
    Greater,    // >
    GreaterEqual, // >=
    And,        // &&
    Or,         // ||
    Bang,       // !
    
    // 分隔符
    LeftParen,   // (
    RightParen,  // )
    LeftBrace,   // {
    RightBrace,  // }
    LeftBracket, // [
    RightBracket, // ]
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    
    // 特殊
    Eof,
}

/// Token
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self { token_type, line, column }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Number(n) => write!(f, "NUMBER({})", n),
            TokenType::String(s) => write!(f, "STRING(\"{}\")", s),
            TokenType::Boolean(b) => write!(f, "BOOLEAN({})", b),
            TokenType::Identifier(s) => write!(f, "IDENT({})", s),
            TokenType::Let => write!(f, "LET"),
            TokenType::Fn => write!(f, "FN"),
            TokenType::If => write!(f, "IF"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Null => write!(f, "NULL"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::And => write!(f, "&&"),
            TokenType::Or => write!(f, "||"),
            TokenType::Bang => write!(f, "!"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
```






## 第三步：词法扫描

### src/lexer/scanner.rs

```rust
use crate::lexer::token::{Token, TokenType};
use crate::error::Error;

/// 词法扫描器
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    /// 创建扫描器
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// 扫描所有 Token
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token::new(TokenType::Eof, self.line, self.column));
        Ok(tokens)
    }
    
    /// 扫描单个 Token
    fn scan_token(&mut self) -> Result<Token, Error> {
        let c = self.advance();
        
        match c {
            // 空白字符
            ' ' | '\r' | '\t' => Ok(self.make_token(TokenType::Eof)), // 跳过
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(self.make_token(TokenType::Eof)) // 跳过
            }
            
            // 单字符 Token
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            '[' => Ok(self.make_token(TokenType::LeftBracket)),
            ']' => Ok(self.make_token(TokenType::RightBracket)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            ':' => Ok(self.make_token(TokenType::Colon)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            '*' => Ok(self.make_token(TokenType::Star)),
            '/' => Ok(self.make_token(TokenType::Slash)),
            '%' => Ok(self.make_token(TokenType::Percent)),
            
            // 可能是多字符的 Token
            '-' => Ok(self.make_token(TokenType::Minus)),
            '=' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::EqualEqual))
                } else {
                    Ok(self.make_token(TokenType::Equal))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::BangEqual))
                } else {
                    Ok(self.make_token(TokenType::Bang))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::LessEqual))
                } else {
                    Ok(self.make_token(TokenType::Less))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenType::GreaterEqual))
                } else {
                    Ok(self.make_token(TokenType::Greater))
                }
            }
            '&' => {
                if self.match_char('&') {
                    Ok(self.make_token(TokenType::And))
                } else {
                    Err(Error::SyntaxError("期望 '&&'".to_string(), self.line))
                }
            }
            '|' => {
                if self.match_char('|') {
                    Ok(self.make_token(TokenType::Or))
                } else {
                    Err(Error::SyntaxError("期望 '||'".to_string(), self.line))
                }
            }
            
            // 字符串
            '"' => self.scan_string(),
            
            // 数字
            '0'..='9' => self.scan_number(),
            
            // 标识符和关键字
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),
            
            _ => Err(Error::SyntaxError(format!("意外字符: {}", c), self.line)),
        }
    }
    
    /// 扫描字符串
    fn scan_string(&mut self) -> Result<Token, Error> {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != '"' {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
            }
            value.push(c);
        }
        
        if self.is_at_end() {
            return Err(Error::SyntaxError("未闭合的字符串".to_string(), self.line));
        }
        
        self.advance(); // 闭合引号
        Ok(self.make_token(TokenType::String(value)))
    }
    
    /// 扫描数字
    fn scan_number(&mut self) -> Result<Token, Error> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        
        // 小数部分
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance(); // 消费 '.'
            
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        
        let value: f64 = self.current_lexeme().parse()
            .map_err(|_| Error::SyntaxError("无效数字".to_string(), self.line))?;
        
        Ok(self.make_token(TokenType::Number(value)))
    }
    
    /// 扫描标识符
    fn scan_identifier(&mut self) -> Result<Token, Error> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        
        let text = self.current_lexeme();
        let token_type = match text.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "true" => TokenType::Boolean(true),
            "false" => TokenType::Boolean(false),
            "null" => TokenType::Null,
            _ => TokenType::Identifier(text.clone()),
        };
        
        Ok(self.make_token(token_type))
    }
    
    // === 辅助方法 ===
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.current] }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { '\0' } else { self.source[self.current + 1] }
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
    
    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }
    
    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn current_lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }
    
    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.line, self.column)
    }
}
```






## 第四步：AST 定义

### src/parser/ast.rs

```rust
use std::fmt;

/// 表达式
#[derive(Debug, Clone)]
pub enum Expr {
    /// 数字字面量
    Number(f64),
    /// 字符串字面量
    String(String),
    /// 布尔字面量
    Boolean(bool),
    /// 空值
    Null,
    /// 标识符
    Identifier(String),
    /// 二元运算
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    /// 一元运算
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    /// 函数调用
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    /// 条件表达式
    Conditional {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
}

/// 二元运算符
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

/// 一元运算符
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// 语句
#[derive(Debug, Clone)]
pub enum Stmt {
    /// 表达式语句
    Expression(Expr),
    /// 变量声明
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    /// 函数声明
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    /// If 语句
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    /// While 语句
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    /// Return 语句
    Return(Option<Expr>),
    /// 块语句
    Block(Vec<Stmt>),
}

/// 程序
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Boolean(b) => write!(f, "{}", b),
            Expr::Null => write!(f, "null"),
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expr::Unary { operator, operand } => {
                write!(f, "({} {})", operator, operand)
            }
            Expr::Call { callee, arguments } => {
                let args: Vec<String> = arguments.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", callee, args.join(", "))
            }
            Expr::Conditional { condition, then_branch, else_branch } => {
                write!(f, "if {} then {}", condition, then_branch)?;
                if let Some(else_expr) = else_branch {
                    write!(f, " else {}", else_expr)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}
```






## 第五步：语法解析

### src/parser/parser.rs

```rust
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::*;
use crate::error::Error;

/// 语法解析器
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// 创建解析器
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    /// 解析程序
    pub fn parse(&mut self) -> Result<Program, Error> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program { statements })
    }
    
    // === 语句解析 ===
    
    fn parse_statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(TokenType::Let) {
            self.parse_var_statement()
        } else if self.match_token(TokenType::Fn) {
            self.parse_function_statement()
        } else if self.match_token(TokenType::If) {
            self.parse_if_statement()
        } else if self.match_token(TokenType::While) {
            self.parse_while_statement()
        } else if self.match_token(TokenType::Return) {
            self.parse_return_statement()
        } else if self.match_token(TokenType::LeftBrace) {
            self.parse_block_statement()
        } else {
            self.parse_expression_statement()
        }
    }
    
    fn parse_var_statement(&mut self) -> Result<Stmt, Error> {
        let name = self.expect_identifier()?;
        
        let initializer = if self.match_token(TokenType::Equal) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.expect_token(TokenType::Semicolon)?;
        
        Ok(Stmt::Var { name, initializer })
    }
    
    fn parse_function_statement(&mut self) -> Result<Stmt, Error> {
        let name = self.expect_identifier()?;
        
        self.expect_token(TokenType::LeftParen)?;
        
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                params.push(self.expect_identifier()?);
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.expect_token(TokenType::RightParen)?;
        self.expect_token(TokenType::LeftBrace)?;
        
        let body = self.parse_block()?;
        
        Ok(Stmt::Function { name, params, body })
    }
    
    fn parse_if_statement(&mut self) -> Result<Stmt, Error> {
        self.expect_token(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(TokenType::RightParen)?;
        
        self.expect_token(TokenType::LeftBrace)?;
        let then_branch = self.parse_block()?;
        
        let else_branch = if self.match_token(TokenType::Else) {
            self.expect_token(TokenType::LeftBrace)?;
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(Stmt::If { condition, then_branch, else_branch })
    }
    
    fn parse_while_statement(&mut self) -> Result<Stmt, Error> {
        self.expect_token(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(TokenType::RightParen)?;
        
        self.expect_token(TokenType::LeftBrace)?;
        let body = self.parse_block()?;
        
        Ok(Stmt::While { condition, body })
    }
    
    fn parse_return_statement(&mut self) -> Result<Stmt, Error> {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.expect_token(TokenType::Semicolon)?;
        
        Ok(Stmt::Return(value))
    }
    
    fn parse_block_statement(&mut self) -> Result<Stmt, Error> {
        let statements = self.parse_block()?;
        Ok(Stmt::Block(statements))
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(statements)
    }
    
    fn parse_expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.parse_expression()?;
        self.expect_token(TokenType::Semicolon)?;
        Ok(Stmt::Expression(expr))
    }
    
    // === 表达式解析 ===
    
    fn parse_expression(&mut self) -> Result<Expr, Error> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_and()?;
        
        while self.match_token(TokenType::Or) {
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_equality()?;
        
        while self.match_token(TokenType::And) {
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_comparison()?;
        
        while let Some(op) = self.match_equality_op() {
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_term()?;
        
        while let Some(op) = self.match_comparison_op() {
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_factor()?;
        
        while let Some(op) = self.match_term_op() {
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_unary()?;
        
        while let Some(op) = self.match_factor_op() {
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(TokenType::Minus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Negate,
                operand: Box::new(operand),
            });
        }
        
        if self.match_token(TokenType::Bang) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Not,
                operand: Box::new(operand),
            });
        }
        
        self.parse_primary()
    }
    
    fn parse_primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(TokenType::Number(_)) {
            if let TokenType::Number(n) = &self.previous().token_type {
                return Ok(Expr::Number(*n));
            }
        }
        
        if self.match_token(TokenType::String(_)) {
            if let TokenType::String(s) = &self.previous().token_type {
                return Ok(Expr::String(s.clone()));
            }
        }
        
        if self.match_token(TokenType::Boolean(_)) {
            if let TokenType::Boolean(b) = &self.previous().token_type {
                return Ok(Expr::Boolean(*b));
            }
        }
        
        if self.match_token(TokenType::Null) {
            return Ok(Expr::Null);
        }
        
        if self.match_token(TokenType::Identifier(_)) {
            if let TokenType::Identifier(name) = &self.previous().token_type {
                let expr = Expr::Identifier(name.clone());
                
                // 函数调用
                if self.match_token(TokenType::LeftParen) {
                    let mut arguments = Vec::new();
                    if !self.check(TokenType::RightParen) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if !self.match_token(TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect_token(TokenType::RightParen)?;
                    
                    return Ok(Expr::Call {
                        callee: Box::new(expr),
                        arguments,
                    });
                }
                
                return Ok(expr);
            }
        }
        
        if self.match_token(TokenType::LeftParen) {
            let expr = self.parse_expression()?;
            self.expect_token(TokenType::RightParen)?;
            return Ok(expr);
        }
        
        Err(Error::SyntaxError(
            format!("期望表达式，找到 {:?}", self.peek().token_type),
            self.peek().line,
        ))
    }
    
    // === 辅助方法 ===
    
    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::EqualEqual) {
            Some(BinaryOp::Equal)
        } else if self.match_token(TokenType::BangEqual) {
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }
    
    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::Less) {
            Some(BinaryOp::Less)
        } else if self.match_token(TokenType::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else if self.match_token(TokenType::Greater) {
            Some(BinaryOp::Greater)
        } else if self.match_token(TokenType::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else {
            None
        }
    }
    
    fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::Plus) {
            Some(BinaryOp::Add)
        } else if self.match_token(TokenType::Minus) {
            Some(BinaryOp::Subtract)
        } else {
            None
        }
    }
    
    fn match_factor_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::Star) {
            Some(BinaryOp::Multiply)
        } else if self.match_token(TokenType::Slash) {
            Some(BinaryOp::Divide)
        } else if self.match_token(TokenType::Percent) {
            Some(BinaryOp::Modulo)
        } else {
            None
        }
    }
    
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&token_type)
        }
    }
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    
    fn expect_token(&mut self, token_type: TokenType) -> Result<(), Error> {
        if self.check(token_type.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(Error::SyntaxError(
                format!("期望 {:?}, 找到 {:?}", token_type, self.peek().token_type),
                self.peek().line,
            ))
        }
    }
    
    fn expect_identifier(&mut self) -> Result<String, Error> {
        if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(Error::SyntaxError(
                format!("期望标识符, 找到 {:?}", self.peek().token_type),
                self.peek().line,
            ))
        }
    }
}
```







