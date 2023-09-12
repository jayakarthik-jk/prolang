use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;

use super::token::{Token, TokenKind};
use crate::common::errors::CompilerError;
use crate::common::literal::Literal;
use crate::lexing::keywords::Keyword;
use crate::lexing::symbols::Symbol::*;

pub(crate) struct Lexer {
    source: Vec<u8>,
    tokens: Vec<Rc<Token>>,
    index: RefCell<usize>,
    position: usize,
    current: char,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.tokens)
    }
}

impl Lexer {
    pub(crate) fn new(source: String) -> Self {
        let source: Vec<u8> = source.bytes().collect();
        let current = if source.is_empty() {
            '\0'
        } else {
            source[0] as char
        };

        Self {
            source,
            tokens: Vec::new(),
            index: RefCell::new(0),
            position: 0,
            current,
            line: 1,
            column: 1,
        }
    }

    pub(crate) fn _next(&mut self) -> char {
        self.current = if self.position + 1 >= self.source.len() {
            // self.position = self.source.len();
            '\0'
        } else {
            self.column += 1;
            self.position += 1;
            self.source[self.position] as char
        };
        self.current
    }

    fn _peek(&self, offset: usize) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + offset] as char
        }
    }

    pub(crate) fn parse_token(&mut self) -> Result<Token, CompilerError> {
        let current_token = match self.current {
            '\0' => Token::new(TokenKind::EndOfFile, self.line, self.column),
            '\r' => {
                self._next();
                if self.current == '\n' {
                    let token = Token::new(TokenKind::NewLine, self.line, self.column);
                    self._next();
                    self.line += 1;
                    self.column = 1;
                    token
                } else {
                    let token = Token::new(TokenKind::NewLine, self.line, self.column);
                    self.line += 1;
                    self.column = 1;
                    token
                }
            }
            '\n' => {
                let token = Token::new(TokenKind::NewLine, self.line, self.column);
                self._next();
                self.line += 1;
                self.column = 1;
                token
            }
            current if current.is_ascii_whitespace() => {
                let start = self.position;
                while self.current.is_ascii_whitespace() {
                    self._next();
                }
                let count = self.position - start;
                let token = Token::new(TokenKind::Whitespace(count), self.line, self.column);
                return Ok(token);
            }
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
                    let number_as_string = self.extract_string_from_start_to_current(start)?;
                    let number: f64 = match number_as_string.parse() {
                        Ok(number) => number,
                        Err(_) => {
                            let len = number_as_string.len();
                            return Err(CompilerError::InvalidNumber(
                                number_as_string,
                                self.line,
                                self.column - len,
                            ));
                        }
                    };
                    Token::new(
                        TokenKind::Literal(Literal::from(number)),
                        self.line,
                        self.column - number_as_string.len(),
                    )
                } else {
                    let number_as_string = self.extract_string_from_start_to_current(start)?;

                    let number: i128 = match number_as_string.parse() {
                        Ok(number) => number,
                        Err(_) => {
                            return Err(CompilerError::InvalidNumber(
                                number_as_string.to_string(),
                                self.line,
                                self.column - number_as_string.len(),
                            ));
                        }
                    };
                    Token::new(
                        TokenKind::Literal(Literal::from(number)),
                        self.line,
                        self.column - number_as_string.len(),
                    )
                }
            }
            current if current.is_alphabetic() || current == '_' => {
                let start = self.position;
                while self.current.is_ascii_alphanumeric() || self.current == '_' {
                    self._next();
                }
                let word = self.extract_string_from_start_to_current(start)?;
                self.parse_keyword(word)
            }
            current if current == '\'' => {
                self._next();
                let start = self.position;
                while self.current != '\'' && self._next() != '\0' {
                    // TODO: Handle escape characters
                }
                let string = if self.current == '\'' {
                    self.extract_string_from_start_to_current(start)?
                } else {
                    return Err(CompilerError::UnterminatedString(
                        self.line,
                        self.column - (self.position - start),
                    ));
                };
                let token = Token::new(
                    TokenKind::Literal(Literal::from(Arc::new(string.to_string()))),
                    self.line,
                    self.column - string.len(),
                );
                self._next();
                token
            }
            current if current == '\"' => {
                self._next();
                let start = self.position;
                while self.current != '\"' && self._next() != '\0' {
                    // TODO: Handle escape characters
                }
                let string = if self.current == '\"' {
                    self.extract_string_from_start_to_current(start)?
                } else {
                    return Err(CompilerError::UnterminatedString(
                        self.line,
                        self.column - (self.position - start),
                    ));
                };
                let token = Token::new(
                    TokenKind::Literal(Literal::from(Arc::new(string.to_string()))),
                    self.line,
                    self.column - string.len(),
                );
                self._next();
                token
            }
            '+' => {
                let token = Token::new(TokenKind::Symbol(Plus), self.line, self.column);
                self._next();
                token
            }
            '-' => {
                let token = Token::new(TokenKind::Symbol(Minus), self.line, self.column);
                self._next();
                token
            }
            '*' => {
                let token = Token::new(TokenKind::Symbol(Asterisk), self.line, self.column);
                self._next();
                token
            }
            '/' => {
                let token = Token::new(TokenKind::Symbol(Slash), self.line, self.column);
                self._next();
                token
            }
            '%' => {
                let token = Token::new(TokenKind::Symbol(Percent), self.line, self.column);
                self._next();
                token
            }
            '(' => {
                let token = Token::new(TokenKind::Symbol(OpenParanthesis), self.line, self.column);
                self._next();
                token
            }
            ')' => {
                let token = Token::new(TokenKind::Symbol(CloseParanthesis), self.line, self.column);
                self._next();
                token
            }
            '{' => {
                let token = Token::new(TokenKind::Symbol(OpenCurlyBracket), self.line, self.column);
                self._next();
                token
            }
            '}' => {
                let token =
                    Token::new(TokenKind::Symbol(CloseCurlyBracket), self.line, self.column);
                self._next();
                token
            }
            '[' => {
                let token =
                    Token::new(TokenKind::Symbol(OpenSquareBracket), self.line, self.column);
                self._next();
                token
            }
            ']' => {
                let token = Token::new(
                    TokenKind::Symbol(CloseSquareBracket),
                    self.line,
                    self.column,
                );
                self._next();
                token
            }
            '=' => {
                let token = Token::new(TokenKind::Symbol(Equals), self.line, self.column);
                self._next();
                token
            }
            '<' => {
                let token = Token::new(TokenKind::Symbol(LessThan), self.line, self.column);
                self._next();
                token
            }
            '>' => {
                let token = Token::new(TokenKind::Symbol(GreaterThan), self.line, self.column);
                self._next();
                token
            }
            '!' => {
                let token = Token::new(TokenKind::Symbol(Exclamation), self.line, self.column);
                self._next();
                token
            }
            ',' => {
                let token = Token::new(TokenKind::Symbol(Comma), self.line, self.column);
                self._next();
                token
            }
            ':' => {
                let token = Token::new(TokenKind::Symbol(Colon), self.line, self.column);
                self._next();
                token
            }
            _ => {
                return Err(CompilerError::InvalidCharacter(
                    self.current,
                    self.line,
                    self.column,
                ))
            }
        };

        Ok(current_token)
    }

    pub(crate) fn extract_string_from_start_to_current(
        &self,
        start: usize,
    ) -> Result<String, CompilerError> {
        let value = if self.position - start == 0 {
            vec![self.source[start]]
        } else if self.current == '\0' {
            self.source[start..=self.position].to_vec()
        } else {
            self.source[start..self.position].to_vec()
        };
        if let Ok(value_as_string) = String::from_utf8(value) {
            Ok(value_as_string)
        } else {
            Err(CompilerError::InvalidUtf8Character)
        }
    }

    pub(crate) fn lex(&mut self) -> Result<(), CompilerError> {
        loop {
            let token = self.parse_token()?;
            match token.kind {
                TokenKind::Whitespace(_) | TokenKind::NewLine => {}
                TokenKind::EndOfFile => {
                    return Ok(());
                }
                _ => {
                    self.tokens.push(Rc::new(token));
                }
            }
        }
    }

    /// returns tokens\[pointer\]
    pub(crate) fn advance(&self) {
        let mut index = self.index.borrow_mut();
        if *index < self.tokens.len() {
            *index += 1;
        }
    }

    pub(crate) fn peek(&self, offset: usize) -> Rc<Token> {
        let index = *self.index.borrow();
        let token = self.tokens.get(index + offset);
        if let Some(token) = token {
            Rc::clone(token)
        } else {
            Rc::new(Token::new(TokenKind::EndOfFile, self.line, self.column))
        }
    }

    /// returns tokens\[pointer\]
    pub(crate) fn get_current_token(&self) -> Rc<Token> {
        let index = *self.index.borrow();
        let token = self.tokens.get(index);
        if let Some(token) = token {
            Rc::clone(token)
        } else {
            Rc::new(Token::new(TokenKind::EndOfFile, self.line, self.column))
        }
    }

    // TODO: check if needed. if not remove it

    /// returns tokens\[++pointer\]

    // pub(crate) fn advance_and_get_current_token(&self) -> Rc<Token> {
    //     self.advance();
    //     self.get_current_token()
    // }

    /// returns tokens\[pointer++\]
    pub(crate) fn get_current_token_and_advance(&self) -> Rc<Token> {
        let current = self.get_current_token();
        self.advance();
        current
    }

    /// returns tokens.len()
    pub(crate) fn get_token_count(&self) -> usize {
        self.tokens.len()
    }

    // pub(crate)fn generate_factory_token(&self, line: usize, column: usize) -> Rc<Token> {
    //     Rc::new(Token::new(TokenKind::FactoryToken, line, column))
    // }

    fn parse_keyword(&self, word: String) -> Token {
        let keyword = Keyword::get_keyword_kind(&word);
        Token::new(keyword, self.line, self.column - word.len())
    }
}
