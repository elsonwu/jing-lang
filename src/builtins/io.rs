//! I/O built-in functions

use crate::features::BuiltinFunction;
use crate::value::Value;
use crate::error::{JingError, JingResult};
use std::io::{self, Write};

/// Read a line from standard input
#[derive(Debug)]
pub struct ReadLineFunction;

impl BuiltinFunction for ReadLineFunction {
    fn name(&self) -> &str {
        "readline"
    }
    
    fn arity(&self) -> usize {
        0
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if !args.is_empty() {
            return Err(JingError::runtime_error("readline() takes no arguments"));
        }
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Remove trailing newline
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }
                Ok(Value::String(input))
            }
            Err(e) => Err(JingError::runtime_error(format!("Failed to read input: {}", e))),
        }
    }
    
    fn help(&self) -> &str {
        "readline() - Read a line from standard input"
    }
}

/// Read a line with a prompt
#[derive(Debug)]
pub struct InputFunction;

impl BuiltinFunction for InputFunction {
    fn name(&self) -> &str {
        "input"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(prompt) => {
                print!("{}", prompt);
                io::stdout().flush().map_err(|e| {
                    JingError::runtime_error(format!("Failed to flush output: {}", e))
                })?;
                
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        // Remove trailing newline
                        if input.ends_with('\n') {
                            input.pop();
                            if input.ends_with('\r') {
                                input.pop();
                            }
                        }
                        Ok(Value::String(input))
                    }
                    Err(e) => Err(JingError::runtime_error(format!("Failed to read input: {}", e))),
                }
            }
            _ => Err(JingError::TypeError { message: "input() expects a string prompt".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "input(prompt) - Display prompt and read a line from standard input"
    }
}
