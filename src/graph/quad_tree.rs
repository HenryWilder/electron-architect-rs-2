use raylib::prelude::RaylibDrawHandle;

use crate::vector2i::Vector2i;
use super::node::Node;

pub trait Positioned {
    fn position(&self) -> Vector2i;
}

impl Positioned for Node {
    fn position(&self) -> Vector2i {
        self.position
    }
}

impl Positioned for Vector2i {
    fn position(&self) -> Vector2i {
        *self
    }
}

const QUAD_TREE_BRANCH_COUNT: usize = 4;

#[derive(Debug)]
struct QuadTreeBranch<T: Positioned> {
    center: Vector2i,
    /// Rows then cols
    branches: Box<[Option<InfiniteQuadTree<T>>; QUAD_TREE_BRANCH_COUNT]>
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
        let branch = self.branch_mut(&item.position());
        if let Some(subtree) = branch {
            subtree.insert(item)
        } else {
            let subtree = InfiniteQuadTree::new_from(item);
            branch.insert(subtree);
            None
        }
    }
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
    pub fn region_bounds(&self) -> (Vector2i, Vector2i) {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        match self {
            Self::Value(vec) => {
                for item in vec {
                    let Vector2i{ x, y } = item.position();
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            },
            Self::Subtree(subtree) => {
                for branch in subtree.branches.iter().flatten() {
                    let (Vector2i{ x: min_x_1, y: min_y_1 }, Vector2i{ x: max_x_1, y: max_y_1 }) = branch.content.region_bounds();
                    min_x = min_x.min(min_x_1);
                    min_y = min_y.min(min_y_1);
                    max_x = max_x.max(max_x_1);
                    max_y = max_y.max(max_y_1);
                }
            },
        }
        (Vector2i::new(min_x, min_y), Vector2i::new(max_x, max_y))
    }

    pub fn restructure(&mut self) {
        println!("restructuring");
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
                .find(|item| item.position() == position),

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

use std::collections::LinkedList;

pub struct Iter<'a, T: 'a + Positioned> {
    stack: LinkedList<(&'a QuadTreeInner<T>, usize)>,
}

impl<'a, T: Positioned> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        use QuadTreeInner::*;
        if let Some((tree, index)) = self.stack.front_mut() {
            let tree = *tree;
            let current_index = *index;
            *index += 1;
            let len = match tree {
                Value(vec) => vec.len(),
                Subtree(_) => QUAD_TREE_BRANCH_COUNT,
            };
            if current_index < len {
                match tree {
                    Value(vec) => return Some(&vec[current_index]),

                    Subtree(QuadTreeBranch { center, branches }) =>
                        if let Some(branch) = branches[current_index].as_ref() {
                            self.stack.push_front((&branch.content, 0));
                        },
                }
            } else {
                self.stack.pop_front();
            }
            self.next()
        } else {
            None
        }
    }
}

impl<T: Positioned> InfiniteQuadTree<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            stack: LinkedList::from([(&self.content, 0)]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use raylib::prelude::*;

    fn debug_next<'a, T: Positioned>(it: &mut Iter<'a, T>, d: &mut impl RaylibDraw) -> Option<&'a T> {
        use QuadTreeInner::*;
        if let Some((tree, index)) = it.stack.front_mut() {
            let tree = *tree;
            let current_index = *index;
            *index += 1;
            let len = match tree {
                Value(vec) => vec.len(),
                Subtree(_) => QUAD_TREE_BRANCH_COUNT,
            };
            let (Vector2i { x: xmin, y: ymin }, Vector2i { x: xmax, y: ymax }) = tree.region_bounds();
            let (width, height) = (xmax - xmin, ymax - ymin);
            d.draw_rectangle_lines(xmin, ymin, width, height, Color::GREENYELLOW);
            if current_index < len {
                match tree {
                    Value(vec) => return Some(&vec[current_index]),

                    Subtree(QuadTreeBranch { center, branches }) => {
                        d.draw_rectangle(center.x, ymin, 1, height, Color::BLUE);
                        d.draw_rectangle(xmin, center.y, width, 1, Color::BLUE);
                        if let Some(branch) = branches[current_index].as_ref() {
                            it.stack.push_front((&branch.content, 0));
                        }
                    },
                }
            } else {
                it.stack.pop_front();
            }
            debug_next(it, d)
        } else {
            None
        }
    }

    #[test]
    fn test() {
        const NUM_POINTS: usize = 100;
        let mut points: Vec<Vector2i> = Vec::with_capacity(NUM_POINTS);
        let mut r = rand::thread_rng();
        for i in 0..NUM_POINTS {
            points.push(Vector2i::new(
                r.gen_range(-256..=256),
                r.gen_range(-256..=256),
            ));
        }
        let points = points;

        let mut tree = InfiniteQuadTree::new();
        for p in points.iter() {
            tree.insert(*p);
        }
        // println!("{tree:#?}");

        for p in points.iter() {
            let found = tree.at(*p).expect("should find exact position");
            assert_eq!(found, p, "exact position should match");
        }

        let tree = &tree;

        let (mut rl, thread) = init()
            .size(640, 480)
            .title("test")
            .build();

        rl.set_target_fps(60);

        let mut camera = Camera2D {
            offset: Vector2::default(),
            target: Vector2::default(),
            rotation: 0.0,
            zoom: 1.0,
        };

        const SPEED: f32 = 2.0;
        while !rl.window_should_close() {
            camera.target.x += (rl.is_key_down(KeyboardKey::KEY_RIGHT) as isize as f32 - rl.is_key_down(KeyboardKey::KEY_LEFT) as isize as f32) * SPEED;
            camera.target.y += (rl.is_key_down(KeyboardKey::KEY_DOWN ) as isize as f32 - rl.is_key_down(KeyboardKey::KEY_UP  ) as isize as f32) * SPEED;
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            {
                let mut c = d.begin_mode2D(camera);
                c.draw_rectangle(0, -1000, 1, 2000, Color::WHITE);
                c.draw_rectangle(-1000, 0, 2000, 1, Color::WHITE);
                let mut it = tree.iter();
                while let Some(item) = debug_next(&mut it, &mut c) {
                    let Vector2i { x, y } = item.position();
                    c.draw_pixel(x, y, Color::ORANGE);
                }
            }
        }
    }
}
