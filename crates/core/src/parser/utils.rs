use std::mem;

use crate::tokenizer::token::Operation;

pub fn is_weak_operation(operation: &Operation) -> bool {
    mem::discriminant(operation) == mem::discriminant(&Operation::Add)
        || mem::discriminant(operation) == mem::discriminant(&Operation::Subtract)
}
