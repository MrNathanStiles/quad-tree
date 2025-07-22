use actr_task::task_manager::TaskManager;
use parking_lot::Mutex;
use std::{
    mem,
    sync::{
        Arc, Weak,
        atomic::{AtomicU64, Ordering},
        mpsc::Sender,
    },
};

use crate::{quad_tree_bounds_ts::QuadTreeBoundsTs, quad_tree_leaf_ts::QuadTreeLeafTs};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub struct QuadTreeBranchTs {
    pub identity: u64,
    pub root: bool,
    pub bounds: QuadTreeBoundsTs,
    pub items: Vec<QuadTreeLeafTs>,
    pub stuck: Vec<QuadTreeLeafTs>,
    pub branches: Vec<Option<Arc<Mutex<QuadTreeBranchTs>>>>,
    pub parent: Option<Weak<Mutex<QuadTreeBranchTs>>>,
}

impl QuadTreeBranchTs {
    pub fn new(
        root: bool,
        x: i64,
        y: i64,
        size: i64,
        parent: Option<Weak<Mutex<QuadTreeBranchTs>>>,
    ) -> Self {
        Self {
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
            root,
            bounds: QuadTreeBoundsTs::new(x, y, size, size),
            items: Vec::with_capacity(2),
            stuck: Vec::new(),
            branches: (0..4).map(|_| None).collect::<Vec<_>>(),
            parent: parent,
        }
    }

    fn index(&self, other: QuadTreeBoundsTs) -> isize {
        // 0 1
        // 3 2

        let ymid = self.bounds.y + (self.bounds.h / 2);
        let xmid = self.bounds.x + (self.bounds.w / 2);

        if other.y + other.h <= ymid {
            if other.x + other.w <= xmid {
                return 0;
            }
            if other.x >= xmid {
                return 1;
            }
        } else if other.y >= ymid {
            if other.x + other.w <= xmid {
                return 3;
            }
            if other.x >= xmid {
                return 2;
            }
        }
        return -1;
    }

    pub fn remove(mut leaf: QuadTreeLeafTs) -> bool {
        let strong = leaf.get_parent();
        if strong.is_none() {
            return false;
        }

        let parent_mutex = strong.unwrap().clone();
        let mut parent = parent_mutex.lock();

        leaf.parent = None;

        let mut result = false;
        let mut item_count = 0;
        parent.items.retain(|l| {
            let retain = l.identity != leaf.identity;
            if retain {
                item_count += 1;
            } else {
                result = true;
            }
            retain
        });

        parent.stuck.retain(|l| {
            let retain = l.identity != leaf.identity;
            if retain {
                item_count += 1;
            } else {
                result = true;
            }
            retain
        });

        if item_count > 0 {
            return result;
        }

        for branch in parent.branches.iter() {
            if branch.is_some() {
                return result;
            }
        }
        if parent.parent.is_none() {
            return result;
        }

        let strong = parent.parent.clone().unwrap().upgrade();
        if strong.is_none() {
            return result;
        }
        let next = strong.unwrap();
        let identity = parent.identity;
        drop(parent);

        Self::remove_child(next, identity, 0);
        return result;
    }

    fn remove_child(self_pointer: Arc<Mutex<QuadTreeBranchTs>>, child_identity: u64, level: usize) {
        let mut this = self_pointer.lock();

        let mut branch_count = 0;
        for i in 0..4 {
            if this.branches[i].is_none() {
                continue;
            }

            let branch_option = this.branches[i].clone();
            let branch_result = branch_option.unwrap();
            let branch = branch_result.lock();
            if branch.identity == child_identity {
                this.branches[i] = None;
            } else {
                branch_count += 1;
            }
        }
        if branch_count > 0 {
            return;
        }
        if this.items.len() > 0 {
            return;
        }
        if this.stuck.len() > 0 {
            return;
        }

        if this.parent.is_none() {
            return;
        }
        let identity = this.identity;
        let strong = this.parent.clone().unwrap().upgrade();
        if strong.is_none() {
            return;
        }
        let next = strong.unwrap();
        drop(this);
        QuadTreeBranchTs::remove_child(next, identity, level + 1);
    }

    pub fn climb(arc: Arc<Mutex<QuadTreeBranchTs>>, list: &mut Vec<QuadTreeBoundsTs>) {
        let this = arc.lock();
        list.push(this.bounds);
        for i in 0..4 {
            if this.branches[i].is_none() {
                continue;
            }
            let branch = this.branches[i].clone();

            QuadTreeBranchTs::climb(branch.unwrap(), list);
        }
    }
    pub fn task_query(
        &self,
        area: QuadTreeBoundsTs,
        task_manager: TaskManager,
        results: Sender<QuadTreeLeafTs>,
    ) {
        if !self.bounds.intersects(area) {
            return;
        }
        for i in 0..4 {
            if self.branches[i].is_none() {
                continue;
            }
            let branch_mutex = self.branches[i].clone().unwrap();

            let results_clone = results.clone();
            let task_manager_clone = task_manager.clone();

            let ztask = move || {
                let branch = branch_mutex.lock();
                branch.task_query(area, task_manager_clone, results_clone);
            };
            task_manager.work(ztask).unwrap();
        }
        for leaf in self.items.iter() {
            if area.intersects(leaf.bounds) {
                results.send(leaf.clone()).unwrap();
            }
        }

        for leaf in self.stuck.iter() {
            if area.intersects(leaf.bounds) {
                results.send(leaf.clone()).unwrap();
            }
        }
    }

