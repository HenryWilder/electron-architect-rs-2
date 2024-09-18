use std::{cell::RefCell, fmt, rc::{Rc, Weak}};
use crate::vector2i::Vector2i;

use super::node::Node;

#[derive(Debug)]
pub struct Wire {
    pub(in crate::graph) input: Weak<RefCell<Node>>,
    pub(in crate::graph) owner: Weak<RefCell<Node>>,
    pub(in crate::graph) elbows: Vec<Vector2i>,
}

impl Wire {
    pub fn new(input: &Rc<RefCell<Node>>, output: &Rc<RefCell<Node>>, elbows: Vec<Vector2i>) -> Self {
        Self {
            input: Rc::downgrade(input),
            owner: Rc::downgrade(output),
            elbows,
        }
    }

    pub fn evaluate(&self) -> Option<bool> {
        self.input
            .upgrade()
            .map(|input_node| input_node.borrow().evaluate())
    }
}
