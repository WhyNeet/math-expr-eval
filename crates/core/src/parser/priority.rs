use std::{cmp::Ordering, mem};

use crate::tokenizer::token::{Operation, Token};

use super::utils::is_weak_operation;

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if mem::discriminant(self) == mem::discriminant(other)
            || is_weak_operation(self) && is_weak_operation(other)
        {
            Some(Ordering::Equal)
        } else if is_weak_operation(self) || self.is_sentinel() {
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

        let other_num = match other {
            Token::Number(val) => *val,
            _ => 0f64,
        };

        let other_op = match other {
            Token::Operation(val) => *val,
            _ => Operation::Unknown('\0'),
        };

        match self {
            Token::Number(num) => num.partial_cmp(&other_num),
            Token::Operation(op) => op.partial_cmp(&other_op),
            _ => None,
        }
    }
}
