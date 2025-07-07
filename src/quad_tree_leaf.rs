use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicUsize, Ordering},
};

use super::{quad_tree_bounds::QuadTreeBounds, quad_tree_branch::QuadTreeBranch};


static SEQUENCE: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct QuadTreeLeaf {
    pub identity: usize,
    pub bounds: QuadTreeBounds,
    pub parent: Option<Weak<Mutex<QuadTreeBranch>>>,
}

impl QuadTreeLeaf {
    pub fn new(bounds: QuadTreeBounds, parent: Option<Weak<Mutex<QuadTreeBranch>>>) -> Self {
        Self {
            bounds,
            parent,
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn remove(self) -> bool {
        QuadTreeBranch::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<Mutex<QuadTreeBranch>>> {
        match &self.parent {
            Some(parent) => parent.upgrade(),
            _ => None
        }
    }
}
