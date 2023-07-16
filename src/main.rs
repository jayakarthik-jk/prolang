use compiler::common::symbol_table::SymbolTable;
use compiler::evaluator::Evaluator;
use compiler::lexical_analysis::lexer::Lexer;
use compiler::semantic_analysis::binder::Binder;
use compiler::syntax_analysis::ast::AbstractSyntaxTree;
use compiler::syntax_analysis::parser::Parser;

use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

fn main() {
    console_mode();
}

fn console_mode() {
    let stdin = stdin();
    let mut show_tree = true;
    let symbol_table = Rc::new(RefCell::new(SymbolTable::new()));
    loop {
        println!("Enter expression:");
        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        if let std::cmp::Ordering::Equal = input.trim().cmp(&"tree()".to_string()) {
            show_tree = !show_tree;
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
                if show_tree {
                    AbstractSyntaxTree::print(&expression);
                }
                expression
            }
            Err(error) => {
                eprintln!("parser error: {}", error);
                continue;
            }
        };

        let binder = Binder::new(ast, symbol_table.clone());

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

        println!("Symbol table:");
        println!("-------------");
        symbol_table.borrow().print();
        println!("-------------");
        println!("Result: {}", result);
    }
}
