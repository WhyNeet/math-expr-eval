use crate::{
    parser::tree::{Node, NodeValue},
    tokenizer::token::Operation,
};

pub fn evaluate(tree: &Node<dyn NodeValue>) -> f64 {
    let is_number = tree.value().as_number().is_some();
    if is_number {
        return tree.value().as_number().unwrap();
    }

    let res = match tree.value().as_operation().unwrap() {
        Operation::Add => evaluate(tree.left().unwrap()) + evaluate(tree.right().unwrap()),
        Operation::Subtract => evaluate(tree.left().unwrap()) - evaluate(tree.right().unwrap()),
        Operation::Multiply => evaluate(tree.left().unwrap()) * evaluate(tree.right().unwrap()),
        Operation::Divide => evaluate(tree.left().unwrap()) / evaluate(tree.right().unwrap()),
        _ => unreachable!(),
    };

    res
}
