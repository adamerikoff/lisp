#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,    // '('
    RightParen,   // ')'

    Identifier(String), // This will capture "+", "if", "true", "false", "my-var", "=="
    String(String),     // "hello"
    Number(f64),        // 123.45

    Eof // End of input
}