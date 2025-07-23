use std::{cell::RefCell, rc::Weak, sync::atomic::{AtomicU64, Ordering}};

use crate::{quad_tree::QuadTree, quad_tree_bounds::QuadTreeBounds};


#[derive(Clone)]
pub struct QuadTreeLeaf {
    pub identity: u64,
    pub bounds: QuadTreeBounds,
    pub parent: Weak<RefCell<QuadTree>>,
}

impl QuadTreeLeaf {
    pub fn new(identity: u64, bounds: QuadTreeBounds, parent: Weak<RefCell<QuadTree>>) -> Self {
        
        Self { bounds, parent, identity }
    }
}

