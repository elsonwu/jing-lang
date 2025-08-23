use crate::error::{JingError, JingResult};
use crate::parser::*;
use crate::value::Value;
use std::collections::HashMap;

/// Bytecode instructions for the Jing VM
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    /// Push a constant onto the stack
    Constant(usize),
    /// Load a variable onto the stack
    Load(String),
    /// Store top of stack to a variable
    Store(String),
    /// Pop top of stack
    Pop,

    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,

    // Comparison operations
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),

    // Function calls
    Call(usize), // arity
    Return,

    // Built-in functions
    Print,

    // Program control
    Halt,
}

/// A compiled chunk of bytecode
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub functions: HashMap<String, FunctionInfo>,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub arity: usize,
    pub start_address: usize,
    pub locals: Vec<String>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            functions: HashMap::new(),
        }
    }

    pub fn emit(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn emit_constant(&mut self, value: Value) -> usize {
        let index = self.constants.len();
        self.constants.push(value);
        self.emit(OpCode::Constant(index));
        index
    }

    pub fn current_address(&self) -> usize {
        self.code.len()
    }

    pub fn patch_jump(&mut self, address: usize, target: usize) {
        match &mut self.code[address] {
            OpCode::Jump(addr) | OpCode::JumpIfFalse(addr) => {
                *addr = target;
            }
            _ => panic!("Cannot patch non-jump instruction"),
        }
    }
}

