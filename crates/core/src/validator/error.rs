use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("invalid parentheses")]
    InvalidParentheses,

    #[error("invalid operation: \"{0}\"")]
    InvalidOperation(char),

    #[error("syntax error")]
    SyntaxError,
}
