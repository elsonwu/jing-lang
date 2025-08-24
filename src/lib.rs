//! # Jing Programming Language
//!
//! A simple toy programming language implemented in Rust with a modular,
//! extensible architecture. The language supports:
//!
//! - Variables and assignment expressions
//! - Arithmetic, comparison, and logical operations  
//! - Control flow (if/else, while loops)
//! - Functions with parameters and return values
//! - Built-in functions (print, math, string, I/O operations)
//! - A REPL for interactive development
//!
//! ## Modular Architecture
//!
//! The language is built with a plugin-like architecture that allows
//! easy extension without modifying core files:
//!
//! - **Features**: Trait-based system for language features
//! - **Registry**: Central registration system for all extensions  
//! - **Builtins**: Modular builtin function system
//! - **Operators**: Pluggable operator system
//!
//! ## Adding New Features
//!
//! To add a new builtin function:
//! 1. Implement the `BuiltinFunction` trait
//! 2. Register it using `register_builtin!` or in an init function
//! 3. Done! No need to touch existing files
//!
//! To add a new operator:
//! 1. Implement the `Operator` trait  
//! 2. Register it using `register_operator!`
//! 3. Done!
//!
//! ## Example Usage
//!
//! ```rust
//! use jing::{Lexer, Parser, Compiler, VM};
//!
//! // Initialize the language extensions
//! jing::init();
//!
//! let input = "let x = 42; print(x + 8);";
//!
//! let mut lexer = Lexer::new(input);
//! let tokens = lexer.tokenize().unwrap();
//!
//! let mut parser = Parser::new(tokens);
//! let statements = parser.parse().unwrap();
//!
//! let mut compiler = Compiler::new();
//! let chunk = compiler.compile(statements).unwrap();
//!
//! let mut vm = VM::new();
//! vm.interpret(chunk).unwrap();
//! ```

// Core modules
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod value;
pub mod vm;

// Modular extension system
pub mod builtins;
pub mod features;
pub mod registry;

// Public re-exports for easy access
pub use compiler::{Chunk, Compiler, OpCode};
pub use error::{JingError, JingResult};
pub use lexer::{Lexer, Token};
pub use parser::{Expr, Parser, Stmt};
pub use value::{Environment, Value};
pub use vm::VM;

// Feature system
pub use features::BuiltinFunction;

/// Initialize the Jing language with all built-in features.
///
/// This function registers all built-in functions and other
/// language features. Call this before using the language.
pub fn init() {
    builtins::init_builtins();
}
