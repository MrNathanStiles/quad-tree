use std::{
    ptr::{self, null_mut},
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{quad_tree_bounds::QuadTreeBounds, quad_tree_leaf::QuadTreeLeaf};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub struct QuadTree {
    pub identity: u64,
    pub root: bool,
    pub bounds: QuadTreeBounds,
    pub items: Vec<QuadTreeLeaf>,
    pub stuck: Vec<QuadTreeLeaf>,
    pub branches: Vec<*mut QuadTree>,
    pub parent: *mut QuadTree,
}

impl QuadTree {
    pub fn new(root: bool, x: i64, y: i64, size: i64, parent: *mut QuadTree) -> Self {
        let mut branches = Vec::with_capacity(4);
        for _ in 0..4 {
            branches.push(null_mut());
        }
        Self {
            identity: SEQUENCE.fetch_add(1, Ordering::Relaxed),
            root,
            bounds: QuadTreeBounds::new(x, y, size, size),
            items: Vec::with_capacity(2),
            stuck: Vec::new(),
            branches,
            parent,
        }
    }

    fn index(&self, bounds: QuadTreeBounds) -> isize {
        // 0 1
        // 3 2
        let ymid = self.bounds.y + (self.bounds.h / 2);
        let xmid = self.bounds.x + (self.bounds.w / 2);

        if bounds.y + bounds.h <= ymid {
            if bounds.x + bounds.w <= xmid {
                return 0;
            }
            if bounds.x >= xmid {
                return 1;
            }
        } else if bounds.y >= ymid {
            if bounds.x + bounds.w <= xmid {
                return 3;
            }
            if bounds.x >= xmid {
                return 2;
            }
        }
        return -1;
    }

    pub fn remove(leaf: &mut QuadTreeLeaf) {
        let parent = unsafe { &mut *leaf.parent };
        leaf.parent = ptr::null_mut();

        let mut removed = false;

        parent.items.retain(|l| {
            let retain = l.identity != leaf.identity;
            if !retain {
                removed = true;
            }
            retain
        });

        if !removed {
            parent.stuck.retain(|l| l.identity != leaf.identity);
        }

        if parent.items.len() > 0 {
            return;
        }
        if parent.stuck.len() > 0 {
            return;
        }

        for branch in parent.branches.iter() {
            if !branch.is_null() {
                return;
            }
        }
        if parent.parent.is_null() {
            return;
        }

        Self::remove_child(parent.parent, parent.identity, 0);
    }

    fn remove_child(self_pointer: *mut QuadTree, child_identity: u64, level: usize) {
        let this = unsafe { &mut *self_pointer };

        let mut branch_count = 0;
        for i in 0..4 {
            let branch_pointer = this.branches[i];
            if branch_pointer.is_null() {
                continue;
            }
            let branch = unsafe { &*branch_pointer };

            if branch.identity == child_identity {
                drop(unsafe { Box::from_raw(this.branches[i]) });
                this.branches[i] = null_mut();
            } else {
                branch_count += 1;
            }
        }

        if this.items.len() > 0 {
            return;
        }
        if this.stuck.len() > 0 {
            return;
        }
        if branch_count > 0 {
            return;
        }
        if this.parent.is_null() {
            return;
        }

        QuadTree::remove_child(this.parent, this.identity, level + 1);
    }

    pub fn climb(&self, list: &mut Vec<QuadTreeBounds>) {
        list.push(self.bounds);
        for i in 0..4 {
            let branch = self.branches[i];
            if branch.is_null() {
                continue;
            }
            let branch = unsafe { &*branch };
            branch.climb(list);
        }
    }

    pub fn query(&self, area: QuadTreeBounds, results: &mut Vec<QuadTreeLeaf>) {
        let mut list = Vec::new();

        if area.intersects(self.bounds) {
            list.push(self);
        }

        while list.len() > 0 {
            let tree = list.pop().unwrap();
            for i in 0..4 {
                let branch_pointer = tree.branches[i];
                if branch_pointer.is_null() {
                    continue;
                }
                let branch = unsafe { &*tree.branches[i] };
                if area.intersects(branch.bounds) {
                    list.push(branch);
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

    pub fn grow(&mut self) {
        let size = self.bounds.w;
        let half = size / 2;

        if !self.branches[0].is_null() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x - half,
                self.bounds.y - half,
                size,
                ptr::from_mut(self),
            );

            new_tree.branches[2] = self.branches[0];
            self.branches[0] = Box::into_raw(Box::new(new_tree));
        }
        if !self.branches[1].is_null() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x + half,
                self.bounds.y - half,
                size,
                ptr::from_mut(self),
            );
            new_tree.branches[3] = self.branches[1];
            self.branches[1] = Box::into_raw(Box::new(new_tree));
        }
        if !self.branches[2].is_null() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x + half,
                self.bounds.y + half,
                size,
                ptr::from_mut(self),
            );
            new_tree.branches[0] = self.branches[2];
            self.branches[2] = Box::into_raw(Box::new(new_tree));
        }
        if !self.branches[3].is_null() {
            let mut new_tree = QuadTree::new(
                false,
                self.bounds.x - half,
                self.bounds.y + half,
                size,
                ptr::from_mut(self),
            );
            new_tree.branches[1] = self.branches[3];
            self.branches[3] = Box::into_raw(Box::new(new_tree));
        }

        self.bounds.x -= half;
        self.bounds.y -= half;
        self.bounds.w += size;
        self.bounds.h += size;
    }

    pub fn insert(&mut self, mut new_leaf: QuadTreeLeaf, level: usize) {
        if self.root {
            loop {
                if self.bounds.contains(new_leaf.bounds) {
                    break;
                }
                self.grow();
            }
        }

        new_leaf.parent = ptr::from_mut(self);
        self.items.push(new_leaf);
        if self.items.len() < 2 {
            return;
        }

        while self.items.len() > 0 {
            let mut leaf = self.items.pop().unwrap();

            let index = self.index(leaf.bounds);

            if index < 0 {
                leaf.parent = ptr::from_mut(self);
                self.stuck.push(leaf);
                continue;
            }

            if self.branches[index as usize].is_null() {
                let size = self.bounds.w / 2;
                let mut x = self.bounds.x;
                let mut y = self.bounds.y;

                if 1 == index {
                    x += size;
                } else if 2 == index {
                    x += size;
                    y += size;
                } else if 3 == index {
                    y += size;
                }
                let new_branch = QuadTree::new(false, x, y, size, ptr::from_mut(self));
                self.branches[index as usize] = Box::into_raw(Box::new(new_branch));
            }
            let branch = unsafe { &mut *self.branches[index as usize] };
            branch.insert(leaf, level)
        }
    }
}

impl Drop for QuadTree {
    fn drop(&mut self) {
        for i in 0..4 {
            let branch_pointer = self.branches[i];
            if branch_pointer.is_null() {
                continue;
            }
            drop(unsafe { Box::from_raw(branch_pointer) });
            self.branches[i] = null_mut();
        }
    }
}
