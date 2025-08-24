//! String manipulation built-in functions

use crate::features::BuiltinFunction;
use crate::value::Value;
use crate::error::{JingError, JingResult};

/// String length function
#[derive(Debug)]
pub struct LenFunction;

impl BuiltinFunction for LenFunction {
    fn name(&self) -> &str {
        "len"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(s) => Ok(Value::Number(s.len() as f64)),
            _ => Err(JingError::TypeError { message: "len() expects a string".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "len(string) - Return the length of a string"
    }
}

/// Convert string to uppercase
#[derive(Debug)]
pub struct UpperFunction;

impl BuiltinFunction for UpperFunction {
    fn name(&self) -> &str {
        "upper"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => Err(JingError::TypeError { message: "upper() expects a string".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "upper(string) - Convert string to uppercase"
    }
}

/// Convert string to lowercase
#[derive(Debug)]
pub struct LowerFunction;

impl BuiltinFunction for LowerFunction {
    fn name(&self) -> &str {
        "lower"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            _ => Err(JingError::TypeError { message: "lower() expects a string".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "lower(string) - Convert string to lowercase"  
    }
}

/// Reverse a string
#[derive(Debug)]
pub struct ReverseFunction;

impl BuiltinFunction for ReverseFunction {
    fn name(&self) -> &str {
        "reverse"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if args.len() != 1 {
            return Err(JingError::runtime_error("reverse() takes exactly 1 argument"));
        }
        
        match &args[0] {
            Value::String(s) => {
                let reversed: String = s.chars().rev().collect();
                Ok(Value::String(reversed))
            }
            _ => Err(JingError::runtime_error("reverse() argument must be a string")),
        }
    }
    
    fn help(&self) -> &str {
        "reverse(string) - Reverse the characters in a string"
    }
}
