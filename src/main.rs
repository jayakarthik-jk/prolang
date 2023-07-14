use compiler::{evaluator::Evaluator, expressions::SyntaxExpression, parser::Parser};
use std::io::stdin;

fn main() {
    let stdin = stdin();
    loop {
        println!("Enter expression:");
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut parser = Parser::new(input);
        let expression = match parser.parse() {
            Ok(expression) => expression,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        SyntaxExpression::print(&expression, "".to_string());
        let result = match Evaluator::evaluate(&Box::new(expression)) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        println!("Result = {:?}", result);
    }
}
