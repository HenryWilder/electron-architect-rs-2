use std::ops::*;

use raylib::prelude::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Default for Vector2i {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

impl Vector2i {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Vector2i> for Vector2 {
    fn from(Vector2i { x, y }: Vector2i) -> Self {
        Self::new(x as f32, y as f32)
    }
}

impl From<Vector2> for Vector2i {
    fn from(Vector2 { x, y }: Vector2) -> Self {
        Self::new(x as i32, y as i32)
    }
}

impl Add for Vector2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
