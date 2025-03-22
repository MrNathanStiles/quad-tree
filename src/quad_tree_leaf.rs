use std::{sync::atomic::{AtomicU64, Ordering}};

use crate::{quad_tree::QuadTree, quad_tree_bounds::QuadTreeBounds};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
pub struct QuadTreeLeaf {
    pub identity: u64,
    pub bounds: QuadTreeBounds,
    pub parent: *mut QuadTree,
}

impl QuadTreeLeaf {
    pub fn new(bounds: QuadTreeBounds, parent: *mut QuadTree) -> Self {
        let identity = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        Self { bounds, parent, identity }
    }
}
