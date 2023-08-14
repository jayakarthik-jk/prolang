use std::fmt::Display;

use crate::parsing::ast::AbstractSyntaxTree;
use crate::parsing::seperated_statements::SeperatedStatements;

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) block: AbstractSyntaxTree,
    pub(crate) parameters: SeperatedStatements<AbstractSyntaxTree>,
}

impl PartialEq for Function {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Function {
    pub(crate) fn new(
        block: AbstractSyntaxTree,
        parameters: SeperatedStatements<AbstractSyntaxTree>,
    ) -> Self {
        Self { block, parameters }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}
