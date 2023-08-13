use crate::common::datatypes::Variable;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Default)]
pub(crate) struct SymbolTable {
    table: HashMap<String, Variable>,
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, name: String, value: Variable) {
        self.table.insert(name, value);
    }

    pub(crate) fn contains(&self, name: &String) -> bool {
        self.table.contains_key(name)
    }

    pub(crate) fn get(&self, name: &String) -> Option<Variable> {
        self.table.get(name).cloned()
    }

    // TODO: check if needed. if not remove it.
    // pub(crate) fn remove(&mut self, name: &String) {
    //     self.table.remove(name);
    // }

    pub(crate) fn clear(&mut self) {
        self.table.clear();
    }
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_str = String::new();
        for (name, value) in self.table.iter() {
            print_str.push_str(&format!(
                "\n{}: {}, mutable {}",
                name,
                value,
                value.is_mutable(),
            ));
        }
        write!(f, "{}", print_str)
    }
}
