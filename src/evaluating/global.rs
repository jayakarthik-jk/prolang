use std::{io::Write, sync::Arc};

use crate::common::errors::CompilerError;
use crate::common::literal::Literal;
use std::io::{stdin, stdout};

type BuiltInFunction = fn(Vec<Literal>) -> Result<Literal, CompilerError>;
lazy_static::lazy_static! {
    static ref GLOBAL_PROPERTIES: Arc<Vec<BuiltInAttributes>> = Arc::new(vec![
        BuiltInAttributes::BuiltInFunctions("print".to_string(), print),
        BuiltInAttributes::BuiltInFunctions("input".to_string(), input),
        BuiltInAttributes::BuiltInProperties("lucky".to_string(), Literal::from(7))
    ]);
}

fn print(variables: Vec<Literal>) -> Result<Literal, CompilerError> {
    for variable in variables.iter() {
        print!("{}", variable);
        stdout().flush().unwrap();
    }
    println!();
    Ok(Literal::from(true))
}

fn input(variables: Vec<Literal>) -> Result<Literal, CompilerError> {
    for variable in variables.iter() {
        print!("{}", variable);
        stdout().flush().unwrap();
    }
    println!();
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    buffer = buffer.replace('\n', "");
    Ok(Literal::from(buffer))
}

enum BuiltInAttributes {
    BuiltInFunctions(String, BuiltInFunction),
    BuiltInProperties(String, Literal),
}

pub(crate) struct GlobalProperties;

impl GlobalProperties {
    pub(crate) fn get_built_in_function(name: &str) -> Option<&'static BuiltInFunction> {
        for property in GLOBAL_PROPERTIES.iter() {
            if let BuiltInAttributes::BuiltInFunctions(function_name, function) = property {
                if name == function_name {
                    return Some(function);
                }
            }
        }
        None
    }
    pub(crate) fn get_built_in_properties(name: &str) -> Option<Literal> {
        for property in GLOBAL_PROPERTIES.iter() {
            if let BuiltInAttributes::BuiltInProperties(property_name, property_value) = property {
                if name == property_name {
                    return Some(property_value.clone());
                }
            }
        }
        None
    }
}
