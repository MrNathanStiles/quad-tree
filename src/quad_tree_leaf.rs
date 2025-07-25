use std::{cell::RefCell, rc::Weak};

use crate::{quad_tree::QuadTree, quad_tree_bounds::QuadTreeBounds};

#[derive(Clone)]
pub struct QuadTreeLeaf {
    pub identity: i64,
    pub bounds: QuadTreeBounds,
    pub parent: Weak<RefCell<QuadTree>>,
}

impl QuadTreeLeaf {
    pub fn new(identity: i64, bounds: QuadTreeBounds, parent: Weak<RefCell<QuadTree>>) -> Self {
        Self {
            bounds,
            parent,
            identity,
        }
    }
}
