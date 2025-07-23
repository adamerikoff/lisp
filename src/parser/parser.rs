use std::io;

use crate::{ast::Expression, tokenizer::Token};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Token, String),
    UnmatchedParenthesis,
    EndOfInput,
}

impl From<ParserError> for io::Error {
    fn from(error: ParserError) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Parser error: {:?}", error))
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current_token_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut program = Vec::new();

        while self.current_token_index < self.tokens.len() {
            if self.check(&Token::Eof) {
                break;
            }

            program.push(self.parse_expression()?);
        }
        Ok(program)
    }

    fn current_token(&self) -> Result<&Token, ParserError> {
        self.tokens.get(self.current_token_index)
            .ok_or(ParserError::EndOfInput)
    }

    fn advance(&mut self) -> Result<(), ParserError> {
        if self.current_token_index < self.tokens.len() {
            self.current_token_index += 1;
            Ok(())
        } else {
            Err(ParserError::EndOfInput)
        }
    }

    fn consume(&mut self, expected_token_type: Token) -> Result<(), ParserError> {
        let current = self.current_token()?.clone();

        let match_found = match (&current, &expected_token_type) {
            (Token::LeftParen, Token::LeftParen) => true,
            (Token::RightParen, Token::RightParen) => true,
            
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::Eof, Token::Eof) => true,

            _ => false,
        };

        if match_found {
            self.advance()?;
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(current, format!("{:?}", expected_token_type)))
        }
    }

    fn check(&self, expected_token_type: &Token) -> bool {
        let current_result = self.current_token();
        if let Ok(current) = current_result {
            match (current, expected_token_type) {
                (Token::LeftParen, Token::LeftParen) => true,
                (Token::RightParen, Token::RightParen) => true,
                (Token::Identifier(_), Token::Identifier(_)) => true,
                (Token::String(_), Token::String(_)) => true,
                (Token::Number(_), Token::Number(_)) => true,
                (Token::Eof, Token::Eof) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let current_token_peek = self.current_token()?;

        match current_token_peek {
            Token::Number(n) => {
                let val = *n;
                self.advance()?;
                Ok(Expression::Number(val))
            },
            Token::String(s) => {
                let val = s.clone();
                self.advance()?;
                Ok(Expression::String(val))
            },
            Token::Identifier(id) => {
                let val = id.clone();
                self.advance()?;
                if val == "true" {
                    Ok(Expression::Boolean(true))
                } else if val == "false" {
                    Ok(Expression::Boolean(false))
                } else {
                    Ok(Expression::Identifier(val))
                }
            },
            Token::LeftParen => self.parse_list_expression(),

            Token::RightParen => Err(ParserError::UnmatchedParenthesis),
            Token::Eof => Err(ParserError::EndOfInput),
        }
    }

    fn parse_list_expression(&mut self) -> Result<Expression, ParserError> {
        self.consume(Token::LeftParen)?;

        let mut elements = Vec::new();
        loop {
            if self.check(&Token::RightParen) {
                break;
            }
            if self.check(&Token::Eof) {
                return Err(ParserError::UnmatchedParenthesis);
            }
            elements.push(self.parse_expression()?);
        }

        self.consume(Token::RightParen)?;
        Ok(Expression::List(elements))
    }
}