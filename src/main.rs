use prolang::common::symbol_table::SymbolTable;
use prolang::evaluator::Evaluator;
use prolang::lexical_analysis::lexer::Lexer;
use prolang::semantic_analysis::binder::Binder;
use prolang::syntax_analysis::ast::AbstractSyntaxTree;
use prolang::syntax_analysis::parser::Parser;

use std::io::stdin;
use std::io::Write;

fn main() {
    console_mode();
}

fn console_mode() {
    let stdin = stdin();
    let mut display_progress = true;
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // println!("{:?}", input);

        if let std::cmp::Ordering::Equal = input.trim().cmp(&"progress()".to_string()) {
            display_progress = !display_progress;
            continue;
        }
        if let std::cmp::Ordering::Equal = input.trim().cmp(&"exit()".to_string()) {
            break;
        }

        let mut lexer = Lexer::new(input);

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

        let mut binder = Binder::new(ast);

        binder.display_process = display_progress;

        let semantic_tree = match binder.bind() {
            Ok(semantic_tree) => semantic_tree,
            Err(err) => {
                eprintln!("binder error: {}", err);
                continue;
            }
        };

        // SemanticTree::print(&semantic_tree);

        let evaluator = Evaluator::new(semantic_tree);

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
            SymbolTable::print();
            println!("-------------");
        }
        println!("{}", result);
    }
}
