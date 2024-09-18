use std::fmt;
use raylib::prelude::{Color, RaylibDraw};
use crate::vector2i::Vector2i;
use super::wire::Wire;
use gate::Gate;

pub mod gate;

#[derive(Debug)]
pub struct Node {
    pub(in crate::graph) inputs: Vec<Wire>,
    pub(in crate::graph) gate: Gate,
    pub                  position: Vector2i,
    pub(in crate::graph) visited: bool,
}

impl Node {
    pub fn new(gate: Gate, position: Vector2i) -> Self {
        Self {
            inputs: Vec::new(),
            gate,
            position,
            visited: false,
        }
    }

    pub fn evaluate(&self) -> bool {
        let input_states = self.inputs
            .iter()
            .filter_map(Wire::evaluate);

        self.gate.evaluate(input_states)
    }
}
