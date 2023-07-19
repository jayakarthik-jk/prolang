use std::cell::RefCell;
use std::rc::Rc;

use super::token::{Token, TokenKind};
use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use crate::common::symbol_table::SymbolTable;
use crate::lexical_analysis::keywords::Keyword;
use crate::lexical_analysis::symbols::Symbol::*;

pub struct Lexer {
    source: Vec<u8>,
    tokens: Vec<Rc<Token>>,
    index: RefCell<usize>,
    position: usize,
    current: char,
    pub line: usize,
    pub column: usize,
    symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Lexer {
    pub fn new(source: String, symbol_table: Rc<RefCell<SymbolTable>>) -> Self {
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
            symbol_table,
        }
    }

    pub fn _next(&mut self) -> char {
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

    pub fn parse_token(&mut self) -> Result<Token, CompilerError> {
        let current_token = match self.current {
            current if current.is_ascii_whitespace() => {
                let start = self.position;
                while self.current.is_ascii_whitespace() {
                    self._next();
                }
                let count = self.position - start;
                let token = Token::new(TokenKind::WhitespaceToken(count), self.line, self.column);
                return Ok(token);
            }
            '\n' => {
                let token = Token::new(TokenKind::NewLineToken, self.line, self.column);
                self._next();
                self.line += 1;
                self.column = 1;
                token
            }
            '\0' => Token::new(TokenKind::EndOfFileToken, self.line, self.column),
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
                        TokenKind::LiteralToken(Variable::from(number)),
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
                        TokenKind::LiteralToken(Variable::from(number)),
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
                    let string = self.extract_string_from_start_to_current(start)?;
                    string
                } else {
                    return Err(CompilerError::UnterminatedString(
                        self.line,
                        self.column - (self.position - start),
                    ));
                };
                let token = Token::new(
                    TokenKind::LiteralToken(Variable::from(string.to_string())),
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
                    let string = self.extract_string_from_start_to_current(start)?;
                    string
                } else {
                    return Err(CompilerError::UnterminatedString(
                        self.line,
                        self.column - (self.position - start),
                    ));
                };
                let token = Token::new(
                    TokenKind::LiteralToken(Variable::from(string.to_string())),
                    self.line,
                    self.column - string.len(),
                );
                self._next();
                token
            }
            '+' => {
                let token = Token::new(TokenKind::SymbolToken(Plus), self.line, self.column);
                self._next();
                token
            }
            '-' => {
                let token = Token::new(TokenKind::SymbolToken(Minus), self.line, self.column);
                self._next();
                token
            }
            '*' => {
                let token = Token::new(TokenKind::SymbolToken(Asterisk), self.line, self.column);
                self._next();
                token
            }
            '/' => {
                let token = Token::new(TokenKind::SymbolToken(Slash), self.line, self.column);
                self._next();
                token
            }
            '%' => {
                let token = Token::new(TokenKind::SymbolToken(Percent), self.line, self.column);
                self._next();
                token
            }
            '(' => {
                let token = Token::new(
                    TokenKind::SymbolToken(OpenParanthesis),
                    self.line,
                    self.column,
                );
                self._next();
                token
            }
            ')' => {
                let token = Token::new(
                    TokenKind::SymbolToken(CloseParanthesis),
                    self.line,
                    self.column,
                );
                self._next();
                token
            }
            '=' => {
                let token = Token::new(TokenKind::SymbolToken(Equals), self.line, self.column);
                self._next();
                token
            }
            '<' => {
                let token = Token::new(TokenKind::SymbolToken(LessThan), self.line, self.column);
                self._next();
                token
            }
            '>' => {
                let token = Token::new(TokenKind::SymbolToken(GreaterThan), self.line, self.column);
                self._next();
                token
            }
            '!' => {
                let token = Token::new(TokenKind::SymbolToken(Exclamation), self.line, self.column);
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

    pub fn extract_string_from_start_to_current(
        &self,
        start: usize,
    ) -> Result<String, CompilerError> {
        let value = if self.position - start == 0 {
            vec![self.source[start]]
        } else {
            if self.current == '\0' {
                self.source[start..=self.position].to_vec()
            } else {
                self.source[start..self.position].to_vec()
            }
        };
        if let Ok(value_as_string) = String::from_utf8(value) {
            Ok(value_as_string)
        } else {
            Err(CompilerError::InvalidUtf8Character)
        }
    }

    pub fn lex_with_whitespace(&mut self) -> Result<(), CompilerError> {
        loop {
            let token = self.parse_token()?;
            match token.kind {
                TokenKind::EndOfFileToken => {
                    return Ok(());
                }
                _ => self.tokens.push(Rc::new(token)),
            }
        }
    }

    pub fn lex(&mut self) -> Result<(), CompilerError> {
        loop {
            let token = self.parse_token()?;
            match token.kind {
                TokenKind::NewLineToken | TokenKind::WhitespaceToken(_) => {}
                TokenKind::EndOfFileToken => {
                    return Ok(());
                }
                _ => {
                    self.tokens.push(Rc::new(token));
                }
            }
        }
    }

    /// returns tokens\[pointer\]
    pub fn advance(&self) {
        let mut index = self.index.borrow_mut();
        if *index < self.tokens.len() {
            *index += 1;
        }
    }

    /// returns --pointer
    pub fn rewind(&self) {
        let mut index = self.index.borrow_mut();
        if *index > 0 {
            *index -= 1;
        }
    }

    pub fn peek(&self, offset: usize) -> Rc<Token> {
        let index = *self.index.borrow();
        let token = self.tokens.get(index + offset);
        if let Some(token) = token {
            Rc::clone(token)
        } else {
            Rc::new(Token::new(
                TokenKind::EndOfFileToken,
                self.line,
                self.column,
            ))
        }
    }

    /// returns tokens\[pointer\]
    pub fn get_current_token(&self) -> Rc<Token> {
        let index = *self.index.borrow();
        let token = self.tokens.get(index);
        if let Some(token) = token {
            Rc::clone(token)
        } else {
            Rc::new(Token::new(
                TokenKind::EndOfFileToken,
                self.line,
                self.column,
            ))
        }
    }

    /// returns tokens\[++pointer\]
    pub fn advance_and_get_current_token(&self) -> Rc<Token> {
        self.advance();
        self.get_current_token()
    }

    /// returns tokens\[--pointer\]
    pub fn rewind_and_get_current_token(&self) -> Rc<Token> {
        self.rewind();
        self.get_current_token()
    }

    /// returns tokens\[pointer++\]
    pub fn get_current_token_and_advance(&self) -> Rc<Token> {
        let current = self.get_current_token();
        self.advance();
        current
    }

    /// returns tokens\[pointer--\]
    pub fn get_current_token_and_rewind(&self) -> Rc<Token> {
        let current = self.get_current_token();
        self.rewind();
        current
    }

    /// returns tokens.len()
    pub fn get_token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn generate_factory_token(&self, line: usize, column: usize) -> Rc<Token> {
        Rc::new(Token::new(TokenKind::FactoryToken, line, column))
    }

    fn parse_keyword(&self, word: String) -> Token {
        let keyword = Keyword::get_keyword_kind(&word);
        let symbol_table = self.symbol_table.borrow_mut();
        if let TokenKind::IdentifierToken(name) = &keyword {
            if !symbol_table.variables.contains_key(name) {
                // symbol_table
                //     .variables
                //     .insert(name.clone(), Variable::InternalUndefined);
                // TODO
            }
        }
        Token::new(keyword, self.line, self.column - word.len())
    }
}
