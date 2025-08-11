use std::{
    fmt::Display,
    sync::{ atomic::{AtomicU64, Ordering}, Arc, Weak},
};

use parking_lot::RwLock;

use crate::{quad_tree_bounds_ts::QuadTreeBoundsTs, quad_tree_branch_ts::QuadTreeBranchTs};

//static SEQUENCE: AtomicU64 = AtomicU64::new(1);
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct QuadTreeLeafTs<T>
where T: Clone + Send + Sync + 'static
{
    pub(crate) identity: u64,
    pub item: T,
    pub bounds: QuadTreeBoundsTs,
    pub parent: Option<Weak<RwLock<QuadTreeBranchTs<T>>>>,
}

impl<T> QuadTreeLeafTs<T>
where T: Clone + Send + Sync + 'static
{
    pub fn new(
        item: T,
        bounds: QuadTreeBoundsTs,
        parent: Option<Weak<RwLock<QuadTreeBranchTs<T>>>>,
    ) -> Self {
        Self {
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
            item,
            bounds,
            parent,
        }
    }

    pub fn remove(self) -> bool {
        QuadTreeBranchTs::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<RwLock<QuadTreeBranchTs<T>>>> {
        match &self.parent {
            Some(parent) => parent.upgrade(),
            _ => None,
        }
    }
}

impl<T> Display for QuadTreeLeafTs<T>
where T: Clone + Send + Sync
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("leaf {}", &self.bounds))
    }
}
