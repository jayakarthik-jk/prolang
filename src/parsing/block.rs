use std::fmt::Display;
use std::sync::{Arc, Mutex, RwLock};

use super::ast::AbstractSyntaxTree;
use super::symbol_table::SymbolTable;
use crate::common::variables::Variable;

#[derive(Debug, Default)]
pub(crate) struct Block {
    pub(crate) parent: Option<Arc<RwLock<Block>>>,
    pub(crate) statements: Vec<AbstractSyntaxTree>,
    symbols: Arc<Mutex<SymbolTable>>,
}

impl Block {
    pub(crate) fn new() -> Self {
        Self {
            statements: vec![],
            symbols: Arc::new(Mutex::new(SymbolTable::new())),
            parent: None,
        }
    }

    pub(crate) fn add_symbol(&self, name: String, value: Variable) {
        if self
            .update_parent_symbol(name.clone(), value.clone())
            .is_none()
        {
            self.symbols.lock().unwrap().add(name, value);
        }
    }

    fn update_symbol(&self, name: String, value: Variable) {
        self.symbols.lock().unwrap().add(name, value)
    }

    fn current_contains_symbol(&self, name: &String) -> bool {
        self.symbols.lock().unwrap().contains(name)
    }

    fn update_parent_symbol(&self, name: String, value: Variable) -> Option<()> {
        if self.current_contains_symbol(&name) {
            self.update_symbol(name, value);
            Some(())
        } else if let Some(parent) = self.parent.as_ref() {
            Block::update_parent_symbol(&parent.read().unwrap(), name, value)
        } else {
            None
        }
    }

    pub(crate) fn contains_symbol(&self, name: &String) -> bool {
        if self.symbols.lock().unwrap().contains(name) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            Block::contains_symbol(&parent.read().unwrap(), name)
        } else {
            false
        }
    }

    pub(crate) fn get_symbol(&self, name: &String) -> Option<Variable> {
        if let Some(variable) = self.symbols.lock().unwrap().get(name) {
            Some(variable)
        } else if let Some(parent) = self.parent.as_ref() {
            Block::get_symbol(&parent.read().unwrap(), name)
        } else {
            None
        }
    }

    // TODO: check if needed. if not remove it.
    // pub(crate) fn remove_symbol(&self, name: &String) {
    //     self.symbols.borrow_mut().remove(name);
    // }

    pub(crate) fn clear_symbols(&self) {
        self.symbols.lock().unwrap().clear();
    }
}

impl From<Vec<AbstractSyntaxTree>> for Block {
    fn from(statements: Vec<AbstractSyntaxTree>) -> Self {
        Self {
            statements,
            symbols: Arc::new(Mutex::new(SymbolTable::new())),
            parent: None,
        }
    }
}

impl From<Arc<RwLock<Block>>> for Block {
    fn from(parent: Arc<RwLock<Block>>) -> Self {
        Self {
            statements: vec![],
            symbols: Arc::new(Mutex::new(SymbolTable::new())),
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
            self.symbols.lock().unwrap()
        )
    }
}
