use std::fmt;

/// Error types that can occur during JiLang execution
#[derive(Debug, Clone, PartialEq)]
pub enum JiLangError {
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

impl fmt::Display for JiLangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JiLangError::LexError { message, line } => {
                write!(f, "Lexical error at line {}: {}", line, message)
            }
            JiLangError::ParseError { message, line } => {
                write!(f, "Parse error at line {}: {}", line, message)
            }
            JiLangError::CompileError { message } => {
                write!(f, "Compilation error: {}", message)
            }
            JiLangError::RuntimeError { message } => {
                write!(f, "Runtime error: {}", message)
            }
            JiLangError::TypeError { message } => {
                write!(f, "Type error: {}", message)
            }
            JiLangError::IoError { message } => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl std::error::Error for JiLangError {}

/// Result type for JiLang operations
pub type JiResult<T> = Result<T, JiLangError>;

/// Helper functions for creating specific error types
impl JiLangError {
    pub fn lex_error(message: impl Into<String>, line: usize) -> Self {
        JiLangError::LexError {
            message: message.into(),
            line,
        }
    }

    pub fn parse_error(message: impl Into<String>, line: usize) -> Self {
        JiLangError::ParseError {
            message: message.into(),
            line,
        }
    }

    pub fn compile_error(message: impl Into<String>) -> Self {
        JiLangError::CompileError {
            message: message.into(),
        }
    }

    pub fn runtime_error(message: impl Into<String>) -> Self {
        JiLangError::RuntimeError {
            message: message.into(),
        }
    }

    pub fn type_error(message: impl Into<String>) -> Self {
        JiLangError::TypeError {
            message: message.into(),
        }
    }

    pub fn io_error(message: impl Into<String>) -> Self {
        JiLangError::IoError {
            message: message.into(),
        }
    }
}
