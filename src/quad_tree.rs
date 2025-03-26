use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

use crate::{quad_tree_bounds::QuadTreeBounds, quad_tree_leaf::QuadTreeLeaf};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub struct QuadTree {
    pub identity: u64,
    pub root: bool,
    pub bounds: QuadTreeBounds,
    pub items: Vec<QuadTreeLeaf>,
    pub stuck: Vec<QuadTreeLeaf>,
    pub branches: Vec<Option<Arc<Mutex<QuadTree>>>>,
    pub parent: Option<Arc<Mutex<QuadTree>>>,
}

impl QuadTree {
    pub fn new(
        root: bool,
        x: i64,
        y: i64,
        size: i64,
        parent: Option<Arc<Mutex<QuadTree>>>,
    ) -> Self {
        let mut branches = Vec::with_capacity(4);
        for _ in 0..4 {
            branches.push(None);
        }
        Self {
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
            root,
            bounds: QuadTreeBounds::new(x, y, size, size),
            items: Vec::with_capacity(2),
            stuck: Vec::new(),
            branches: branches,
            parent: parent,
        }
    }

    fn index(&self, other: QuadTreeBounds) -> isize {
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

    pub fn remove(mut leaf: QuadTreeLeaf) {
        let parent_mutex = leaf.parent.unwrap().clone();
        let mut parent = parent_mutex.lock().unwrap();

        leaf.parent = None;

        let mut item_count = 0;
        parent.items.retain(|l| {
            let retain = l.identity != leaf.identity;
            if retain {
                item_count += 1;
            }
            retain
        });

        parent.stuck.retain(|l| {
            let retain = l.identity != leaf.identity;
            if retain {
                item_count += 1;
            }
            retain
        });

        if item_count > 0 {
            return;
        }

        for branch in parent.branches.iter() {
            if branch.is_some() {
                return;
            }
        }
        if parent.parent.is_none() {
            return;
        }

        let next = parent.parent.clone().unwrap();

        let identity = parent.identity;

        drop(parent);

        Self::remove_child(next, identity, 0);
    }

    fn remove_child(self_pointer: Arc<Mutex<QuadTree>>, child_identity: u64, level: usize) {
        let mut this = self_pointer.lock().unwrap();

        let mut branch_count = 0;
        for i in 0..4 {
            if this.branches[i].is_none() {
                continue;
            }

            let branch_option = this.branches[i].clone();
            let branch_result = branch_option.unwrap();
            let branch_lock_result = branch_result.lock();
            let branch = branch_lock_result.unwrap();
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
        let next = this.parent.clone().unwrap();
        drop(this);
        QuadTree::remove_child(next, identity, level + 1);
    }

    pub fn climb(arc: Arc<Mutex<QuadTree>>, list: &mut Vec<QuadTreeBounds>) {
        let this = arc.lock().unwrap();
        list.push(this.bounds);
        for i in 0..4 {
            if this.branches[i].is_none() {
                continue;
            }
            let branch = this.branches[i].clone();

            QuadTree::climb(branch.unwrap(), list);
        }
    }

    pub fn query(arc: Arc<Mutex<QuadTree>>, area: QuadTreeBounds, results: &mut Vec<QuadTreeLeaf>) {
        let mut list = Vec::new();

        let this = arc.lock().unwrap();
        if area.intersects(this.bounds) {
            list.push(arc.clone());
        }
        drop(this);
        while list.len() > 0 {
            let arc = list.pop().unwrap();
            let tree = arc.lock().unwrap();
            for i in 0..4 {
                if tree.branches[i].is_none() {
                    continue;
                }
                let branch_option = tree.branches[i].clone().unwrap();
                let branch = branch_option.lock().unwrap();
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

    pub fn grow(&mut self, zarc: Arc<Mutex<QuadTree>>) {
        //let mut this = arc.lock().unwrap();
        let size = self.bounds.w;
        let half = size / 2;

        if !self.branches[0].is_none() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x - half,
                self.bounds.y - half,
                size,
                Some(zarc.clone()),
            );

            new_tree.branches[2] = self.branches[0].clone();
            self.branches[0] = Some(Arc::new(Mutex::new(new_tree)));
        }
        if !self.branches[1].is_none() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x + half,
                self.bounds.y - half,
                size,
                Some(zarc.clone()),
            );
            new_tree.branches[3] = self.branches[1].clone();
            self.branches[1] = Some(Arc::new(Mutex::new(new_tree)));
        }
        if !self.branches[2].is_none() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x + half,
                self.bounds.y + half,
                size,
                Some(zarc.clone()),
            );
            new_tree.branches[0] = self.branches[2].clone();
            self.branches[2] = Some(Arc::new(Mutex::new(new_tree)));
        }
        if !self.branches[3].is_none() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x - half,
                self.bounds.y + half,
                size,
                Some(zarc.clone()),
            );
            new_tree.branches[1] = self.branches[3].clone();
            self.branches[3] = Some(Arc::new(Mutex::new(new_tree)));
        }

        self.bounds.x -= half;
        self.bounds.y -= half;
        self.bounds.w += size;
        self.bounds.h += size;
    }

    pub fn insert(arc: Arc<Mutex<QuadTree>>, mut new_leaf: QuadTreeLeaf) {
        let mut this = arc.lock().unwrap();

        if this.root {
            loop {
                if this.bounds.contains(new_leaf.bounds) {
                    break;
                }

                QuadTree::grow(&mut *this, arc.clone());
            }
        }
        new_leaf.parent = Some(arc.clone());
        this.items.push(new_leaf);
        if this.items.len() < 2 {
            return;
        }

        while this.items.len() > 0 {
            let mut leaf = this.items.pop().unwrap();

            let index = this.index(leaf.bounds);

            if index < 0 {
                leaf.parent = Some(arc.clone());
                this.stuck.push(leaf);
                continue;
            }

            if this.branches[index as usize].is_none() {
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
                let new_branch = QuadTree::new(false, x, y, size, Some(arc.clone()));
                this.branches[index as usize] = Some(Arc::new(Mutex::new(new_branch)));
            }
            let branch_arc = this.branches[index as usize].clone().unwrap();
            QuadTree::insert(branch_arc, leaf);
        }
    }
}

impl Drop for QuadTree {
    fn drop(&mut self) {
        for i in 0..4 {
            if self.branches[i].is_none() {
                continue;
            }
            self.branches[i] = None;
        }
    }
}
