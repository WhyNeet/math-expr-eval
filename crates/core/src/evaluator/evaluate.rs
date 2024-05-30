use crate::{
    parser::tree::{Node, NodeValue},
    tokenizer::token::Operation,
};

pub fn evaluate(tree: &Node<dyn NodeValue>) -> f64 {
    let is_number = tree.value().get_number().is_some();
    if is_number {
        return tree.value().get_number().unwrap();
    }

    match tree.value().get_operation().unwrap() {
        Operation::Add => evaluate(tree.left().unwrap()) + evaluate(tree.right().unwrap()),
        Operation::Subtract => evaluate(tree.left().unwrap()) - evaluate(tree.right().unwrap()),
        Operation::Multiply => evaluate(tree.left().unwrap()) * evaluate(tree.right().unwrap()),
        Operation::Divide => evaluate(tree.left().unwrap()) / evaluate(tree.right().unwrap()),
        Operation::Unknown(op) => panic!("unknown operation: {op}"),
        Operation::Sentinel => unreachable!(),
    }
}
