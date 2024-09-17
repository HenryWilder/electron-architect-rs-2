use std::rc::{Rc, Weak};
use super::node::Node;
use elbow::Elbow;

pub mod elbow;

pub struct Wire {
    input:  Weak<Node>,
    output: Weak<Node>,
    elbow: Elbow
}

impl Wire {
    pub fn evaluate_input(&self) -> bool {
        self.input
            .upgrade()
            .as_ref()
            .map_or(false, |input_node| input_node.evaluate())
    }
}
