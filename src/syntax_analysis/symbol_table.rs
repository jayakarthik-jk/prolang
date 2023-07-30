use crate::common::datatypes::Variable;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SymbolTable {
    table: HashMap<String, Variable>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, value: Variable) {
        self.table.insert(name, value);
    }

    pub fn contains(&self, name: &String) -> bool {
        self.table.contains_key(name)
    }

    pub fn get(&self, name: &String) -> Option<Variable> {
        self.table.get(name).cloned()
    }

    pub fn remove(&mut self, name: &String) {
        self.table.remove(name);
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn print(&self) {
        let iterator = self.table.iter();
        for (name, value) in iterator {
            println!("{}: {}, mutable {}", name, value, value.is_mutable(),);
        }
    }
}
