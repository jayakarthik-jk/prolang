use compiler::{evaluator::Evaluator, expressions::SyntaxExpression, parser::Parser};
use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut show_tree = true;
    loop {
        println!("Enter expression:");
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        if let std::cmp::Ordering::Equal = input.trim().cmp(&"tree()".to_string()) {
            show_tree = !show_tree;
            continue;
        }
        if let std::cmp::Ordering::Equal = input.trim().cmp(&"exit()".to_string()) {
            break;
        }
        let mut parser = Parser::new(input);
        let expression = match parser.parse() {
            Ok(expression) => expression,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        if show_tree {
            SyntaxExpression::print(&expression, "".to_string());
        }
        let result = match Evaluator::evaluate(&Box::new(expression)) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        println!("{}", result);
    }
}
