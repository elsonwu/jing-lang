use crate::bytecode::{Chunk, OpCode, Value, Function};
use std::collections::HashMap;

pub struct VM {
    pub stack: Vec<Value>,
    globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
}

#[derive(Debug, Clone)]
struct CallFrame {
    function: Function,
    ip: usize,           // instruction pointer
    stack_offset: usize, // where this frame's locals start on the stack
}

#[derive(Debug)]
pub enum RuntimeError {
    StackUnderflow,
    TypeError(String),
    UndefinedVariable(String),
    UndefinedFunction(String),
    ArityMismatch { expected: usize, got: usize },
    DivisionByZero,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::StackUnderflow => write!(f, "Stack underflow"),
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            RuntimeError::UndefinedFunction(name) => write!(f, "Undefined function: {}", name),
            RuntimeError::ArityMismatch { expected, got } => {
                write!(f, "Expected {} arguments but got {}", expected, got)
            }
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: HashMap::new(),
            frames: Vec::new(),
        }
    }
    
    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), RuntimeError> {
        // Create a main function to hold the top-level code
        let main_function = Function {
            name: "main".to_string(),
            arity: 0,
            chunk,
        };
        
        self.frames.push(CallFrame {
            function: main_function,
            ip: 0,
            stack_offset: 0,
        });
        
        self.run()
    }
    
    fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            if self.frames.is_empty() {
                break;
            }
            
            let instruction = {
                let frame = self.frames.last().unwrap();
                if frame.ip >= frame.function.chunk.code.len() {
                    self.frames.pop();
                    continue;
                }
                frame.function.chunk.code[frame.ip].clone()
            };
            
            // Advance IP
            self.frames.last_mut().unwrap().ip += 1;
            
            match instruction {
                OpCode::Constant(index) => {
                    let frame = self.frames.last().unwrap();
                    let value = frame.function.chunk.constants[index].clone();
                    self.stack.push(value);
                }
                OpCode::Pop => {
                    self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                }
                OpCode::Add => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b));
                        }
                        (Value::String(a), Value::String(b)) => {
                            self.stack.push(Value::String(format!("{}{}", a, b)));
                        }
                        _ => return Err(RuntimeError::TypeError("Invalid operands for +".to_string())),
                    }
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a - b));
                        }
                        _ => return Err(RuntimeError::TypeError("Invalid operands for -".to_string())),
                    }
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a * b));
                        }
                        _ => return Err(RuntimeError::TypeError("Invalid operands for *".to_string())),
                    }
                }
                OpCode::Divide => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            if b == 0.0 {
                                return Err(RuntimeError::DivisionByZero);
                            }
                            self.stack.push(Value::Number(a / b));
                        }
                        _ => return Err(RuntimeError::TypeError("Invalid operands for /".to_string())),
                    }
                }
                OpCode::Negate => {
                    let value = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    match value {
                        Value::Number(n) => self.stack.push(Value::Number(-n)),
                        _ => return Err(RuntimeError::TypeError("Invalid operand for unary -".to_string())),
                    }
                }
                OpCode::Equal => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    self.stack.push(Value::Boolean(self.values_equal(&a, &b)));
                }
                OpCode::Greater => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a > b));
                        }
                        _ => return Err(RuntimeError::TypeError("Invalid operands for >".to_string())),
                    }
                }
                OpCode::Less => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a < b));
                        }
                        _ => return Err(RuntimeError::TypeError("Invalid operands for <".to_string())),
                    }
                }
                OpCode::Not => {
                    let value = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    self.stack.push(Value::Boolean(!value.is_truthy()));
                }
                OpCode::DefineGlobal(name_index) => {
                    let frame = self.frames.last().unwrap();
                    let name = match &frame.function.chunk.constants[name_index] {
                        Value::String(s) => s.clone(),
                        _ => return Err(RuntimeError::TypeError("Variable name must be string".to_string())),
                    };
                    
                    let value = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    self.globals.insert(name, value);
                }
                OpCode::GetGlobal(name_index) => {
                    let frame = self.frames.last().unwrap();
                    let name = match &frame.function.chunk.constants[name_index] {
                        Value::String(s) => s.clone(),
                        _ => return Err(RuntimeError::TypeError("Variable name must be string".to_string())),
                    };
                    
                    let value = self.globals.get(&name)
                        .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?
                        .clone();
                    
                    self.stack.push(value);
                }
                OpCode::SetGlobal(name_index) => {
                    let frame = self.frames.last().unwrap();
                    let name = match &frame.function.chunk.constants[name_index] {
                        Value::String(s) => s.clone(),
                        _ => return Err(RuntimeError::TypeError("Variable name must be string".to_string())),
                    };
                    
                    if !self.globals.contains_key(&name) {
                        return Err(RuntimeError::UndefinedVariable(name));
                    }
                    
                    let value = self.stack.last().unwrap().clone(); // Don't pop for assignment
                    self.globals.insert(name, value);
                }
                OpCode::GetLocal(slot) => {
                    let frame = self.frames.last().unwrap();
                    let index = frame.stack_offset + slot;
                    let value = self.stack[index].clone();
                    self.stack.push(value);
                }
                OpCode::SetLocal(slot) => {
                    let frame = self.frames.last().unwrap();
                    let index = frame.stack_offset + slot;
                    let value = self.stack.last().unwrap().clone(); // Don't pop for assignment
                    self.stack[index] = value;
                }
                OpCode::Jump(offset) => {
                    self.frames.last_mut().unwrap().ip += offset;
                }
                OpCode::JumpIfFalse(offset) => {
                    let value = self.stack.last().unwrap();
                    if !value.is_truthy() {
                        self.frames.last_mut().unwrap().ip += offset;
                    }
                }
                OpCode::Loop(offset) => {
                    self.frames.last_mut().unwrap().ip -= offset;
                }
                OpCode::Call(arg_count) => {
                    let callee = self.stack[self.stack.len() - arg_count - 1].clone();
                    
                    match callee {
                        Value::Function(function) => {
                            if arg_count != function.arity {
                                return Err(RuntimeError::ArityMismatch {
                                    expected: function.arity,
                                    got: arg_count,
                                });
                            }
                            
                            let stack_offset = self.stack.len() - arg_count - 1;
                            self.frames.push(CallFrame {
                                function,
                                ip: 0,
                                stack_offset,
                            });
                        }
                        _ => return Err(RuntimeError::UndefinedFunction("Not a function".to_string())),
                    }
                }
                OpCode::Return => {
                    let result = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let frame = self.frames.pop().unwrap();
                    
                    // Clean up the stack (remove locals and function)
                    self.stack.truncate(frame.stack_offset);
                    
                    // Push the return value
                    self.stack.push(result);
                }
                OpCode::Print => {
                    let value = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    println!("{}", self.value_to_string(&value));
                }
            }
        }
        
        Ok(())
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
    
    pub fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
            Value::Function(f) => format!("<fn {}>", f.name),
        }
    }
}
