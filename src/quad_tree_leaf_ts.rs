use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicUsize, Ordering},
};

use crate::{quad_tree_bounds_ts::QuadTreeBoundsTs, quad_tree_branch_ts::QuadTreeBranchTs};



static SEQUENCE: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct QuadTreeLeafTs {
    pub identity: usize,
    pub bounds: QuadTreeBoundsTs,
    pub parent: Option<Weak<Mutex<QuadTreeBranchTs>>>,
}

impl QuadTreeLeafTs {
    pub fn new(bounds: QuadTreeBoundsTs, parent: Option<Weak<Mutex<QuadTreeBranchTs>>>) -> Self {
        Self {
            bounds,
            parent,
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn remove(self) -> bool {
        QuadTreeBranchTs::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<Mutex<QuadTreeBranchTs>>> {
        match &self.parent {
            Some(parent) => parent.upgrade(),
            _ => None
        }
    }
}
