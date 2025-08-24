//! Core trait definitions for the modular feature system.
//! 
//! This module defines the fundamental traits that enable a plugin-like
//! architecture for extending the Jing language with new features and
//! builtin functions without touching core implementation files.

use crate::error::JingResult;
use crate::value::Value;

/// Trait for builtin functions that can be dynamically registered
/// 
/// Implementing this trait allows you to add new builtin functions
/// without modifying the core VM or compiler.
pub trait BuiltinFunction: Send + Sync + std::fmt::Debug {
    /// Name of the function as it appears in Jing code
    fn name(&self) -> &str;
    
    /// Number of parameters this function expects
    fn arity(&self) -> usize;
    
    /// Execute the function with the given arguments
    fn call(&self, args: Vec<Value>) -> JingResult<Value>;
    
    /// Help text for the function (used in documentation/REPL help)
    fn help(&self) -> &str {
        "No help available"
    }
}
