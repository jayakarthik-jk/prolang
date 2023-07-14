use std::cell::RefCell;

use crate::datatypes::DataType;
use crate::errors::CompilerError;
use crate::keywords::Keyword;
use crate::operators::arithmetic::Arithmetic::*;
use crate::operators::{Operator, Operator::*};
use crate::symbols::Symbol::{self, *};
pub struct Lexer {
    source: Vec<u8>,
    tokens: Vec<Token>,
    index: RefCell<usize>,
    position: usize,
    current: char,
}
impl Lexer {
    pub fn new(source: String) -> Self {
        let source: Vec<u8> = source.bytes().collect();
        let current = if source.is_empty() {
            '\0'
        } else {
            source[0] as char
        };

        Self {
            source,
            tokens: vec![],
            index: RefCell::new(0),
            position: 0,
            current,
        }
    }

    fn _next(&mut self) -> char {
        self.current = if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.position += 1;
            self.source[self.position] as char
        };
        self.current
    }

    pub fn parse_token(&mut self) -> Result<&Token, CompilerError> {
        if self.current.is_ascii_whitespace() {
            // let start = self.position;
            while self.current.is_ascii_whitespace() {
                self._next();
            }
            // let count = self.position - start;
            // let token = Token::new(TokenKind::WhitespaceToken(count), start);
            // return Ok(token);
        }

        let current_token = if self.current == '\r' && self._next() == '\n' {
            let token = Token::new(TokenKind::NewLineToken, self.position);
            self._next();
            token
        } else {
            match self.current {
                '\0' => return Err(CompilerError::EndOfSourceCodeError),
                current if current.is_ascii_digit() => {
                    let start = self.position;
                    while self.current.is_ascii_digit() {
                        self._next();
                    }
                    if self.current == '.' {
                        self._next();
                        while self.current.is_ascii_digit() {
                            self._next();
                        }
                        let number = &self.source[start..self.position];
                        let number = String::from_utf8_lossy(number);
                        let number: f64 = match number.parse() {
                            Ok(number) => number,
                            Err(_) => {
                                return Err(CompilerError::CustomError(format!(
                                    "{number} is not a valid float64"
                                )));
                            }
                        };
                        Token::new(TokenKind::LiteralToken(DataType::Float(number)), start)
                    } else {
                        let number = &self.source[start..self.position];
                        let number = String::from_utf8_lossy(number);
                        let number: i64 = match number.parse() {
                            Ok(number) => number,
                            Err(_) => {
                                return Err(CompilerError::CustomError(format!(
                                    "{number} is not a valid int64"
                                )));
                            }
                        };
                        Token::new(TokenKind::LiteralToken(DataType::Number(number)), start)
                    }
                }
                current if current.is_alphabetic() => {
                    let start = self.position;
                    while self.current.is_ascii_alphanumeric() {
                        self._next();
                    }
                    let word = &self.source[start..self.position];
                    let word = String::from_utf8_lossy(word);
                    Keyword::parse(word.to_string(), start)?
                }
                current if current == '\'' => {
                    self._next();
                    let start = self.position;
                    while self.current != '\'' && self._next() != '\0' {}
                    let (string, start) = if self.current == '\'' {
                        let string = &self.source[start..self.position];
                        let string = String::from_utf8_lossy(string);
                        (string, start)
                    } else {
                        return Err(CompilerError::EndOfSourceCodeError);
                    };
                    let token = Token::new(
                        TokenKind::LiteralToken(DataType::String(string.to_string())),
                        start,
                    );
                    self._next();
                    token
                }
                current if current == '\"' => {
                    self._next();
                    let start = self.position;
                    while self.current != '\"' && self._next() != '\0' {}
                    let (string, start) = if self.current == '\"' {
                        let string = &self.source[start..self.position];
                        let string = String::from_utf8_lossy(string);
                        (string, start)
                    } else {
                        return Err(CompilerError::EndOfSourceCodeError);
                    };
                    let token = Token::new(
                        TokenKind::LiteralToken(DataType::String(string.to_string())),
                        start,
                    );
                    self._next();
                    token
                }
                '+' => {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Addition)),
                        self.position,
                    );
                    self._next();
                    token
                }
                '-' => {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Subtraction)),
                        self.position,
                    );
                    self._next();
                    token
                }
                '*' => {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Multiplication)),
                        self.position,
                    );
                    self._next();
                    token
                }
                '/' => {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Division)),
                        self.position,
                    );
                    self._next();
                    token
                }
                '%' => {
                    let token = Token::new(
                        TokenKind::OperatorToken(ArithmeticOperator(Modulo)),
                        self.position,
                    );
                    self._next();
                    token
                }
                '(' => {
                    let token = Token::new(TokenKind::SymbolToken(OpenParanthesis), self.position);
                    self._next();
                    token
                }
                ')' => {
                    let token = Token::new(TokenKind::SymbolToken(CloseParanthesis), self.position);
                    self._next();
                    token
                }
                _ => return Err(CompilerError::UnknownTokenError(self.current)),
            }
        };

        self.tokens.push(current_token);
        return Ok(self.tokens.last().unwrap());
    }

    pub fn prepare(&mut self) -> Result<(), CompilerError> {
        loop {
            match self.parse_token() {
                Ok(_) => {}
                Err(error) if CompilerError::EndOfSourceCodeError == error => {
                    return Ok(());
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
    }

    /// returns tokens\[pointer\]
    pub fn advance(&self) {
        let mut index = self.index.borrow_mut();
        *index += 1;
    }

    /// returns --pointer
    pub fn rewind(&self) {
        let mut index = self.index.borrow_mut();
        if *index > 0 {
            *index -= 1;
        }
    }

    /// returns tokens\[pointer\]
    pub fn get_current_token(&self) -> Result<&Token, CompilerError> {
        let index = *self.index.borrow();
        match self.tokens.get(index) {
            Some(token) => Ok(token),
            None => {
                if self.tokens.is_empty() {
                    Err(CompilerError::NoTokensAvailable)
                } else {
                    Err(CompilerError::EndOfSourceCodeError)
                }
            }
        }
    }

    /// returns tokens\[++pointer\]
    pub fn advance_and_get_current_token(&self) -> Result<&Token, CompilerError> {
        self.advance();
        self.get_current_token()
    }

    /// returns tokens\[--pointer\]
    pub fn rewind_and_get_current_token(&self) -> Result<&Token, CompilerError> {
        self.rewind();
        self.get_current_token()
    }

    /// returns tokens\[pointer++\]
    pub fn get_current_token_and_advance(&self) -> Result<&Token, CompilerError> {
        let current = self.get_current_token();
        self.advance();
        current
    }

    /// returns tokens\[pointer--\]
    pub fn get_current_token_and_rewind(&self) -> Result<&Token, CompilerError> {
        let current = self.get_current_token();
        self.rewind();
        current
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    position: usize,
}

impl Token {
    pub fn new(kind: TokenKind, position: usize) -> Self {
        Self { kind, position }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LiteralToken(DataType),
    /// number of whitespace
    WhitespaceToken(usize),
    NewLineToken,
    OperatorToken(Operator),
    SymbolToken(Symbol),
}
