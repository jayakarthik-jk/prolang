use crate::common::literal::Literal;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Default)]
pub(crate) struct SymbolTable {
    table: HashMap<String, Literal>,
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, name: String, value: Literal) {
        self.table.insert(name, value);
    }

    pub(crate) fn contains(&self, name: &str) -> bool {
        self.table.contains_key(name)
    }

    pub(crate) fn get(&self, name: &str) -> Option<Literal> {
        self.table.get(name).cloned()
    }

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
