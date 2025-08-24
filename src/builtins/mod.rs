//! Built-in functions for the Jing language.
//!
//! This module contains implementations of built-in functions using
//! the modular trait system. Adding new builtins is as simple as
//! implementing the BuiltinFunction trait and registering it.

pub mod core;
pub mod http;
pub mod io;
pub mod math;
pub mod string;

use crate::registry::register_builtin;
use std::sync::Arc;

/// Initialize all built-in functions
pub fn init_builtins() {
    // Core functions
    register_builtin(Arc::new(core::PrintFunction));
    register_builtin(Arc::new(core::TypeFunction));

    // Math functions
    register_builtin(Arc::new(math::SqrtFunction));
    register_builtin(Arc::new(math::AbsFunction));
    register_builtin(Arc::new(math::MaxFunction));
    register_builtin(Arc::new(math::MinFunction));

    // String functions
    register_builtin(Arc::new(string::LenFunction));
    register_builtin(Arc::new(string::UpperFunction));
    register_builtin(Arc::new(string::LowerFunction));
    register_builtin(Arc::new(string::ReverseFunction));

    // I/O functions
    register_builtin(Arc::new(io::ReadLineFunction));
    register_builtin(Arc::new(io::InputFunction));
    register_builtin(Arc::new(io::ReadFileFunction));
    register_builtin(Arc::new(io::WriteFileFunction));
    register_builtin(Arc::new(io::FileExistsFunction));

    // HTTP server functions
    register_builtin(Arc::new(http::StartHttpServerFunction));
    register_builtin(Arc::new(http::StopHttpServerFunction));
    register_builtin(Arc::new(http::HttpResponseFunction));
    register_builtin(Arc::new(http::ListHttpServersFunction));
    register_builtin(Arc::new(http::HttpRegisterHandlerFunction));
}

/// Get all registered builtin function names
pub fn get_builtin_names() -> Vec<String> {
    crate::registry::builtin_names()
}

/// Check if a function name is a builtin
pub fn is_builtin(name: &str) -> bool {
    crate::registry::get_builtin(name).is_some()
}

/// Call a builtin function
pub fn call_builtin(
    name: &str,
    args: Vec<crate::value::Value>,
) -> crate::error::JingResult<crate::value::Value> {
    match crate::registry::get_builtin(name) {
        Some(builtin) => {
            if args.len() != builtin.arity() {
                return Err(crate::error::JingError::runtime_error(format!(
                    "Function '{}' expects {} arguments, got {}",
                    name,
                    builtin.arity(),
                    args.len()
                )));
            }
            builtin.call(args)
        }
        None => Err(crate::error::JingError::runtime_error(format!(
            "Unknown builtin function: {}",
            name
        ))),
    }
}
