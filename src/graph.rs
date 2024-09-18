use std::{cell::RefCell, rc::{Rc, Weak}};
use node::{gate::Gate, Node};
use quad_tree::InfiniteQuadTree;
use raylib::{prelude::{Color, RaylibDraw, Rectangle, Vector2}};
use wire::Wire;
use crate::vector2i::Vector2i;

pub mod node;
pub mod wire;
pub mod quad_tree;

pub struct Graph {
    nodes: InfiniteQuadTree<Node>,
}

impl Graph {
    pub const GRID_SIZE: f32 = 15.0;

    pub fn new() -> Self {
        Self {
            nodes: InfiniteQuadTree::new(|node| node.position),
        }
    }

    /// Returns the cell containing the position
    pub fn world_to_grid(&self, position: Vector2) -> Vector2i {
        Vector2i::new(
            (position.x / Self::GRID_SIZE).floor() as i32,
            (position.y / Self::GRID_SIZE).floor() as i32,
        )
    }

    /// Returns the top left corner of the cell
    pub fn grid_to_world(&self, cell: Vector2i) -> Vector2 {
        Vector2::new(
            cell.x as f32 * Self::GRID_SIZE,
            cell.y as f32 * Self::GRID_SIZE,
        )
    }

    /// Returns the top left corner of the cell
    pub fn grid_to_world_centered(&self, cell: Vector2i) -> Vector2 {
        Vector2::new(
            (cell.x as f32 + 0.5) * Self::GRID_SIZE,
            (cell.y as f32 + 0.5) * Self::GRID_SIZE,
        )
    }

    pub fn node_world_centered(&self, node: &Rc<RefCell<Node>>) -> Vector2 {
        self.grid_to_world_centered(node.borrow().position)
    }

    fn reset_visited(&mut self) {
        todo!();
        // for node in &mut self.nodes {
        //     node.borrow_mut().visited = false;
        // }
    }

    pub fn evaluate_all(&mut self) {
        todo!();
        // for node in &mut self.nodes {
        //     let state = node
        //         .borrow()
        //         .evaluate();
        // }
    }

    pub fn put_node(&mut self, gate: Gate, position: Vector2i) -> Weak<RefCell<Node>> {
        todo!();
        // let new_node = Node::new(gate, position);
        // let node_rc = Rc::new(RefCell::new(new_node));
        // self.nodes.push(node_rc);
        // if let Some(node_ref) = self.nodes.iter().last() {
        //     Rc::downgrade(node_ref)
        // } else {
        //     Weak::new()
        // }
    }

    pub fn find_node_at(&self, position: Vector2i) -> Option<Weak<RefCell<Node>>> {
        todo!();
        // self.nodes
        //     .iter()
        //     .find_map(|node|
        //         (node.borrow().position == position)
        //             .then(|| Rc::downgrade(node))
        //     )
    }

    /// Wires cannot be created from nodes already known to be dropped.
    pub fn wire(&mut self, input: Rc<RefCell<Node>>, output: Rc<RefCell<Node>>) {
        let p1 = input.borrow().position;
        let p2 = output.borrow().position;
        let elbow = Vector2i::new(p2.x, p1.y);
        let new_wire = Wire::new(&input, &output, Vec::from([elbow]));
        output.borrow_mut().inputs.push(new_wire);
    }

    pub fn draw_wires(&self, d: &mut impl RaylibDraw) {
        todo!();
        // for node in self.nodes.iter() {
        //     let node = node.borrow();
        //     let node_position = self.grid_to_world_centered(node.position);
        //     for wire in node.inputs.iter() {
        //         if let Some(input) = wire.input.upgrade() {
        //             let input = input.borrow();
        //             let input_position = self.grid_to_world_centered(input.position);

        //             let mut points = Vec::with_capacity(wire.elbows.len() + 2);
        //             points.push(input_position);
        //             points.extend(wire.elbows.iter().map(|&p| self.grid_to_world_centered(p)));
        //             points.push(node_position);
        //             d.draw_line_strip(points.as_slice(), Color::WHITE);
        //         }
        //     }
        // }
    }

    pub fn draw_nodes(&self, d: &mut impl RaylibDraw) {
        todo!();
        // for node in self.nodes.iter() {
        //     let Vector2 { x, y } = self.grid_to_world(node.borrow().position);
        //     d.draw_rectangle_rec(Rectangle::new(x, y, Self::GRID_SIZE, Self::GRID_SIZE), Color::WHITE);
        // }
    }

    pub fn draw_proxy_nodes(&self, d: &mut impl RaylibDraw) {
        todo!();
        // for node in self.nodes.iter() {
        //     let Vector2 { x, y } = self.grid_to_world(node.borrow().position);
        //     d.draw_rectangle_rec(Rectangle::new(x, y, Self::GRID_SIZE, Self::GRID_SIZE), Color::WHITE);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_wire() {
        let mut graph = Graph::new();
        let node_1 = graph.put_node(Gate::Or, Vector2i::new(0,0));
        let node_2 = graph.put_node(Gate::Or, Vector2i::new(1,0));
        if let (Some(input), Some(output)) = (node_1.upgrade(), node_2.upgrade()) {
            graph.wire(input, output);
        } else {
            assert!(false, "one or both weak references failed to be upgraded");
        }
    }
}
