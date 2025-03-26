use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

use crate::{quad_tree::QuadTree, quad_tree_bounds::QuadTreeBounds};

static SEQUENCE: AtomicU64 = AtomicU64::new(1);
pub struct QuadTreeLeaf {
    pub identity: u64,
    pub bounds: QuadTreeBounds,
    pub parent: Option<Arc<Mutex<QuadTree>>>,
}

impl QuadTreeLeaf {
    pub fn new(bounds: QuadTreeBounds, parent: Option<Arc<Mutex<QuadTree>>>) -> Self {
        let identity = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Self {
            bounds,
            parent,
            identity,
        }
    }

    pub fn remove(self) {
        QuadTree::remove(self);
    }
}

impl Clone for QuadTreeLeaf {
    fn clone(&self) -> Self {
        Self {
            identity: self.identity,
            bounds: self.bounds,
            parent: self.parent.clone(),
        }
    }
}
