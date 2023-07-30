use std::cell::RefCell;
use std::rc::Rc;

use super::ast::AbstractSyntaxTree;
use super::symbol_table::SymbolTable;
use crate::common::datatypes::Variable;

#[derive(Debug, Default)]
pub struct Block {
    pub parent: Option<Rc<Block>>,
    pub statements: Vec<Box<AbstractSyntaxTree>>,
    symbols: Rc<RefCell<SymbolTable>>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: vec![],
            symbols: Rc::new(RefCell::new(SymbolTable::new())),
            parent: None,
        }
    }
    pub fn add_symbol(&self, name: String, value: Variable) {
        self.symbols.borrow_mut().add(name, value)
    }

    pub fn contains_symbol(&self, name: &String) -> bool {
        if self.symbols.borrow().contains(name) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            Block::contains_symbol(parent, name)
        } else {
            false
        }
    }

    pub fn get_symbol(&self, name: &String) -> Option<Variable> {
        if let Some(variable) = self.symbols.borrow().get(name) {
            Some(variable)
        } else if let Some(parent) = self.parent.as_ref() {
            Block::get_symbol(parent, name)
        } else {
            None
        }
    }

    pub fn remove_symbol(&self, name: &String) {
        self.symbols.borrow_mut().remove(name);
    }

    pub fn clear_symbols(&self) {
        self.symbols.borrow_mut().clear();
    }

    pub fn print(&self) {
        self.symbols.borrow().print();
    }
}

impl From<Vec<Box<AbstractSyntaxTree>>> for Block {
    fn from(statements: Vec<Box<AbstractSyntaxTree>>) -> Self {
        Self {
            statements,
            symbols: Rc::new(RefCell::new(SymbolTable::new())),
            parent: None,
        }
    }
}

impl From<Rc<Block>> for Block {
    fn from(parent: Rc<Block>) -> Self {
        Self {
            statements: vec![],
            symbols: Rc::new(RefCell::new(SymbolTable::new())),
            parent: Some(parent),
        }
    }
}
