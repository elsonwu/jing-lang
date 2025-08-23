use crate::error::{JingError, JingResult};
use std::collections::HashMap;
use std::fmt;

/// Values in Jing are dynamically typed
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Function {
        name: String,
        arity: usize,
        chunk_start: usize,
    },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Function { name, arity, .. } => {
                write!(f, "<fn {}({} args)>", name, arity)
            }
        }
    }
}

impl Value {
    /// Check if the value is truthy (following Lua-like semantics)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    /// Check if the value is falsy
    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    /// Get the type name of the value
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Function { .. } => "function",
        }
    }

    /// Convert value to string representation
    /// Convert value to string representation for concatenation
    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        }
    }

    /// Convert value to number if possible
    pub fn to_number(&self) -> JingResult<f64> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| JingError::type_error(format!("Cannot convert '{}' to number", s))),
            _ => Err(JingError::type_error(format!(
                "Cannot convert {} to number",
                self.type_name()
            ))),
        }
    }

    /// Add two values
    pub fn add(&self, other: &Value) -> JingResult<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), other) => Ok(Value::String(format!("{}{}", a, other))),
            (self_val, Value::String(b)) => Ok(Value::String(format!("{}{}", self_val, b))),
            _ => Err(JingError::type_error(format!(
                "Cannot add {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    /// Subtract two values
    pub fn subtract(&self, other: &Value) -> JingResult<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(JingError::type_error(format!(
                "Cannot subtract {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    /// Multiply two values
    pub fn multiply(&self, other: &Value) -> JingResult<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(JingError::type_error(format!(
                "Cannot multiply {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    /// Divide two values
    pub fn divide(&self, other: &Value) -> JingResult<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err(JingError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(JingError::type_error(format!(
                "Cannot divide {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    /// Modulo operation
    pub fn modulo(&self, other: &Value) -> JingResult<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err(JingError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Number(a % b))
                }
            }
            _ => Err(JingError::type_error(format!(
                "Cannot modulo {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    /// Negate a value
    pub fn negate(&self) -> JingResult<Value> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(JingError::type_error(format!(
                "Cannot negate {}",
                self.type_name()
            ))),
        }
    }

    /// Logical NOT
    pub fn not(&self) -> Value {
        Value::Bool(self.is_falsy())
    }

    /// Compare two values for equality
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }

    /// Compare two values for less than
    pub fn less_than(&self, other: &Value) -> JingResult<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a < b),
            (Value::String(a), Value::String(b)) => Ok(a < b),
            _ => Err(JingError::type_error(format!(
                "Cannot compare {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    /// Compare two values for greater than
    pub fn greater_than(&self, other: &Value) -> JingResult<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a > b),
            (Value::String(a), Value::String(b)) => Ok(a > b),
            _ => Err(JingError::type_error(format!(
                "Cannot compare {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }
}

/// Environment for storing variables
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> JingResult<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(JingError::runtime_error(format!(
            "Undefined variable '{}'",
            name
        )))
    }

    pub fn set(&mut self, name: &str, value: Value) -> JingResult<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(JingError::runtime_error(format!(
            "Undefined variable '{}'",
            name
        )))
    }
}
