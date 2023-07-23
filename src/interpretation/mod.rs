use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::lexer::Lexer;
use crate::semantic_analysis::binder::Binder;
use crate::syntax_analysis::parser::Parser;

pub fn interpretate(source_code: String) -> Result<Variable, CompilerError> {
    let mut lexer = Lexer::new(source_code);

    lexer.lex()?;

    let mut parser = Parser::new(lexer);

    let ast = parser.parse()?;

    let binder = Binder::new(ast);

    let semantic_tree = binder.bind()?;

    let evaluator = Evaluator::new(semantic_tree);

    let result = evaluator.evaluate()?;

    return Ok(result);
}
