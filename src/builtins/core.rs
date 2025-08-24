//! Core built-in functions

use crate::error::{JingError, JingResult};
use crate::features::BuiltinFunction;
use crate::value::Value;

/// Print function - displays values to stdout
#[derive(Debug)]
pub struct PrintFunction;

impl BuiltinFunction for PrintFunction {
    fn name(&self) -> &str {
        "print"
    }

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if args.len() != 1 {
            return Err(JingError::runtime_error("print() takes exactly 1 argument"));
        }

        println!("{}", args[0]);
        Ok(Value::Nil)
    }

    fn help(&self) -> &str {
        "print(value) - Print a value to standard output"
    }
}

/// Type function - returns the type name of a value
#[derive(Debug)]
pub struct TypeFunction;

impl BuiltinFunction for TypeFunction {
    fn name(&self) -> &str {
        "type"
    }

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if args.len() != 1 {
            return Err(JingError::runtime_error("type() takes exactly 1 argument"));
        }

        let type_name = match &args[0] {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Nil => "nil",
            Value::Function { .. } => "function",
            Value::BuiltinFunction { .. } => "builtin_function",
        };

        Ok(Value::String(type_name.to_string()))
    }

    fn help(&self) -> &str {
        "type(value) - Return the type name of a value"
    }
}
