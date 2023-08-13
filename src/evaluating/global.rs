use std::{io::Write, sync::Arc};

use crate::common::datatypes::Variable;
use crate::common::errors::CompilerError;
use std::io::{stdin, stdout};

lazy_static::lazy_static! {
    static ref GLOBAL_PROPERTIES: Arc<Vec<BuiltInAttributes>> = Arc::new(vec![
        BuiltInAttributes::BuiltInFunctions("print".to_string(), print),
        BuiltInAttributes::BuiltInFunctions("input".to_string(), input),
        BuiltInAttributes::BuiltInProperties("PI".to_string(), Variable::from(3.14))
    ]);
}

fn print(variables: Vec<Variable>) -> Result<Variable, CompilerError> {
    for variable in variables.iter() {
        print!("{}", variable);
        stdout().flush().unwrap();
    }
    return Ok(Variable::from(true));
}

fn input(variables: Vec<Variable>) -> Result<Variable, CompilerError> {
    for variable in variables.iter() {
        print!("{}", variable);
        stdout().flush().unwrap();
    }
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    return Ok(Variable::from(buffer));
}

enum BuiltInAttributes {
    BuiltInFunctions(String, fn(Vec<Variable>) -> Result<Variable, CompilerError>),
    BuiltInProperties(String, Variable),
}

pub(crate) struct GlobalProperties;

impl GlobalProperties {
    pub(crate) fn get_built_in_function(
        name: &String,
    ) -> Option<&'static fn(Vec<Variable>) -> Result<Variable, CompilerError>> {
        for property in GLOBAL_PROPERTIES.iter() {
            if let BuiltInAttributes::BuiltInFunctions(function_name, function) = property {
                if name == function_name {
                    return Some(function);
                }
            }
        }
        None
    }
    pub(crate) fn get_built_in_properties(name: &String) -> Option<Variable> {
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
