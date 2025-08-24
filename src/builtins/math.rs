//! Mathematical built-in functions

use crate::features::BuiltinFunction;
use crate::value::Value;
use crate::error::{JingError, JingResult};

/// Square root function
#[derive(Debug)]
pub struct SqrtFunction;

impl BuiltinFunction for SqrtFunction {
    fn name(&self) -> &str {
        "sqrt"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::Number(n) => {
                if *n < 0.0 {
                    Err(JingError::runtime_error("Cannot take square root of negative number"))
                } else {
                    Ok(Value::Number(n.sqrt()))
                }
            }
            _ => Err(JingError::TypeError { message: "sqrt() expects a number".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "sqrt(number) - Return the square root of a number"
    }
}

/// Absolute value function
#[derive(Debug)]
pub struct AbsFunction;

impl BuiltinFunction for AbsFunction {
    fn name(&self) -> &str {
        "abs"
    }
    
    fn arity(&self) -> usize {
        1
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::Number(n) => Ok(Value::Number(n.abs())),
            _ => Err(JingError::TypeError { message: "abs() expects a number".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "abs(number) - Return the absolute value of a number"
    }
}

/// Maximum of two numbers
#[derive(Debug)]
pub struct MaxFunction;

impl BuiltinFunction for MaxFunction {
    fn name(&self) -> &str {
        "max"
    }
    
    fn arity(&self) -> usize {
        2
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(*b))),
            _ => Err(JingError::TypeError { message: "max() expects two numbers".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "max(a, b) - Return the maximum of two numbers"
    }
}

/// Minimum of two numbers  
#[derive(Debug)]
pub struct MinFunction;

impl BuiltinFunction for MinFunction {
    fn name(&self) -> &str {
        "min"
    }
    
    fn arity(&self) -> usize {
        2
    }
    
    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(*b))),
            _ => Err(JingError::TypeError { message: "min() expects two numbers".to_string() }),
        }
    }
    
    fn help(&self) -> &str {
        "min(a, b) - Return the minimum of two numbers"
    }
}
