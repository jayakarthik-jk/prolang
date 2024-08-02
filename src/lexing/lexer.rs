use std::{
    collections::VecDeque,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use super::keywords::Keyword;
use super::token::{Token, TokenKind};
use crate::common::literal::Literal;
use crate::lexing::symbols::Symbol::*;
use crate::lexing::token::TokenKind::*;

pub(crate) struct Lexer {
    file_chunk_receiver: Receiver<VecDeque<u8>>,
    token_transmitter: Sender<Token>,
    current_chunks: VecDeque<u8>,
    line: usize,
    column: usize,
}

impl Lexer {
    pub(crate) fn new(
        file_chunk_receiver: Receiver<VecDeque<u8>>,
        token_transmitter: Sender<Token>,
    ) -> Self {
        Self {
            file_chunk_receiver,
            token_transmitter,
            current_chunks: VecDeque::default(),
            column: 0,
            line: 0,
        }
    }

    pub(crate) fn next(&mut self) -> char {
        let val = if !self.current_chunks.is_empty() {
            self.current_chunks.pop_front().unwrap_or(0) as char
        } else if let Ok(mut chunk) = self.file_chunk_receiver.recv() {
            let ch = chunk.pop_front();
            self.current_chunks = chunk;
            ch.unwrap_or(0) as char
        } else {
            '\0'
        };

        if val != '\0' {
            self.column += 1;
        }
        val
    }

    pub(crate) fn lex(mut self) {
        let mut temp = None;
        loop {
            let mut current = if let Some(ch) = temp {
                temp = None;
                ch
            } else {
                self.next()
            };

            if current == '\0' {
                let eof = Token::new(TokenKind::EndOfFile, self.line, self.column);
                self.token_transmitter.send(eof).unwrap();
                return;
            }

            let token = match current {
                '\r' => {
                    if self.next() != '\n' {
                        panic!("Expected '\\n' after '\\r'")
                    }
                    self.line += 1;
                    // let token = Token::new(NewLine, self.increment_line(), self.column);
                    // token
                    continue;
                }
                '\n' => {
                    self.line += 1;
                    // let token = Token::new(NewLine, self.increment_line(), self.column);
                    // token
                    continue;
                }
                ch if ch.is_ascii_whitespace() => {
                    // let token = Token::new(Whitespace, self.line, self.column);
                    // token
                    continue;
                }
                ch if ch.is_ascii_digit() => {
                    let mut number_as_string = String::from(ch);
                    current = self.next();
                    while current.is_ascii_digit() {
                        number_as_string.push(current);
                        current = self.next();
                    }
                    if current == '.' {
                        number_as_string.push('.');
                        current = self.next();
                        while current.is_ascii_digit() {
                            number_as_string.push(current);
                            current = self.next();
                        }
                    }
                    temp = Some(current);
                    let number: f64 = match number_as_string.parse() {
                        Ok(number) => number,
                        Err(_) => {
                            panic!("Invalid Number")
                        }
                    };
                    Token::new(
                        TokenKind::Literal(Literal::from(number)),
                        self.line,
                        self.column,
                    )
                }
                ch if ch.is_alphabetic() || ch == '_' => {
                    let mut word = String::from(ch);
                    current = self.next();
                    while current.is_ascii_alphanumeric() || current == '_' {
                        word.push(current);
                        current = self.next();
                    }
                    temp = Some(current);
                    let keyword = Keyword::get_keyword_kind(&word);
                    Token::new(keyword, self.line, self.column)
                }
                '\'' => {
                    current = self.next();
                    let mut word = String::new();
                    while current != '\'' && current != '\0' {
                        word.push(current);
                        current = self.next();
                    }
                    if current != '\'' {
                        panic!("Unterminated string")
                    };
                    Token::new(
                        Literal(Literal::from(Arc::new(word.to_string()))),
                        self.line,
                        self.column,
                    )
                }
                '\"' => {
                    current = self.next();
                    let mut word = String::new();
                    while current != '\"' && current != '\0' {
                        word.push(current);
                        current = self.next();
                    }
                    if current != '\"' {
                        panic!("Unterminated string")
                    };
                    Token::new(
                        TokenKind::Literal(Literal::from(Arc::new(word.to_string()))),
                        self.line,
                        self.column,
                    )
                }
                '+' => Token::new(Symbol(Plus), self.line, self.column),
                '-' => Token::new(Symbol(Minus), self.line, self.column),
                '*' => Token::new(Symbol(Asterisk), self.line, self.column),
                '/' => Token::new(Symbol(Slash), self.line, self.column),
                '%' => Token::new(Symbol(Percent), self.line, self.column),
                '(' => Token::new(Symbol(OpenParanthesis), self.line, self.column),
                ')' => Token::new(Symbol(CloseParanthesis), self.line, self.column),
                '{' => Token::new(Symbol(OpenCurlyBracket), self.line, self.column),
                '}' => Token::new(Symbol(CloseCurlyBracket), self.line, self.column),
                '[' => Token::new(Symbol(OpenSquareBracket), self.line, self.column),
                ']' => Token::new(Symbol(CloseSquareBracket), self.line, self.column),
                '=' => Token::new(Symbol(Equals), self.line, self.column),
                '<' => Token::new(Symbol(LessThan), self.line, self.column),
                '>' => Token::new(Symbol(GreaterThan), self.line, self.column),
                '!' => Token::new(Symbol(Exclamation), self.line, self.column),
                ',' => Token::new(Symbol(Comma), self.line, self.column),
                ':' => Token::new(Symbol(Colon), self.line, self.column),
                _ => {
                    panic!("invalid character")
                }
            };

            self.token_transmitter.send(token).unwrap();
        }
    }
}
