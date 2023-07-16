use crate::lexical_analysis::lexer::Token;
use std::rc::Rc;

pub enum SemanticTree {
    LiteralExpression(Rc<Token>),
    IdentifierExpression(Rc<Token>),
    UnaryExpression(Rc<Token>, Box<SemanticTree>),
    BinaryExpression(Box<SemanticTree>, Rc<Token>, Box<SemanticTree>),
    AssignmentExpression(String, Rc<Token>, Box<SemanticTree>),
}
