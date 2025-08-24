use crate::compiler::{Chunk, OpCode};
use crate::error::{JingError, JingResult};
use crate::value::{Environment, Value};

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    #[allow(dead_code)]
    function_name: String,
    return_address: usize,
    stack_base: usize,
}

/// Virtual Machine for executing Jing bytecode
pub struct VM {
    chunk: Chunk,
    ip: usize,         // Instruction pointer
    stack: Vec<Value>, // Value stack
    globals: Environment,
    call_stack: Vec<CallFrame>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
            globals: Environment::new(),
            call_stack: Vec::new(),
        }
    }

    /// Load and execute a chunk of bytecode
    pub fn interpret(&mut self, chunk: Chunk) -> JingResult<()> {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    /// Main execution loop
    fn run(&mut self) -> JingResult<()> {
        loop {
            if self.ip >= self.chunk.code.len() {
                break;
            }

            let instruction = self.chunk.code[self.ip].clone();
            self.ip += 1;

            match instruction {
                OpCode::Constant(index) => {
                    if index < self.chunk.constants.len() {
                        let value = self.chunk.constants[index].clone();
                        self.push(value);
                    } else {
                        return Err(JingError::runtime_error("Invalid constant index"));
                    }
                }

                OpCode::Load(name) => {
                    // First try to load from globals (variables)
                    if let Ok(value) = self.globals.get(&name) {
                        self.push(value);
                    } else if let Some(func_info) = self.chunk.functions.get(&name) {
                        // If not found in globals, try to load as a function
                        let function_value = Value::Function {
                            name: func_info.name.clone(),
                            arity: func_info.arity,
                            chunk_start: func_info.start_address,
                        };
                        self.push(function_value);
                    } else if let Some(builtin) = crate::registry::get_builtin(&name) {
                        // Check for builtin functions
                        let builtin_value = Value::BuiltinFunction {
                            name: name.clone(),
                            function: builtin,
                        };
                        self.push(builtin_value);
                    } else {
                        return Err(JingError::runtime_error(format!(
                            "Undefined variable or function '{}'",
                            name
                        )));
                    }
                }

                OpCode::Store(name) => {
                    let value = self.pop()?;
                    self.globals.define(name, value);
                }

                OpCode::Pop => {
                    self.pop()?;
                }

                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = a.add(&b)?;
                    self.push(result);
                }

                OpCode::Subtract => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = a.subtract(&b)?;
                    self.push(result);
                }

                OpCode::Multiply => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = a.multiply(&b)?;
                    self.push(result);
                }

                OpCode::Divide => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = a.divide(&b)?;
                    self.push(result);
                }

                OpCode::Modulo => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = a.modulo(&b)?;
                    self.push(result);
                }

                OpCode::Negate => {
                    let a = self.pop()?;
                    let result = a.negate()?;
                    self.push(result);
                }

                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Bool(a.equals(&b));
                    self.push(result);
                }

                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Bool(!a.equals(&b));
                    self.push(result);
                }

                OpCode::Less => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Bool(a.less_than(&b)?);
                    self.push(result);
                }

                OpCode::LessEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Bool(a.less_than(&b)? || a.equals(&b));
                    self.push(result);
                }

                OpCode::Greater => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Bool(a.greater_than(&b)?);
                    self.push(result);
                }

                OpCode::GreaterEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Bool(a.greater_than(&b)? || a.equals(&b));
                    self.push(result);
                }

                OpCode::Not => {
                    let a = self.pop()?;
                    let result = a.not();
                    self.push(result);
                }

                OpCode::And => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = if a.is_truthy() { b } else { a };
                    self.push(result);
                }

                OpCode::Or => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = if a.is_truthy() { a } else { b };
                    self.push(result);
                }

                OpCode::Jump(address) => {
                    self.ip = address;
                }

                OpCode::JumpIfFalse(address) => {
                    let condition = self.peek()?;
                    if condition.is_falsy() {
                        self.ip = address;
                    }
                }

                OpCode::Call(arity) => {
                    self.call_function(arity)?;
                }

                OpCode::Return => {
                    if let Some(frame) = self.call_stack.pop() {
                        // Restore the previous call frame
                        let return_value = self.pop()?;

                        // Remove the function's local variables from the stack
                        self.stack.truncate(frame.stack_base);

                        // Push the return value
                        self.push(return_value);

                        // Return to the caller
                        self.ip = frame.return_address;
                    } else {
                        // Top-level return, halt execution
                        break;
                    }
                }

                OpCode::Print => {
                    let value = self.pop()?;
                    println!("{}", value);
                }

                OpCode::Halt => {
                    break;
                }
            }
        }

        Ok(())
    }

    fn call_function(&mut self, arity: usize) -> JingResult<()> {
        let function = self.peek_at(0)?; // Get function from top of stack

        match function {
            Value::Function {
                name,
                arity: expected_arity,
                chunk_start,
            } => {
                if arity != expected_arity {
                    return Err(JingError::runtime_error(format!(
                        "Function '{}' expects {} arguments, got {}",
                        name, expected_arity, arity
                    )));
                }

                // Get function info to access parameter names
                let func_info = self.chunk.functions.get(&name).cloned();
                if let Some(func_info) = func_info {
                    // Bind arguments to parameter names in global environment
                    let args = self.get_function_args(arity);
                    for (i, param_name) in func_info.locals.iter().enumerate() {
                        if i < arity {
                            self.globals.define(param_name.clone(), args[i].clone());
                        }
                    }
                }

                // Create a new call frame
                let frame = CallFrame {
                    function_name: name.clone(),
                    return_address: self.ip,
                    stack_base: self.stack.len() - arity - 1, // -1 for the function itself
                };

                self.call_stack.push(frame);

                // Jump to the function's code
                self.ip = chunk_start;

                // Remove the function and arguments from the stack
                // We'll keep the function result handling as is
                for _ in 0..=arity {
                    self.stack.pop();
                }
            }
            Value::BuiltinFunction { name, function } => {
                if arity != function.arity() {
                    return Err(JingError::runtime_error(format!(
                        "Builtin function '{}' expects {} arguments, got {}",
                        name,
                        function.arity(),
                        arity
                    )));
                }

                // Collect arguments from the stack using helper method
                let args = self.get_function_args(arity);

                // Call the builtin function
                let result = function.call(args)?;

                // Remove the function and arguments from the stack
                for _ in 0..=arity {
                    self.stack.pop();
                }

                // Push the result
                self.push(result);
            }
            _ => {
                return Err(JingError::runtime_error("Can only call functions"));
            }
        }

        Ok(())
    }

    /// Extract function arguments from the stack
    /// Arguments are arranged as: [..., arg0, arg1, ..., argN, function]
    fn get_function_args(&self, arity: usize) -> Vec<Value> {
        let stack_len = self.stack.len();
        let mut args = Vec::with_capacity(arity);

        for i in 0..arity {
            let arg_index = stack_len - arity - 1 + i; // -1 for function itself
            args.push(self.stack[arg_index].clone());
        }

        args
    }

    /// Push a value onto the stack
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Pop a value from the stack
    fn pop(&mut self) -> JingResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| JingError::runtime_error("Stack underflow"))
    }

    /// Peek at the top of the stack without popping
    fn peek(&self) -> JingResult<Value> {
        self.stack
            .last()
            .cloned()
            .ok_or_else(|| JingError::runtime_error("Empty stack"))
    }

    /// Peek at a value at a given distance from the top of the stack
    fn peek_at(&self, distance: usize) -> JingResult<Value> {
        if distance >= self.stack.len() {
            return Err(JingError::runtime_error("Stack index out of bounds"));
        }

        let index = self.stack.len() - 1 - distance;
        Ok(self.stack[index].clone())
    }

    /// Get the current stack for debugging
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    /// Get a global variable by name
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.globals.get(name).ok()
    }

    /// Get the top value from the stack (result of last expression)
    pub fn get_result(&self) -> JingResult<Value> {
        if self.stack.is_empty() {
            Ok(Value::Nil)
        } else {
            Ok(self.stack[self.stack.len() - 1].clone())
        }
    }

    /// Reset the VM state
    pub fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.globals = Environment::new();
        self.call_stack.clear();
    }
}

