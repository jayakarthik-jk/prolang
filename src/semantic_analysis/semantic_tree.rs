use crate::lexical_analysis::token::Token;
use std::rc::Rc;

pub enum SemanticTree {
    LiteralExpression(Rc<Token>),
    IdentifierExpression(Rc<Token>),
    UnaryExpression(Rc<Token>, Box<SemanticTree>),
    BinaryExpression(Box<SemanticTree>, Rc<Token>, Box<SemanticTree>),
    AssignmentExpression(String, Rc<Token>, Box<SemanticTree>),
}

impl SemanticTree {
    pub fn print(node: &SemanticTree) {
        SemanticTree::print_tree(node, 0);
    }
    fn print_tree(node: &SemanticTree, indent: usize) {
        match node {
            SemanticTree::UnaryExpression(operator, operand) => {
                println!("{}├─{}", " ".repeat(indent), operator);
                SemanticTree::print_tree(operand, indent + 3);
            }
            SemanticTree::BinaryExpression(left, operator, right) => {
                println!("{}├─{}", " ".repeat(indent), operator);
                SemanticTree::print_tree(left, indent + 3);
                SemanticTree::print_tree(right, indent + 3);
            }
            SemanticTree::LiteralExpression(value) => {
                println!("{}└─{}", " ".repeat(indent), value);
            }
            SemanticTree::AssignmentExpression(identifier, equals, expression) => {
                println!("{}├─{}", " ".repeat(indent), equals);
                println!("{}   └─{}", " ".repeat(indent), identifier);
                SemanticTree::print_tree(expression, indent + 3);
            }
            SemanticTree::IdentifierExpression(value) => {
                println!("{}└─{}", " ".repeat(indent), value);
            }
        }
    }
}
