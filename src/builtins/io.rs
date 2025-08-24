//! I/O built-in functions

use crate::error::{JingError, JingResult};
use crate::features::BuiltinFunction;
use crate::value::Value;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Read a line from standard input
#[derive(Debug)]
pub struct ReadLineFunction;

impl BuiltinFunction for ReadLineFunction {
    fn name(&self) -> &str {
        "readline"
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if !args.is_empty() {
            return Err(JingError::runtime_error("readline() takes no arguments"));
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Remove trailing newline
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }
                Ok(Value::String(input))
            }
            Err(e) => Err(JingError::runtime_error(format!(
                "Failed to read input: {}",
                e
            ))),
        }
    }

    fn help(&self) -> &str {
        "readline() - Read a line from standard input"
    }
}

/// Read a line with a prompt
#[derive(Debug)]
pub struct InputFunction;

impl BuiltinFunction for InputFunction {
    fn name(&self) -> &str {
        "input"
    }

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(prompt) => {
                print!("{}", prompt);
                io::stdout().flush().map_err(|e| {
                    JingError::runtime_error(format!("Failed to flush output: {}", e))
                })?;

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        // Remove trailing newline
                        if input.ends_with('\n') {
                            input.pop();
                            if input.ends_with('\r') {
                                input.pop();
                            }
                        }
                        Ok(Value::String(input))
                    }
                    Err(e) => Err(JingError::runtime_error(format!(
                        "Failed to read input: {}",
                        e
                    ))),
                }
            }
            _ => Err(JingError::TypeError {
                message: "input() expects a string prompt".to_string(),
            }),
        }
    }

    fn help(&self) -> &str {
        "input(prompt) - Display prompt and read a line from standard input"
    }
}

/// Read entire file contents as string
#[derive(Debug)]
pub struct ReadFileFunction;

impl BuiltinFunction for ReadFileFunction {
    fn name(&self) -> &str {
        "read_file"
    }

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(file_path) => match fs::read_to_string(file_path) {
                Ok(contents) => Ok(Value::String(contents)),
                Err(e) => Err(JingError::runtime_error(format!(
                    "Failed to read file '{}': {}",
                    file_path, e
                ))),
            },
            _ => Err(JingError::TypeError {
                message: "read_file() expects a string file path".to_string(),
            }),
        }
    }

    fn help(&self) -> &str {
        "read_file(path) - Read entire file contents as string"
    }
}

/// Write string content to file
#[derive(Debug)]
pub struct WriteFileFunction;

impl BuiltinFunction for WriteFileFunction {
    fn name(&self) -> &str {
        "write_file"
    }

    fn arity(&self) -> usize {
        2
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match (&args[0], &args[1]) {
            (Value::String(file_path), Value::String(content)) => {
                match fs::write(file_path, content) {
                    Ok(_) => Ok(Value::Nil),
                    Err(e) => Err(JingError::runtime_error(format!(
                        "Failed to write to file '{}': {}",
                        file_path, e
                    ))),
                }
            }
            _ => Err(JingError::TypeError {
                message: "write_file() expects (file_path: string, content: string)".to_string(),
            }),
        }
    }

    fn help(&self) -> &str {
        "write_file(path, content) - Write string content to file"
    }
}

/// Check if file exists
#[derive(Debug)]
pub struct FileExistsFunction;

impl BuiltinFunction for FileExistsFunction {
    fn name(&self) -> &str {
        "file_exists"
    }

    fn arity(&self) -> usize {
        1
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        match &args[0] {
            Value::String(file_path) => {
                let path = Path::new(file_path);
                Ok(Value::Bool(path.exists()))
            }
            _ => Err(JingError::TypeError {
                message: "file_exists() expects a string file path".to_string(),
            }),
        }
    }

    fn help(&self) -> &str {
        "file_exists(path) - Check if file or directory exists"
    }
}
