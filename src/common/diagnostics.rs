use std::sync::Mutex;

use super::errors::CompilerError;
lazy_static::lazy_static! {
    static ref DIAGNOSTICS_VECTOR: Mutex<Vec<CompilerError>> = Mutex::new(Vec::new());
}


// Define the Diagnostics struct
pub struct Diagnostics;

impl Diagnostics {
    // Function to add an error to the vector
    pub fn add_error(error: CompilerError) {
        DIAGNOSTICS_VECTOR.lock().unwrap().push(error);
    }


    // Function to read all errors in the vector
    pub fn read_errors() -> Vec<CompilerError> {
        DIAGNOSTICS_VECTOR.lock().unwrap().clone()
    }
}
