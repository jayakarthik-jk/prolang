use std::io::Write;
use std::sync::{Arc, RwLock};

use crate::common::literal::Literal;
use crate::common::{datatypes::DataType, errors::CompilerError};
use crate::parsing::block::Block;
use std::io::{stdin, stdout};

fn to_number(variables: Vec<Literal>) -> Result<Literal, CompilerError> {
    if variables.len() != 1 {
        return Err(CompilerError::ArgumentLengthMismatch(
            "int".to_string(),
            1,
            variables.len(),
        ));
    }
    let variable = variables.first().unwrap();
    match &variable.value {
        DataType::String(string) => {
            let int = string.parse::<i128>();
            if int.is_err() {
                return Err(CompilerError::InvalidStringParsing(variable.clone()));
            }
            Ok(Literal::from(int.unwrap()))
        }
        DataType::Float(float) => Ok(Literal::from(float.floor() as i128)),
        DataType::Integer(_) => Ok(variable.clone()),
        _ => Err(CompilerError::InvalidType(variable.value.to_string())),
    }
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
    buffer = buffer.trim().to_string();
    Ok(Literal::from(buffer))
}

type BuiltInFunction = fn(Vec<Literal>) -> Result<Literal, CompilerError>;
enum BuiltInAttributes {
    BuiltInFunctions(String, BuiltInFunction),
    BuiltInProperties(String, Literal),
}

pub(crate) struct Global {
    properties: Vec<BuiltInAttributes>,
    pub(crate) block: Arc<RwLock<Block>>,
}

impl Global {
    pub(crate) fn new() -> Self {
        Self {
            block: Arc::new(RwLock::new(Block::new())),
            properties: vec![
                BuiltInAttributes::BuiltInFunctions("print".to_string(), print),
                BuiltInAttributes::BuiltInFunctions("input".to_string(), input),
                BuiltInAttributes::BuiltInFunctions("number".to_string(), to_number),
                BuiltInAttributes::BuiltInProperties("lucky".to_string(), Literal::from(7)),
            ],
        }
    }
    pub(crate) fn get_built_in_function(&self, name: &str) -> Option<&BuiltInFunction> {
        for property in self.properties.iter() {
            if let BuiltInAttributes::BuiltInFunctions(function_name, function) = property {
                if name == function_name {
                    return Some(function);
                }
            }
        }
        None
    }
    pub(crate) fn get_built_in_properties(&self, name: &str) -> Option<Literal> {
        for property in self.properties.iter() {
            if let BuiltInAttributes::BuiltInProperties(property_name, property_value) = property {
                if name == property_name {
                    return Some(property_value.clone());
                }
            }
        }
        None
    }
}
