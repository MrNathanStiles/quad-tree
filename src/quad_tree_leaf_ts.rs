use std::{
    fmt::Display,
    sync::{ Arc, Weak},
};

use parking_lot::RwLock;

use crate::{quad_tree_bounds_ts::QuadTreeBoundsTs, quad_tree_branch_ts::QuadTreeBranchTs};

//static SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
pub struct QuadTreeLeafTs {
    pub identity: i64,
    pub bounds: QuadTreeBoundsTs,
    pub parent: Option<Weak<RwLock<QuadTreeBranchTs>>>,
}

impl QuadTreeLeafTs {
    pub fn new(
        identity: i64,
        bounds: QuadTreeBoundsTs,
        parent: Option<Weak<RwLock<QuadTreeBranchTs>>>,
    ) -> Self {
        Self {
            identity,
            bounds,
            parent,
        }
    }

    pub fn remove(self) -> bool {
        QuadTreeBranchTs::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<RwLock<QuadTreeBranchTs>>> {
        match &self.parent {
            Some(parent) => parent.upgrade(),
            _ => None,
        }
    }
}

impl Display for QuadTreeLeafTs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("leaf identity: ").unwrap();
        f.write_str(&self.identity.to_string()).unwrap();
        f.write_str(", bounds: ").unwrap();
        f.write_str(&self.bounds.to_string())
    }
}
