use std::rc::Rc;

use crate::common::errors::CompilerError;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::lexer::Lexer;
use crate::semantic_analysis::binder::Binder;
use crate::syntax_analysis::parser::Parser;

pub fn interpretate(source_code: String) -> Result<(), CompilerError> {
    let mut lexer = Lexer::new(source_code);

    lexer.lex()?;

    let mut parser = Parser::new(lexer);

    let global_block = parser.parse()?;

    for statement in global_block.statements.iter() {
        let binder = Binder::new(statement, Rc::clone(&global_block));
        let binded_statement = binder.bind()?;
        let evaluator = Evaluator::new(&binded_statement, Rc::clone(&global_block));
        evaluator.evaluate()?;
    }

    global_block.print();

    Ok(())
}
