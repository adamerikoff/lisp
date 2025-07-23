use std::io;

use crate::tokenizer::token::Token; 

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    UnexpectedCharacter(char, usize),
    UnterminatedString(usize),
    MalformedNumber(usize),
}

impl From<TokenizerError> for io::Error {
    fn from(error: TokenizerError) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Tokenizer error: {:?}", error))
    }
}


#[derive(Debug)]
pub struct Tokenizer {
    current_position: usize,
    current_char: Option<char>,
    source: Vec<char>,
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

    fn peek(&self) -> Option<char> {
        self.source.get(self.current_position + 1).copied()
    }

    fn next_token(&mut self) -> Result<Token, TokenizerError> {
        self.skip_whitespace();

        let Some(current_char) = self.current_char else {
            return Ok(Token::Eof);
        };
        let start_pos = self.current_position;

        let token = match current_char {
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }

            '"' => self.read_string()?,

            '0'..='9' => self.read_number()?,

            c if !c.is_whitespace() => self.read_identifier()?,

            _ => return Err(TokenizerError::UnexpectedCharacter(current_char, start_pos)),
        };

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
        self.advance(); // Consume the opening '"'

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

        let string_value: String = self.source[string_content_start..self.current_position]
            .iter()
            .collect();
        self.advance();
        Ok(Token::String(string_value))
    }

    fn read_number(&mut self) -> Result<Token, TokenizerError> {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        if self.current_char == Some('.') && self.peek().map_or(false, |c| c.is_digit(10)) {
            self.advance();
            while let Some(c) = self.current_char {
                if c.is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_str: String = self.source[start_pos..self.current_position]
            .iter()
            .collect();
        let value = num_str
            .parse::<f64>()
            .map_err(|_| TokenizerError::MalformedNumber(start_pos))?;

        Ok(Token::Number(value))
    }

    fn read_identifier(&mut self) -> Result<Token, TokenizerError> {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if !c.is_whitespace() && c != '(' && c != ')' && c != '"' {
                self.advance();
            } else {
                break;
            }
        }

        let identifier_str: String = self.source[start_pos..self.current_position]
            .iter()
            .collect();

        if identifier_str.is_empty() {
            Err(TokenizerError::UnexpectedCharacter(
                self.current_char.unwrap_or('\0'),
                start_pos,
            ))
        } else {
            Ok(Token::Identifier(identifier_str))
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    tokens.push(token.clone());
                    if token == Token::Eof {
                        break;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(tokens)
    }
}
