use std::{
    fmt::Display,
    sync::{
        Arc, Weak,
        atomic::{AtomicI64, AtomicU64, Ordering},
    },
};

use parking_lot::{Mutex, RwLock};

use crate::{quad_tree_bounds_ts::QuadTreeBoundsTs, quad_tree_branch_ts::QuadTreeBranchTs};
//-9223372036854775808
static SEQUENCE: AtomicI64 = AtomicI64::new(-9007199254740991);
//static SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
struct QuadTreeLeafContainer<T>
where
    T: Clone + Send + Sync + 'static,
{
    parent: Option<Weak<RwLock<QuadTreeBranchTs<T>>>>,
}

#[derive(Clone)]
pub struct QuadTreeLeafTs<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub(crate) identity: i64,
    pub(crate) bounds: QuadTreeBoundsTs,
    pub(crate) item: T,
    container: Arc<Mutex<QuadTreeLeafContainer<T>>>,
}

impl<T> QuadTreeLeafTs<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(item: T, bounds: QuadTreeBoundsTs) -> Self {
        Self {
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
            bounds,
            item,
            container: Arc::new(Mutex::new(QuadTreeLeafContainer {
                
                parent: None,
            })),
        }
    }

    pub fn remove(&mut self) -> bool {
        QuadTreeBranchTs::remove(self)
    }

    pub fn get_parent(&self) -> Option<Arc<RwLock<QuadTreeBranchTs<T>>>> {
        let guard = self.container.lock();
        match &guard.parent {
            Some(parent) => parent.upgrade(),
            _ => None,
        }
    }

    pub fn set_parent(&self, parent: Option<Weak<parking_lot::lock_api::RwLock<parking_lot::RawRwLock, QuadTreeBranchTs<T>>>>) {
        let mut guard = self.container.lock();
        guard.parent = parent;
    }

    pub fn get_identity(&self) -> i64 {
        self.identity
    }

    pub fn get_item(&self) -> T {
        self.item.clone()
    }
    pub fn get_bounds(&self) -> QuadTreeBoundsTs {
        self.bounds
    }

}

impl<T> Display for QuadTreeLeafTs<T>
where
    T: Clone + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        f.write_str(&format!("leaf {}", &self.bounds))
    }
}
