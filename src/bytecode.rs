#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Stack operations
    Constant(usize),     // Push constant from constant pool
    Pop,                 // Pop top value from stack
    
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    
    // Comparison
    Equal,
    Greater,
    Less,
    
    // Logical
    Not,
    
    // Variables
    DefineGlobal(usize), // Define global variable (name from constant pool)
    GetGlobal(usize),    // Get global variable (name from constant pool)
    SetGlobal(usize),    // Set global variable (name from constant pool)
    GetLocal(usize),     // Get local variable by slot
    SetLocal(usize),     // Set local variable by slot
    
    // Control flow
    Jump(usize),         // Unconditional jump
    JumpIfFalse(usize),  // Jump if top of stack is false
    Loop(usize),         // Jump backward (for loops)
    
    // Functions
    Call(usize),         // Call function with n arguments
    Return,              // Return from function
    
    // Built-ins
    Print,               // Print top of stack
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }
    
    pub fn write(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode);
        self.lines.push(line);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
}

impl Function {
    pub fn new(name: String, arity: usize) -> Self {
        Self {
            name,
            arity,
            chunk: Chunk::new(),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
    
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Nil => "nil",
            Value::Function(_) => "function",
        }
    }
}
