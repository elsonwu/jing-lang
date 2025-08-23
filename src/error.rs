use std::fmt;

/// Error types that can occur during Jing execution
#[derive(Debug, Clone, PartialEq)]
pub enum JingError {
    /// Lexical analysis errors
    LexError { message: String, line: usize },
    /// Parsing errors
    ParseError { message: String, line: usize },
    /// Compilation errors
    CompileError { message: String },
    /// Runtime errors
    RuntimeError { message: String },
    /// Type errors
    TypeError { message: String },
    /// I/O errors
    IoError { message: String },
}

impl fmt::Display for JingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JingError::LexError { message, line } => {
                write!(f, "Lexical error at line {}: {}", line, message)
            }
            JingError::ParseError { message, line } => {
                write!(f, "Parse error at line {}: {}", line, message)
            }
            JingError::CompileError { message } => {
                write!(f, "Compilation error: {}", message)
            }
            JingError::RuntimeError { message } => {
                write!(f, "Runtime error: {}", message)
            }
            JingError::TypeError { message } => {
                write!(f, "Type error: {}", message)
            }
            JingError::IoError { message } => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl std::error::Error for JingError {}

/// Result type for Jing operations
pub type JingResult<T> = Result<T, JingError>;

/// Helper functions for creating specific error types
impl JingError {
    pub fn lex_error(message: impl Into<String>, line: usize) -> Self {
        JingError::LexError {
            message: message.into(),
            line,
        }
    }

    pub fn parse_error(message: impl Into<String>, line: usize) -> Self {
        JingError::ParseError {
            message: message.into(),
            line,
        }
    }

    pub fn compile_error(message: impl Into<String>) -> Self {
        JingError::CompileError {
            message: message.into(),
        }
    }

    pub fn runtime_error(message: impl Into<String>) -> Self {
        JingError::RuntimeError {
            message: message.into(),
        }
    }

    pub fn type_error(message: impl Into<String>) -> Self {
        JingError::TypeError {
            message: message.into(),
        }
    }

    pub fn io_error(message: impl Into<String>) -> Self {
        JingError::IoError {
            message: message.into(),
        }
    }
}
