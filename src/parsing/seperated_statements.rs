use std::slice::Iter;

use crate::lexing::symbols::Symbol;

use super::ast::AbstractSyntaxTree;

#[derive(Debug)]
pub(crate) struct SeperatedStatements {
    pub(crate) seperated_with: Symbol,
    pub(crate) enclosed_with: Symbol,
    pub(crate) statements: Vec<Box<AbstractSyntaxTree>>,
}

impl SeperatedStatements {
    pub(crate) fn new(
        seperated_with: Symbol,
        enclosed_with: Symbol,
        statements: Vec<Box<AbstractSyntaxTree>>,
    ) -> Self {
        Self {
            seperated_with,
            enclosed_with,
            statements,
        }
    }

    pub(crate) fn iter(&self) -> Iter<Box<AbstractSyntaxTree>> {
        self.statements.iter()
    }
}
