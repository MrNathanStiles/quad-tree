use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicUsize, Ordering},
};

use super::{quad_tree_bounds::QuadTreeBounds, quad_tree_branch::QuadTreeBranch};


static SEQUENCE: AtomicUsize = AtomicUsize::new(1);
pub struct QuadTreeLeaf {
    pub identity: usize,
    pub bounds: QuadTreeBounds,
    pub parent: Option<Weak<Mutex<QuadTreeBranch>>>,
}

impl QuadTreeLeaf {
    pub fn new(bounds: QuadTreeBounds, parent: Option<Weak<Mutex<QuadTreeBranch>>>) -> Self {
        let identity = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Self {
            bounds,
            parent,
            identity,
        }
    }

    pub fn remove(self) -> bool {
        QuadTreeBranch::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<Mutex<QuadTreeBranch>>> {
        if self.parent.is_none() {
            return None;
        }
        self.parent.clone().unwrap().upgrade()
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
