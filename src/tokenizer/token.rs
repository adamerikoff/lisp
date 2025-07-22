#[derive(Debug, PartialEq, Clone)] // These traits are useful for debugging, comparison, and copying tokens
pub enum Token {
    LeftParen,    // (
    RightParen,   // )
    Minus,        // -
    Plus,         // +
    Slash,        // /
    Star,         // *

    NotEqual,   // !=
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    Identifier(String), // e.g., variable names, function names
    String(String),     // e.g., "hello world"
    Number(f64),        // e.g., 123, 3.14

    If,
    Let,
    Lambda,
    True,
    False,
    Eof
}