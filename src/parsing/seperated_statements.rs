use std::slice::Iter;

use crate::lexing::symbols::Symbol;

#[derive(Debug)]
pub(crate) struct SeperatedStatements<Item> {
    pub(crate) seperated_with: Symbol,
    pub(crate) enclosed_with: Symbol,
    pub(crate) statements: Vec<Item>,
}

impl<Item> SeperatedStatements<Item> {
    pub(crate) fn new(
        seperated_with: Symbol,
        enclosed_with: Symbol,
        statements: Vec<Item>,
    ) -> Self {
        Self {
            seperated_with,
            enclosed_with,
            statements,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.statements.len()
    }

    pub(crate) fn iter(&self) -> Iter<Item> {
        self.statements.iter()
    }
}
