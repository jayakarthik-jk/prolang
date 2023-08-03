use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::ast::AbstractSyntaxTree;
use super::symbol_table::SymbolTable;
use crate::common::datatypes::Variable;

#[derive(Debug, Default)]
pub struct Block {
    pub parent: Option<Rc<RefCell<Block>>>,
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
        if self
            .update_parent_symbol(name.clone(), value.clone())
            .is_none()
        {
            self.symbols.borrow_mut().add(name, value);
        }
    }

    fn update_symbol(&self, name: String, value: Variable) {
        self.symbols.borrow_mut().add(name, value)
    }

    fn current_contains_symbol(&self, name: &String) -> bool {
        self.symbols.borrow().contains(name)
    }

    fn update_parent_symbol(&self, name: String, value: Variable) -> Option<()> {
        if self.current_contains_symbol(&name) {
            self.update_symbol(name, value);
            Some(())
        } else if let Some(parent) = self.parent.as_ref() {
            Block::update_parent_symbol(&parent.borrow(), name, value)
        } else {
            None
        }
    }

    pub fn contains_symbol(&self, name: &String) -> bool {
        if self.symbols.borrow().contains(name) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            Block::contains_symbol(&parent.borrow(), name)
        } else {
            false
        }
    }

    pub fn get_symbol(&self, name: &String) -> Option<Variable> {
        if let Some(variable) = self.symbols.borrow().get(name) {
            Some(variable)
        } else if let Some(parent) = self.parent.as_ref() {
            Block::get_symbol(&parent.borrow(), name)
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

impl From<Rc<RefCell<Block>>> for Block {
    fn from(parent: Rc<RefCell<Block>>) -> Self {
        Self {
            statements: vec![],
            symbols: Rc::new(RefCell::new(SymbolTable::new())),
            parent: Some(parent),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "total Statements: {}\nSymbols: {}",
            self.statements.len(),
            self.symbols.borrow()
        )
    }
}
