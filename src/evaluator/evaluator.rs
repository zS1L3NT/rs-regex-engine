use super::super::parser::{Node, Quantity, Single};

pub struct Evaluator<'a> {
    node: Node,
    string: &'a str,
}

impl<'a> Evaluator<'a> {
    pub fn new(node: Node, string: &'a str) -> Self {
        Self { node, string }
    }

    pub fn evaluate(&self) -> Option<Vec<String>> {
        None
    }
}
