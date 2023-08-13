use std::rc::Rc;

use crate::common::errors::CompilerError;
use crate::evaluating::evaluator::Evaluator;
use crate::lexing::lexer::Lexer;
use crate::parsing::parser::Parser;

pub fn interpretate(source_code: String) -> Result<(), CompilerError> {
    let mut lexer = Lexer::new(source_code);

    lexer.lex()?;

    let mut parser = Parser::new(lexer);

    let global_block = parser.parse()?;

    for statement in global_block.borrow().statements.iter() {
        let evaluator = Evaluator::new(statement, Rc::clone(&global_block));
        evaluator.evaluate()?;
    }

    Ok(())
}