/// REPL (Read-Eval-Print Loop) for interactive Jing sessions
pub struct REPL {
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        REPL { vm: VM::new() }
    }

    /// Evaluate a single line of Jing code
    pub fn eval(&mut self, source: &str) -> JingResult<()> {
        use crate::compiler::Compiler;
        use crate::lexer::Lexer;
        use crate::parser::Parser;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(statements)?;

        self.vm.interpret(chunk)
    }

    /// Start an interactive REPL session
    pub fn run(&mut self) -> JingResult<()> {
        use std::io::{self, Write};

        println!("Jing REPL v0.1.0");
        println!("Type 'exit' to quit.");
        println!();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();

                    if input.is_empty() {
                        continue;
                    }

                    if input == "exit" || input == "quit" {
                        break;
                    }

                    match self.eval(input) {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("Error: {}", err);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                    break;
                }
            }
        }

        println!("Goodbye!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn run_code(source: &str) -> JingResult<VM> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(statements)?;

        let mut vm = VM::new();
        vm.interpret(chunk)?;

        Ok(vm)
    }

    #[test]
    fn test_simple_arithmetic() {
        let vm = run_code("let result = 10 + 5;").unwrap();

        let result = vm.globals.get("result").unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 15.0),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_variables() {
        let vm = run_code(
            r"
            let x = 42;
            let y = x + 8;
        ",
        )
        .unwrap();

        let y = vm.globals.get("y").unwrap();
        match y {
            Value::Number(n) => assert_eq!(n, 50.0),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_string_concatenation() {
        let vm = run_code(
            r#"
            let greeting = "Hello, " + "World!";
        "#,
        )
        .unwrap();

        let greeting = vm.globals.get("greeting").unwrap();
        match greeting {
            Value::String(s) => assert_eq!(s, "Hello, World!"),
            _ => panic!("Expected string result"),
        }
    }
}
