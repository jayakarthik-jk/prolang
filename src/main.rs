pub(crate) mod common;
pub(crate) mod evaluating;
pub(crate) mod lexing;
pub(crate) mod parsing;

use evaluating::{evaluator::Evaluator, global::Global};
use lexing::{FileReader, Lexer};
use parsing::Parser;
use std::{sync::Arc, thread};

fn main() {
    // Global fields and functions
    let global = Global::new();
    // channels
    let (file_chunk_transmitter, file_chunk_receiver) = std::sync::mpsc::channel();
    let (token_transmitter, token_receiver) = std::sync::mpsc::channel();
    let (statement_transmitter, statement_receiver) = std::sync::mpsc::channel();

    let file_reader = FileReader::new("app.prolang", file_chunk_transmitter);
    let lexer = Lexer::new(file_chunk_receiver, token_transmitter);
    let parser = Parser::new(
        token_receiver,
        statement_transmitter,
        Arc::clone(&global.block),
    );
    let evaluator = Evaluator::new(statement_receiver, global);

    thread::scope(move |scope| {
        scope.spawn(move || evaluator.evaluate());
        scope.spawn(move || parser.parse());
        scope.spawn(move || lexer.lex());
        file_reader.read();
    });
}
