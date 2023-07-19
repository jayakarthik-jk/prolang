use crate::common::{datatypes::Variable, operators::Operator};

pub enum SemanticTree {
    LiteralExpression(Variable),
    IdentifierExpression(String),
    UnaryExpression(Operator, Box<SemanticTree>),
    BinaryExpression(Box<SemanticTree>, Operator, Box<SemanticTree>),
    AssignmentExpression(String, Operator, Box<SemanticTree>),
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
