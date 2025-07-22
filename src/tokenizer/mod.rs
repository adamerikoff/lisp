// src/tokenizer/mod.rs

pub mod token;      // Declares the 'token' module (looks for src/tokenizer/token.rs)
pub mod tokenizer;  // Declares the 'tokenizer' module (looks for src/tokenizer/tokenizer.rs)

// Re-export key items for easier access
pub use self::token::Token;
pub use self::tokenizer::{Tokenizer, TokenizerError};