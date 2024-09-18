use crate::vector2i::Vector2i;
use super::node::Node;

pub trait Positioned {
    fn position(&self) -> &Vector2i;
}

impl Positioned for Node {
    fn position(&self) -> &Vector2i {
        &self.position
    }
}

impl Positioned for Vector2i {
    fn position(&self) -> &Vector2i {
        self
    }
}

#[derive(Debug)]
struct QuadTreeBranch<T: Positioned> {
    center: Vector2i,
    /// Rows then cols
    branches: Box<[Option<InfiniteQuadTree<T>>; 4]>
}

impl<T: Positioned> QuadTreeBranch<T> {
    pub fn new() -> Self {
        Self {
            center: Vector2i::default(),
            branches: Box::default(),
        }
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

    pub fn insert(&mut self, item: T) -> Option<T> {
        let branch = self.branch_mut(item.position());
        if let Some(subtree) = branch {
            subtree.insert(item)
        } else {
            let subtree = InfiniteQuadTree::new_from(item);
            branch.insert(subtree);
            None
        }
    }

    // pub fn iter(&self) -> impl Iterator<Item = &T> {
    //     self.branches
    //         .iter()
    //         .flat_map(|branch| branch
    //             .as_ref()
    //             .and_then(|tree| Some(tree.iter())))
    //         .flatten()
    // }
}

#[derive(Debug)]
enum QuadTreeInner<T: Positioned> {
    Value(Vec<T>),
    Subtree(QuadTreeBranch<T>),
}

impl<T: Positioned> Default for QuadTreeInner<T> {
    fn default() -> Self {
        QuadTreeInner::Value(Vec::new())
    }
}

impl<T: Positioned> QuadTreeInner<T> {
    pub fn restructure(&mut self) {
        if let Self::Value(vec) = std::mem::replace(self, Self::Subtree(QuadTreeBranch::new())) {
            if let Self::Subtree(subtree) = self {
                {
                    let (mut center_x, mut center_y) = (0,0);
                    for position in vec.iter().map(Positioned::position) {
                        center_x += position.x;
                        center_y += position.y;
                    }
                    let n = vec.len() as i32;
                    center_x /= n;
                    center_y /= n;
                    subtree.center = Vector2i::new(center_x, center_y);
                }
                for item in vec {
                    subtree.insert(item);
                }
            } else {
                panic!("expected the memory I just replaced to have been replaced with what I frickin replaced it with");
            }
        } else {
            unimplemented!("should not restructure subtree, only value")
        }
    }
}

#[derive(Debug)]
pub struct InfiniteQuadTree<T: Positioned> {
    content: QuadTreeInner<T>,
}

impl<T: Positioned> InfiniteQuadTree<T> {
    const RESTRUCTURE_THRESHOLD: usize = 4;

    pub fn new() -> Self {
        Self { content: QuadTreeInner::default(), }
    }

    pub fn new_from(item: T) -> Self {
        Self { content: QuadTreeInner::Value(Vec::from([item])) }
    }

    /// Returns whatever item was already there; `None` if the position was free.
    pub fn insert(&mut self, value: T) -> Option<T> {
        let position = value.position();
        match &mut self.content {
            QuadTreeInner::Value(vec) => {
                let existing_index = vec.iter().position(|item| item.position() == position);
                match existing_index {
                    Some(index) => Some(std::mem::replace(&mut vec[index], value)),
                    None => {
                        vec.push(value);
                        if vec.len() >= Self::RESTRUCTURE_THRESHOLD {
                            self.content.restructure();
                        }
                        None
                    },
                }
            },

            QuadTreeInner::Subtree(subtree) => {
                let branch = subtree.branch_mut(&position);
                match branch {
                    Some(branch_tree) => branch_tree.insert(value),
                    None => branch.insert(InfiniteQuadTree::new()).insert(value),
                }
            },
        }
    }

    pub fn at(&self, position: Vector2i) -> Option<&T> {
        match &self.content {
            QuadTreeInner::Value(vec) => vec
                .iter()
                .find(|item| item.position() == &position),

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

pub struct Iter<'a, T: 'a + Positioned> {
    stack: Vec<(&'a InfiniteQuadTree<T>, usize)>,
}

impl<'a, T: Positioned> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(top) = self.stack.last_mut() {
            let (tree, mut index) = top;
            match &tree.content {
                QuadTreeInner::Value(vec) => {
                    if index < vec.len() {
                        Some(&vec[index])
                    } else {
                        self.stack.pop();
                        if let Some((_, index)) = self.stack.last_mut() {
                            *index += 1;
                            self.next()
                        } else {
                            None
                        }
                    }
                },
                QuadTreeInner::Subtree(subtree) => {
                    if let Some(branch) = subtree.branches[index].as_ref() {
                        self.stack.push((branch, 0));
                    }
                    index += 1;
                    self.next()
                },
            }
        } else {
            None
        }
    }
}

impl<T: Positioned> InfiniteQuadTree<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            stack: Vec::from([(self, 0)]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raylib::prelude::*;

    #[test]
    fn test() {
        const POINTS: [Vector2i; 6] = [
            Vector2i::new(5, 5),
            Vector2i::new(-6, 3),
            Vector2i::new(92, 46),
            Vector2i::new(32, 46),
            Vector2i::new(55, 12),
            Vector2i::new(-29, 14),
        ];

        let mut tree = InfiniteQuadTree::new();
        for p in POINTS {
            tree.insert(p);
        }
        println!("{tree:#?}");

        for p in POINTS {
            let found = tree.at(p).expect("should find exact position");
            assert_eq!(found, &p, "exact position should match");
        }

        let tree = &tree;

        let (mut rl, thread) = init()
            .size(640, 480)
            .title("test")
            .build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            for item in tree.iter() {
                let &Vector2i { x, y } = item.position();
                d.draw_pixel(x, y, Color::BLUE);
            }
        }
    }
}
