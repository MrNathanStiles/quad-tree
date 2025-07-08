use std::{cell::RefCell, rc::Weak, sync::atomic::{AtomicU64, Ordering}};

use crate::{quad_tree::QuadTree, quad_tree_bounds::QuadTreeBounds};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
pub struct QuadTreeLeaf {
    pub identity: u64,
    pub item: usize,
    pub bounds: QuadTreeBounds,
    pub parent: Weak<RefCell<QuadTree>>,
}

impl QuadTreeLeaf {
    pub fn new(item: usize, bounds: QuadTreeBounds, parent: Weak<RefCell<QuadTree>>) -> Self {
        let identity = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Self { item, bounds, parent, identity }
    }
}

impl Clone for QuadTreeLeaf {
    fn clone(&self) -> Self {
        
        Self { item: self.item.clone(), bounds: self.bounds.clone(), parent: self.parent.clone(), identity: self.identity }
    }
}
pub struct QuadTreeResult {
    pub item: usize,
    pub bounds: QuadTreeBounds,
    pub leaf: QuadTreeLeaf,
}

impl QuadTreeResult {
    pub fn new(leaf: &QuadTreeLeaf) -> QuadTreeResult {
        QuadTreeResult {
            bounds: leaf.bounds,
            item: leaf.item,
            leaf: leaf.clone(),
        }
    }
}