/// Compiler that converts AST to bytecode
pub struct Compiler {
    chunk: Chunk,
    locals: Vec<String>,
    scope_depth: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
        }
    }

    /// Compile a list of statements to bytecode
    pub fn compile(&mut self, statements: Vec<Stmt>) -> JingResult<Chunk> {
        for stmt in statements {
            self.compile_statement(stmt)?;
        }

        self.chunk.emit(OpCode::Halt);
        Ok(std::mem::replace(&mut self.chunk, Chunk::new()))
    }

    fn compile_statement(&mut self, stmt: Stmt) -> JingResult<()> {
        match stmt {
            Stmt::Expression(expr_stmt) => {
                self.compile_expression(expr_stmt.expr)?;
                self.chunk.emit(OpCode::Pop);
            }
            Stmt::Let(let_stmt) => {
                self.compile_expression(let_stmt.initializer)?;
                self.chunk.emit(OpCode::Store(let_stmt.name.clone()));

                // Track local variables
                if !self.locals.contains(&let_stmt.name) {
                    self.locals.push(let_stmt.name);
                }
            }
            Stmt::Print(print_stmt) => {
                self.compile_expression(print_stmt.expr)?;
                self.chunk.emit(OpCode::Print);
            }
            Stmt::Block(block_stmt) => {
                self.begin_scope();
                for stmt in block_stmt.statements {
                    self.compile_statement(stmt)?;
                }
                self.end_scope();
            }
            Stmt::If(if_stmt) => {
                self.compile_if_statement(if_stmt)?;
            }
            Stmt::While(while_stmt) => {
                self.compile_while_statement(while_stmt)?;
            }
            Stmt::Function(func_stmt) => {
                self.compile_function_declaration(func_stmt)?;
            }
            Stmt::Return(return_stmt) => {
                if let Some(value) = return_stmt.value {
                    self.compile_expression(value)?;
                } else {
                    self.chunk.emit_constant(Value::Nil);
                }
                self.chunk.emit(OpCode::Return);
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: Expr) -> JingResult<()> {
        match expr {
            Expr::Literal(literal) => {
                let value = match literal.value {
                    LiteralValue::Number(n) => Value::Number(n),
                    LiteralValue::String(s) => Value::String(s),
                    LiteralValue::Bool(b) => Value::Bool(b),
                    LiteralValue::Nil => Value::Nil,
                };
                self.chunk.emit_constant(value);
            }
            Expr::Variable(var) => {
                self.chunk.emit(OpCode::Load(var.name));
            }
            Expr::Binary(binary) => {
                self.compile_binary_expression(binary)?;
            }
            Expr::Unary(unary) => {
                self.compile_unary_expression(unary)?;
            }
            Expr::Logical(logical) => {
                self.compile_logical_expression(logical)?;
            }
            Expr::Call(call) => {
                self.compile_call_expression(call)?;
            }
        }
        Ok(())
    }

    fn compile_binary_expression(&mut self, binary: BinaryExpr) -> JingResult<()> {
        self.compile_expression(*binary.left)?;
        self.compile_expression(*binary.right)?;

        match binary.operator {
            BinaryOperator::Add => self.chunk.emit(OpCode::Add),
            BinaryOperator::Subtract => self.chunk.emit(OpCode::Subtract),
            BinaryOperator::Multiply => self.chunk.emit(OpCode::Multiply),
            BinaryOperator::Divide => self.chunk.emit(OpCode::Divide),
            BinaryOperator::Modulo => self.chunk.emit(OpCode::Modulo),
            BinaryOperator::Equal => self.chunk.emit(OpCode::Equal),
            BinaryOperator::NotEqual => self.chunk.emit(OpCode::NotEqual),
            BinaryOperator::Less => self.chunk.emit(OpCode::Less),
            BinaryOperator::LessEqual => self.chunk.emit(OpCode::LessEqual),
            BinaryOperator::Greater => self.chunk.emit(OpCode::Greater),
            BinaryOperator::GreaterEqual => self.chunk.emit(OpCode::GreaterEqual),
        }
        Ok(())
    }

    fn compile_unary_expression(&mut self, unary: UnaryExpr) -> JingResult<()> {
        self.compile_expression(*unary.operand)?;

        match unary.operator {
            UnaryOperator::Minus => self.chunk.emit(OpCode::Negate),
            UnaryOperator::Not => self.chunk.emit(OpCode::Not),
        }
        Ok(())
    }

    fn compile_logical_expression(&mut self, logical: LogicalExpr) -> JingResult<()> {
        match logical.operator {
            LogicalOperator::And => {
                self.compile_expression(*logical.left)?;
                let jump_address = self.chunk.current_address();
                self.chunk.emit(OpCode::JumpIfFalse(0)); // Will be patched

                self.chunk.emit(OpCode::Pop); // Pop the left operand if it's truthy
                self.compile_expression(*logical.right)?;

                let end_address = self.chunk.current_address();
                self.chunk.patch_jump(jump_address, end_address);
            }
            LogicalOperator::Or => {
                self.compile_expression(*logical.left)?;
                let jump_address = self.chunk.current_address();
                self.chunk.emit(OpCode::JumpIfFalse(0)); // Will be patched

                // If left is truthy, jump over the right operand
                let skip_right_jump = self.chunk.current_address();
                self.chunk.emit(OpCode::Jump(0)); // Will be patched

                let right_start = self.chunk.current_address();
                self.chunk.patch_jump(jump_address, right_start);

                self.chunk.emit(OpCode::Pop); // Pop the left operand if it's falsy
                self.compile_expression(*logical.right)?;

                let end_address = self.chunk.current_address();
                self.chunk.patch_jump(skip_right_jump, end_address);
            }
        }
        Ok(())
    }

    fn compile_call_expression(&mut self, call: CallExpr) -> JingResult<()> {
        // Handle built-in functions
        if let Expr::Variable(var) = call.callee.as_ref() {
            match var.name.as_str() {
                "print" => {
                    if call.args.len() != 1 {
                        return Err(JingError::compile_error(
                            "print() expects exactly 1 argument",
                        ));
                    }
                    self.compile_expression(call.args[0].clone())?;
                    self.chunk.emit(OpCode::Print);
                    return Ok(());
                }
                "len" => {
                    if call.args.len() != 1 {
                        return Err(JingError::compile_error("len() expects exactly 1 argument"));
                    }
                    self.compile_expression(call.args[0].clone())?;
                    // For now, we'll implement len as a simple operation
                    // In a real implementation, you'd add a LEN opcode
                    return Err(JingError::compile_error("len() not yet implemented"));
                }
                "str" => {
                    if call.args.len() != 1 {
                        return Err(JingError::compile_error("str() expects exactly 1 argument"));
                    }
                    self.compile_expression(call.args[0].clone())?;
                    // For now, we'll implement str as a simple operation
                    // In a real implementation, you'd add a STR opcode
                    return Err(JingError::compile_error("str() not yet implemented"));
                }
                _ => {}
            }
        }

        // Compile arguments
        for arg in call.args.iter() {
            self.compile_expression(arg.clone())?;
        }

        // Compile function call
        self.compile_expression(*call.callee)?;
        self.chunk.emit(OpCode::Call(call.args.len()));
        Ok(())
    }

    fn compile_if_statement(&mut self, if_stmt: IfStmt) -> JingResult<()> {
        self.compile_expression(if_stmt.condition)?;

        let then_jump = self.chunk.current_address();
        self.chunk.emit(OpCode::JumpIfFalse(0)); // Will be patched

        self.chunk.emit(OpCode::Pop); // Pop condition if true
        self.compile_statement(*if_stmt.then_branch)?;

        if let Some(else_branch) = if_stmt.else_branch {
            let else_jump = self.chunk.current_address();
            self.chunk.emit(OpCode::Jump(0)); // Will be patched

            let else_start = self.chunk.current_address();
            self.chunk.patch_jump(then_jump, else_start);

            self.chunk.emit(OpCode::Pop); // Pop condition if false
            self.compile_statement(*else_branch)?;

            let end_address = self.chunk.current_address();
            self.chunk.patch_jump(else_jump, end_address);
        } else {
            let end_address = self.chunk.current_address();
            self.chunk.patch_jump(then_jump, end_address);
            self.chunk.emit(OpCode::Pop); // Pop condition if false
        }

        Ok(())
    }

    fn compile_while_statement(&mut self, while_stmt: WhileStmt) -> JingResult<()> {
        let loop_start = self.chunk.current_address();

        self.compile_expression(while_stmt.condition)?;

        let exit_jump = self.chunk.current_address();
        self.chunk.emit(OpCode::JumpIfFalse(0)); // Will be patched

        self.chunk.emit(OpCode::Pop); // Pop condition if true
        self.compile_statement(*while_stmt.body)?;

        self.chunk.emit(OpCode::Jump(loop_start));

        let end_address = self.chunk.current_address();
        self.chunk.patch_jump(exit_jump, end_address);
        self.chunk.emit(OpCode::Pop); // Pop condition if false

        Ok(())
    }

    fn compile_function_declaration(&mut self, func_stmt: FunctionStmt) -> JingResult<()> {
        // Jump over the function body during normal execution
        let skip_jump = self.chunk.current_address();
        self.chunk.emit(OpCode::Jump(0)); // Will be patched

        let function_start = self.chunk.current_address();

        // Store function info
        let function_info = FunctionInfo {
            name: func_stmt.name.clone(),
            arity: func_stmt.params.len(),
            start_address: function_start,
            locals: func_stmt.params.clone(),
        };

        self.chunk
            .functions
            .insert(func_stmt.name.clone(), function_info);

        // Compile function body
        self.begin_scope();

        // Parameters are local variables
        for param in &func_stmt.params {
            self.locals.push(param.clone());
        }

        self.compile_statement(*func_stmt.body)?;

        // Implicit return nil if no explicit return
        self.chunk.emit_constant(Value::Nil);
        self.chunk.emit(OpCode::Return);

        self.end_scope();

        let function_end = self.chunk.current_address();
        self.chunk.patch_jump(skip_jump, function_end);

        // Define the function as a constant
        let func_value = Value::Function {
            name: func_stmt.name.clone(),
            arity: func_stmt.params.len(),
            chunk_start: function_start,
        };

        self.chunk.emit_constant(func_value);
        self.chunk.emit(OpCode::Store(func_stmt.name));

        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // Remove local variables from the current scope
        // In a more complete implementation, you'd track which variables
        // belong to which scope and only remove those from the current scope
        if !self.locals.is_empty() {
            // For now, we'll keep it simple and not remove locals
            // This is a placeholder for proper scope handling
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_compile_simple_expression() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(statements).unwrap();

        // Should have: CONSTANT(42), STORE(x), HALT
        assert!(chunk.code.len() >= 3);
        assert!(matches!(chunk.code[0], OpCode::Constant(0)));
        assert!(matches!(chunk.code[1], OpCode::Store(ref name) if name == "x"));
        assert!(matches!(chunk.code[chunk.code.len() - 1], OpCode::Halt));
    }

    #[test]
    fn test_compile_arithmetic() {
        let mut lexer = Lexer::new("let result = 10 + 5;");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(statements).unwrap();

        // Should compile to: CONSTANT(10), CONSTANT(5), ADD, STORE(result), HALT
        assert!(chunk.code.contains(&OpCode::Add));
    }
}
