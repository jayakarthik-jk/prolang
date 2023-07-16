use compiler::common::symbol_table::SymbolTable;
use compiler::evaluator::Evaluator;
use compiler::lexical_analysis::lexer::Lexer;
use compiler::semantic_analysis::binder::Binder;
use compiler::syntax_analysis::ast::AbstractSyntaxTree;
use compiler::syntax_analysis::parser::Parser;

use std::io::stdin;

fn main() {
    console_mode();
}

fn console_mode() {
    let stdin = stdin();
    let mut display_progress = true;
    let symbol_table = SymbolTable::sharable();
    loop {
        println!("Enter expression:");

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if let std::cmp::Ordering::Equal = input.trim().cmp(&"progress()".to_string()) {
            display_progress = !display_progress;
            continue;
        }
        if let std::cmp::Ordering::Equal = input.trim().cmp(&"exit()".to_string()) {
            break;
        }

        let mut lexer = Lexer::new(input, symbol_table.clone());

        if let Err(error) = lexer.lex() {
            eprintln!("lexer error: {}", error);
            continue;
        }

        let mut parser = Parser::new(lexer);

        let ast = match parser.parse() {
            Ok(expression) => {
                if display_progress {
                    AbstractSyntaxTree::print(&expression);
                }
                expression
            }
            Err(error) => {
                eprintln!("parser error: {}", error);
                continue;
            }
        };

        let mut binder = Binder::new(ast, symbol_table.clone());

        binder.display_process = display_progress;

        let semantic_tree = match binder.bind() {
            Ok(semantic_tree) => semantic_tree,
            Err(err) => {
                eprintln!("binder error: {}", err);
                continue;
            }
        };

        let evaluator = Evaluator::new(semantic_tree, symbol_table.clone());

        let result = match evaluator.evaluate() {
            Ok(result) => result,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        if display_progress {
            println!("Symbol table:");
            println!("-------------");
            symbol_table.borrow().print();
            println!("-------------");
        }
        println!("Result: {}", result);
    }
}
