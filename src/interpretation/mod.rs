use std::rc::Rc;

use crate::common::errors::CompilerError;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::lexer::Lexer;
use crate::syntax_analysis::parser::Parser;

pub fn interpretate(source_code: String) -> Result<(), CompilerError> {
    let mut lexer = Lexer::new(source_code);

    lexer.lex()?;

    let mut parser = Parser::new(lexer);

    let global_block = parser.parse()?;

    for statement in global_block.borrow().statements.iter() {
        let evaluator = Evaluator::new(statement, Rc::clone(&global_block));
        evaluator.evaluate()?;
    }

    println!("{}", global_block.borrow());

    Ok(())
}
