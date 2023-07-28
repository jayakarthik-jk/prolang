use std::collections::HashMap;

use crate::common::datatypes::Variable;

use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref SYMBOL_TABLE: Mutex<HashMap<String, Variable>> = Mutex::new(HashMap::new());
}

pub struct SymbolTable;

impl SymbolTable {
    pub fn add(name: String, value: Variable) {
        SYMBOL_TABLE.lock().unwrap().insert(name, value);
    }

    pub fn contains(name: &String) -> bool {
        SYMBOL_TABLE.lock().unwrap().contains_key(name)
    }

    pub fn get(name: &String) -> Option<Variable> {
        SYMBOL_TABLE.lock().unwrap().get(name).cloned()
    }

    pub fn remove(name: &String) {
        SYMBOL_TABLE.lock().unwrap().remove(name);
    }

    pub fn clear() {
        SYMBOL_TABLE.lock().unwrap().clear();
    }

    pub fn print() {
        let table = SYMBOL_TABLE.lock().unwrap();
        let iterator = table.iter();
        for (name, value) in iterator {
            println!("{}: {}, mutable {}", name, value, value.is_mutable(),);
        }
    }
}