    pub fn query(
        arc: Arc<Mutex<QuadTreeBranchTs>>,
        area: QuadTreeBoundsTs,
        results: &mut Vec<QuadTreeLeafTs>,
    ) {
        let mut list = Vec::new();

        {
            let this = arc.lock();
            if area.intersects(this.bounds) {
                list.push(arc.clone());
            }
            drop(this);
        }
        while list.len() > 0 {
            let arc = list.pop().unwrap();
            let tree = arc.lock();
            for i in 0..4 {
                if tree.branches[i].is_none() {
                    continue;
                }
                let branch_option = tree.branches[i].clone().unwrap();
                let branch = branch_option.lock();
                if area.intersects(branch.bounds) {
                    drop(branch);
                    list.push(branch_option);
                }
            }
            for leaf in tree.items.iter() {
                if area.intersects(leaf.bounds) {
                    results.push(leaf.clone());
                }
            }

            for leaf in tree.stuck.iter() {
                if area.intersects(leaf.bounds) {
                    results.push(leaf.clone());
                }
            }
        }
    }

    pub fn grow(&mut self, zarc: Arc<Mutex<QuadTreeBranchTs>>) {
        let size = self.bounds.w;
        let half = size / 2;

        if !self.branches[0].is_none() {
            let mut new_tree = QuadTreeBranchTs::new(
                false,
                self.bounds.x - half,
                self.bounds.y - half,
                size,
                Some(Arc::downgrade(&zarc)),
            );

            new_tree.branches[2] = self.branches[0].clone();
            self.branches[0] = Some(Arc::new(Mutex::new(new_tree)));
        }
        if !self.branches[1].is_none() {
            let mut new_tree = QuadTreeBranchTs::new(
                false,
                self.bounds.x + half,
                self.bounds.y - half,
                size,
                Some(Arc::downgrade(&zarc)),
            );
            new_tree.branches[3] = self.branches[1].clone();
            self.branches[1] = Some(Arc::new(Mutex::new(new_tree)));
        }
        if !self.branches[2].is_none() {
            let mut new_tree = QuadTreeBranchTs::new(
                false,
                self.bounds.x + half,
                self.bounds.y + half,
                size,
                Some(Arc::downgrade(&zarc)),
            );
            new_tree.branches[0] = self.branches[2].clone();
            self.branches[2] = Some(Arc::new(Mutex::new(new_tree)));
        }
        if !self.branches[3].is_none() {
            let mut new_tree = QuadTreeBranchTs::new(
                false,
                self.bounds.x - half,
                self.bounds.y + half,
                size,
                Some(Arc::downgrade(&zarc)),
            );
            new_tree.branches[1] = self.branches[3].clone();
            self.branches[3] = Some(Arc::new(Mutex::new(new_tree)));
        }

        self.bounds.x -= half;
        self.bounds.y -= half;
        self.bounds.w += size;
        self.bounds.h += size;
    }

    pub fn insert(arc: Arc<Mutex<QuadTreeBranchTs>>, mut new_leaf: QuadTreeLeafTs) {
        let mut this = arc.lock();

        if this.root {
            loop {
                if this.bounds.contains(new_leaf.bounds) {
                    break;
                }

                QuadTreeBranchTs::grow(&mut *this, arc.clone());
            }
        }
        new_leaf.parent = Some(Arc::downgrade(&arc));
        this.items.push(new_leaf);
        if this.items.len() < 2 {
            return;
        }

        while this.items.len() > 0 {
            let mut leaf = this.items.pop().unwrap();

            let index = this.index(leaf.bounds);

            if index < 0 || this.bounds.w < 16 {
                leaf.parent = Some(Arc::downgrade(&arc));
                this.stuck.push(leaf);
                continue;
            }

            let branch_option = this.branches.get(index as usize).unwrap();
            let branch_arc = if branch_option.is_none() {
                let size = this.bounds.w / 2;
                let mut x = this.bounds.x;
                let mut y = this.bounds.y;

                if 1 == index {
                    x += size;
                } else if 2 == index {
                    x += size;
                    y += size;
                } else if 3 == index {
                    y += size;
                }
                let new_branch = Arc::new(Mutex::new(QuadTreeBranchTs::new(
                    false,
                    x,
                    y,
                    size,
                    Some(Arc::downgrade(&arc)),
                )));
                this.branches[index as usize] = Some(new_branch.clone());
                new_branch
            } else {
                branch_option.clone().unwrap()
            };

            QuadTreeBranchTs::insert(branch_arc, leaf);
        }
    }
}

fn drop_helper(tree: &mut QuadTreeBranchTs, stack: &mut Vec<Arc<Mutex<QuadTreeBranchTs>>>) {
    for i in 0..4 {
        if tree.branches[i].is_none() {
            continue;
        }
        stack.push(mem::replace(&mut tree.branches[i], None).unwrap());
    }
}
impl Drop for QuadTreeBranchTs {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        drop_helper(self, &mut stack);

        loop {
            let tree_option = stack.pop();
            if tree_option.is_none() {
                break;
            }
            let arc = tree_option.unwrap();
            let mut tree = arc.lock();
            drop_helper(&mut tree, &mut stack);
        }
    }
}
