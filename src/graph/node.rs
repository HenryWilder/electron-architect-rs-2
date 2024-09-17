use std::{collections::VecDeque, rc::{Rc, Weak}};
use super::wire::Wire;
use gate::Gate;

pub mod gate;

pub struct Node {
    inputs:  VecDeque<Weak<Wire>>,
    outputs: VecDeque<Weak<Wire>>,
    gate: Gate,
}

impl Node {
    pub fn evaluate(&self) -> bool {
        let input_states = self.inputs
            .iter()
            .map(|input_node| input_node
                .upgrade()
                .map_or(false, |input_node| input_node.as_ref().evaluate_input())
            );

        self.gate.evaluate(input_states)
    }
}
