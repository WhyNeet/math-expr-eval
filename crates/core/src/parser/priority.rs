use std::{cmp::Ordering, mem};

use crate::tokenizer::token::{Operation, Token};

use super::utils::is_weak_operation;

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !self.is_sentinel() && other.is_sentinel() {
            return Some(Ordering::Less);
        }

        if mem::discriminant(self) == mem::discriminant(other)
            || is_weak_operation(self) && is_weak_operation(other)
        {
            Some(Ordering::Equal)
        } else if is_weak_operation(self) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if mem::discriminant(self) != mem::discriminant(other) {
            return None;
        }

        match self {
            Token::Number(num) => num.partial_cmp(&other.as_number().unwrap()),
            Token::Operation(op) => op.partial_cmp(&other.as_operation().unwrap()),
            _ => None,
        }
    }
}
