use std::{sync::Weak, sync::{
    atomic::{AtomicUsize, Ordering}, Arc, Mutex
}};

use crate::{quad_tree::QuadTree, quad_tree_bounds::QuadTreeBounds};

static SEQUENCE: AtomicUsize = AtomicUsize::new(1);
pub struct QuadTreeLeaf {
    pub identity: usize,
    pub bounds: QuadTreeBounds,
    pub parent: Option<Weak<Mutex<QuadTree>>>,
}

impl QuadTreeLeaf {
    pub fn new(bounds: QuadTreeBounds, parent: Option<Weak<Mutex<QuadTree>>>) -> Self {
        let identity = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Self {
            bounds,
            parent,
            identity,
        }
    }

    pub fn remove(self) -> bool {
        QuadTree::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<Mutex<QuadTree>>> {
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
