use std::{cell::RefCell, cmp::Ordering, ops::Index, rc::Rc};

use crate::vector2i::Vector2i;

#[derive(Debug)]
struct QuadTreeBranch<T> {
    center: Vector2i,
    /// Rows then cols
    branches: Box<[Option<InfiniteQuadTree<T>>; 4]>
}

impl<T> QuadTreeBranch<T> {
    pub fn new() -> Self {
        todo!()
    }

    fn branch_containing(&self, position: &Vector2i) -> usize {
        ((self.center.y.cmp(&position.y).is_ge() as usize) << 1) + (self.center.x.cmp(&position.x).is_ge() as usize)
    }

    pub fn branch(&self, position: &Vector2i) -> &Option<InfiniteQuadTree<T>> {
        &self.branches[self.branch_containing(position)]
    }

    pub fn branch_mut(&mut self, position: &Vector2i) -> &mut Option<InfiniteQuadTree<T>> {
        &mut self.branches[self.branch_containing(position)]
    }
}

#[derive(Debug)]
enum QuadTreeInner<T> {
    Value(Vec<T>),
    Subtree(QuadTreeBranch<T>),
}

type KeyOf<T> = fn(&T) -> Vector2i;

#[derive(Debug)]
pub struct InfiniteQuadTree<T> {
    key_of: KeyOf<T>,
    content: QuadTreeInner<T>,
}

use Ordering::*;

impl<T> InfiniteQuadTree<T> {
    /// Construct an empty quad tree with a specified keying function.
    pub fn new(key_of: KeyOf<T>) -> Self {
        Self {
            key_of,
            content: QuadTreeInner::Value(Vec::new()),
        }
    }

    /// Returns whatever item was already there; `None` if the position was free.
    pub fn insert(&mut self, value: T) -> Option<T> {
        let position = (self.key_of)(&value);
        match &mut self.content {
            QuadTreeInner::Value(vec) => {
                let existing_index = vec.iter().position(|item| (self.key_of)(item) == position);
                match existing_index {
                    Some(index) => Some(std::mem::replace(&mut vec[index], value)),
                    None => { vec.push(value); None },
                }
            },

            QuadTreeInner::Subtree(subtree) => {
                let branch = subtree.branch_mut(&position);
                match branch {
                    Some(branch_tree) => branch_tree.insert(value),
                    None => branch.insert(InfiniteQuadTree::new(self.key_of)).insert(value),
                }
            },
        }
    }

    pub fn at(&self, position: Vector2i) -> Option<&T> {
        match &self.content {
            QuadTreeInner::Value(vec) => vec
                .iter()
                .find(|item| (self.key_of)(item) == position),

            QuadTreeInner::Subtree(subtree) => subtree
                .branch(&position)
                .as_ref()
                .and_then(|branch| branch.at(position)),
        }
    }

    pub fn at_mut(&mut self, position: Vector2i) -> Option<&mut T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const POINTS: [Vector2i; 3] = [
            Vector2i::new(5, 5),
            Vector2i::new(-6, 3),
            Vector2i::new(92, 46),
        ];

        let mut tree = InfiniteQuadTree::<Vector2i>::new(|p| *p);
        for p in POINTS {
            tree.insert(p);
        }
        println!("{tree:#?}");

        for p in POINTS {
            let found = tree.at(p).expect("should find exact position");
            assert_eq!(found, &p, "exact position should match");
        }
    }
}
