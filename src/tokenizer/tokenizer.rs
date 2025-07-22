use crate::tokenizer::token::Token;

#[derive(Debug)]
pub struct Tokenizer {
    current_position: usize,
    current_char: Option<char>,
    source: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    /// Encountered an unexpected character at a given position.
    UnexpectedCharacter(char, usize),
    /// A string literal was started but not terminated before EOF.
    UnterminatedString(usize),
    /// A number literal was malformed (e.g., "123.").
    MalformedNumber(usize), // Added for more specific number errors
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let source_chars: Vec<char> = input.chars().collect();
        let initial_char = source_chars.first().copied();

        Tokenizer {
            current_position: 0,
            current_char: initial_char,
            source: source_chars,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let consumed_char = self.current_char;
        self.current_position += 1;
        self.current_char = self.source.get(self.current_position).copied();

        consumed_char
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.current_position + 1).copied() // Use .get() for bounds checking
    }

    pub fn next_token(&mut self) -> Result<Token, TokenizerError> {
        self.skip_whitespace();

        let Some(current_char) = self.current_char else {
            return Ok(Token::Eof);
        };
        let start_pos = self.current_position;

        let token = match current_char {
            // Single-character tokens: Consume and return.
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '*' => Token::Star,
            '/' => Token::Slash,

            // One or two character tokens: Check `peek()` before consuming.
            '!' => {
                if self.peek() == Some('=') {
                    self.advance(); 
                    Token::NotEqual
                } else {
                    return Err(TokenizerError::UnexpectedCharacter(current_char, start_pos));
                }
            }
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    return Err(TokenizerError::UnexpectedCharacter(current_char, start_pos));
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }

            '"' => self.read_string()?,
            '0'..='9' => self.read_number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier_or_keyword(),

            _ => return Err(TokenizerError::UnexpectedCharacter(current_char, start_pos)),
        };

        // For single-character tokens and the first character of multi-char tokens
        // (excluding those handled by `read_...` methods), advance the tokenizer.
        // `read_string`, `read_number`, `read_identifier_or_keyword` handle their own advancement.
        if !matches!(token, Token::String(_) | Token::Number(_) | Token::Identifier(_)) {
            self.advance();
        }

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, TokenizerError> {
        let start_char_pos = self.current_position; 
        self.advance();

        let string_content_start = self.current_position;

        while let Some(c) = self.current_char {
            if c == '"' {
                break; 
            }
            self.advance();
        }

        if self.current_char != Some('"') {
            return Err(TokenizerError::UnterminatedString(start_char_pos)); 
        }

        let string_value: String = self.source[string_content_start..self.current_position].iter().collect();
        self.advance();
        Ok(Token::String(string_value))
    }

    fn read_number(&mut self) -> Result<Token, TokenizerError> {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if self.current_char == Some('.') && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
            while let Some(c) = self.current_char {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_str: String = self.source[start_pos..self.current_position].iter().collect();
        let value = num_str.parse::<f64>()
            .map_err(|_| TokenizerError::MalformedNumber(start_pos))?;

        Ok(Token::Number(value))
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let identifier_str: String = self.source[start_pos..self.current_position].iter().collect();

        match identifier_str.as_str() {
            "lambda" => Token::Lambda,
            "if" => Token::If,
            "false" => Token::False,
            "true" => Token::True,
            "let" => Token::Let,
            _ => Token::Identifier(identifier_str),
        }
    }
}