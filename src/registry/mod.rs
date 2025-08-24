//! Central registry system for managing language extensions.
//! 
//! This module provides a simple registry for builtin functions
//! that allows easy extension without modifying core files.

use crate::features::BuiltinFunction;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple global storage for builtin functions
static BUILTIN_FUNCTIONS: Mutex<Option<HashMap<String, Arc<dyn BuiltinFunction>>>> = Mutex::new(None);

/// Initialize the builtin functions storage
fn init_builtins_storage() {
    let mut storage = BUILTIN_FUNCTIONS.lock().unwrap();
    if storage.is_none() {
        *storage = Some(HashMap::new());
    }
}

/// Register a builtin function globally
pub fn register_builtin(function: Arc<dyn BuiltinFunction>) {
    init_builtins_storage();
    let mut storage = BUILTIN_FUNCTIONS.lock().unwrap();
    let functions = storage.as_mut().unwrap();
    functions.insert(function.name().to_string(), function);
}

/// Get a builtin function by name
pub fn get_builtin(name: &str) -> Option<Arc<dyn BuiltinFunction>> {
    init_builtins_storage();
    let storage = BUILTIN_FUNCTIONS.lock().unwrap();
    let functions = storage.as_ref().unwrap();
    functions.get(name).cloned()
}

/// Get all builtin function names
pub fn builtin_names() -> Vec<String> {
    init_builtins_storage();
    let storage = BUILTIN_FUNCTIONS.lock().unwrap();
    let functions = storage.as_ref().unwrap();
    functions.keys().cloned().collect()
}
