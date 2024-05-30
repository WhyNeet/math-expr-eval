use super::tree::{Node, NodeValue};
use crate::tokenizer::token::{Operation, Parenthesis, Token};

pub fn parse(tokens: &[Token]) -> Node<dyn NodeValue> {
    if tokens.len() == 1 {
        return Node::new(Box::new(tokens.first().unwrap().as_number().unwrap()));
    }

    // parentheses counter
    // increment on every "("
    // decrement on every ")"
    let mut par_count = 0;

    let (operation, idx) = tokens
        .iter()
        .enumerate()
        .filter(|(_, token)| token.as_number().is_none())
        .rev()
        .fold((Operation::Sentinel, 0), |acc, (idx, token)| {
            let (acc, acc_idx) = acc;

            if let Some(par) = token.as_parenthesis() {
                match par {
                    Parenthesis::Left => par_count += 1,
                    Parenthesis::Right => par_count -= 1,
                    Parenthesis::Unknown(_) => unreachable!(),
                }

                return (acc, acc_idx);
            }

            let token = token.as_operation().unwrap();

            if par_count == 0 && (acc.is_sentinel() || token < acc) {
                return (token, idx);
            }

            (acc, acc_idx)
        });

    // if the expression is surrounded by parentheses
    if matches!(operation, Operation::Sentinel) {
        return parse(&tokens[1..(tokens.len() - 1)]);
    }

    let mut root = Node::<dyn NodeValue>::new(Box::new(operation));
    let left = parse(&tokens[..idx]);
    let right = parse(&tokens[(idx + 1)..]);

    root.set_left(Some(Box::new(left)));
    root.set_right(Some(Box::new(right)));

    root
}
