use crate::tokenizer::token::Operation;

#[derive(Debug)]
pub struct Node<V: NodeValue + ?Sized> {
    value: Box<V>,
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
}

impl<V: NodeValue + ?Sized> Node<V> {
    pub fn new(value: Box<V>) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn left(&self) -> Option<&Node<V>> {
        self.left.as_ref().map(|v| v.as_ref())
    }

    pub fn right(&self) -> Option<&Node<V>> {
        self.right.as_ref().map(|v| v.as_ref())
    }

    pub fn set_left(&mut self, node: Option<Box<Node<V>>>) {
        self.left = node;
    }

    pub fn set_right(&mut self, node: Option<Box<Node<V>>>) {
        self.right = node;
    }
}

pub trait NodeValue: std::fmt::Debug {
    fn as_number(&self) -> Option<f64>;
    fn as_operation(&self) -> Option<Operation>;
}

impl NodeValue for Operation {
    fn as_number(&self) -> Option<f64> {
        None
    }

    fn as_operation(&self) -> Option<Operation> {
        Some(*self)
    }
}

impl NodeValue for f64 {
    fn as_number(&self) -> Option<f64> {
        Some(*self)
    }

    fn as_operation(&self) -> Option<Operation> {
        None
    }
}
