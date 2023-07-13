use compiler::{
    evaluator::Evaluator,
    parser::{Expression, Parser},
};
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
        Expression::print(&expression, "".to_string());
        let result = Evaluator::evaluate(&Box::new(expression)).unwrap();
        println!("Result = {}", result);
    }
}
