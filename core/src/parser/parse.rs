use crate::tokenizer::token::{Operation, Token};

use super::tree::{Node, NodeValue};

pub fn parse(tokens: &[Token]) -> Node<dyn NodeValue> {
    if tokens.len() == 1 {
        return Node::new(Box::new(tokens.first().unwrap().get_number().unwrap()));
    }

    let (operation, idx) = tokens
        .iter()
        .map(|token| match token {
            Token::Operation(operation) => Some(*operation),
            _ => None,
        })
        .enumerate()
        .filter(|(_, token)| token.is_some())
        .map(|(idx, token)| (idx, token.unwrap()))
        .fold((Operation::Sentinel, 0), |acc, (idx, token)| {
            let (acc, acc_idx) = acc;

            if acc.is_sentinel() || token < acc {
                return (token, idx);
            }

            (acc, acc_idx)
        });

    let mut root = Node::<dyn NodeValue>::new(Box::new(operation));
    let left = parse(&tokens[..idx]);
    let right = parse(&tokens[(idx + 1)..]);

    root.set_left(Some(Box::new(left)));
    root.set_right(Some(Box::new(right)));

    root
}
