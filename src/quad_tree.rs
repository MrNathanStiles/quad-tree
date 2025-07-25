use std::{
    cell::RefCell,
    mem, ptr,
    rc::{Rc, Weak},
};

use crate::{
    quad_tree_bounds::QuadTreeBounds,
    quad_tree_leaf::{QuadTreeLeaf},
};

pub struct QuadTree {
    pub root: bool,
    pub bounds: QuadTreeBounds,
    pub items: Vec<QuadTreeLeaf>,
    pub stuck: Vec<QuadTreeLeaf>,
    pub branches: Vec<Option<Rc<RefCell<QuadTree>>>>,
    pub parent: Option<Weak<RefCell<QuadTree>>>,
}

impl QuadTree {
    pub fn new(
        root: bool,
        x: i64,
        y: i64,
        size: i64,
        parent: Option<Weak<RefCell<QuadTree>>>,
    ) -> Self {
        let mut branches = Vec::new();
        for _ in 0..4 {
            branches.push(Option::None);
        }
        Self {
            root,
            bounds: QuadTreeBounds::new(x, y, size, size),
            items: Vec::with_capacity(2),
            stuck: Vec::new(),
            branches,
            parent,
        }
    }

    fn index(&self, bounds: &QuadTreeBounds) -> isize {
        // 0 1
        // 3 2
        let ymid = self.bounds.y + (self.bounds.h / 2);
        let xmid = self.bounds.x + (self.bounds.w / 2);

        if bounds.y + bounds.h <= ymid {
            // top half
            if bounds.x + bounds.w <= xmid {
                // left half
                return 0;
            }
            if bounds.x >= xmid {
                // right half
                return 1;
            }
        } else if bounds.y >= ymid {
            // bottom half
            if bounds.x + bounds.w <= xmid {
                // left half
                return 3;
            }
            if bounds.x >= xmid {
                // right half
                return 2;
            }
        }
        return -1;
    }

    pub fn remove(leaf: &QuadTreeLeaf) ->bool {
        let mut removed = false;
        let foo = leaf.parent.upgrade().unwrap();
        {
            let mut tree = foo.borrow_mut();

            

            tree.items.retain(|l| {
                let retain = l.identity != leaf.identity;
                if !retain {
                    removed = true;
                }
                retain
            });

            if !removed {
                tree.stuck.retain(|l| {
                    let retain = l.identity != leaf.identity;
                    if !retain {
                        removed = true;
                    }
                    retain
                });
            }

            if tree.items.len() > 0 {
                return removed;
            }
            if tree.stuck.len() > 0 {
                return removed;
            }

            for branch in tree.branches.iter() {
                if branch.is_some() {
                    return removed;
                }
            }
        }
        let foo = leaf.parent.upgrade().unwrap();
        QuadTree::_actr_quad_tree_remove_tree(foo, 0);
        removed
    }

    
    fn _actr_quad_tree_remove_tree(child_rc: Rc<RefCell<QuadTree>>, level: usize) {


        let child_ref = child_rc.as_ref();
        let child_borrow = child_ref.borrow();

        if child_borrow.parent.is_none() {
            return;
        }

        let parent_weak = child_borrow.parent.as_ref().unwrap();
        let parent_rc = parent_weak.upgrade().unwrap();
        {
            let parent_ref = parent_rc.as_ref();
            let mut parent_borrow = parent_ref.borrow_mut();

            if parent_borrow.root {
                return;
            }

            for i in 0..4 {
                if parent_borrow.branches[i].is_none() {
                    continue;
                }

                let branch_rc = parent_borrow.branches[i].as_ref().unwrap();
                let branch_ref = branch_rc.as_ref();
                //let branch_borrow = branch_rc.borrow();

                if ptr::eq(child_ref, branch_ref) {
                    parent_borrow.branches[i] = None;
                    break;
                }
            }

            if parent_borrow.items.len() > 0 {
                return;
            }

            if parent_borrow.stuck.len() > 0 {
                return;
            }

            for i in 0..4 {
                if parent_borrow.branches[i].is_some() {
                    return;
                }
            }
        }
        QuadTree::_actr_quad_tree_remove_tree(parent_rc, level + 1);
    }

    pub fn query(
        self_rc: Rc<RefCell<QuadTree>>,
        area: QuadTreeBounds,
        results: &mut Vec<QuadTreeLeaf>,
    ) {
        let mut list = Vec::new();

        let self_ref = self_rc.as_ref();
        let self_borrow = self_ref.borrow();

        if area.intersects(&self_borrow.bounds) {
            list.push(self_rc.clone());
        }

        while list.len() > 0 {
            let tree_rc = list.pop().unwrap();
            let tree_ref = tree_rc.as_ref();
            let tree_borrow = tree_ref.borrow();

            for i in 0..4 {
                if tree_borrow.branches[i].is_none() {
                    continue;
                }

                let branch_option = tree_borrow.branches[i].as_ref();
                let branch_rc = branch_option.unwrap();
                let branch_ref = branch_rc.as_ref();
                let branch = branch_ref.borrow();

                if area.intersects(&branch.bounds) {
                    list.push(branch_rc.clone());
                }
            }
            for leaf in tree_borrow.items.iter() {
                if area.intersects(&leaf.bounds) {
                    //QuadTree::log(format!("area {} intersects {}", area, &leaf.bounds));
                    results.push(leaf.clone());
                }
            }

            for leaf in tree_borrow.stuck.iter() {
                if area.intersects(&leaf.bounds) {
                    //QuadTree::log(format!("area {} intersects {}", area, &leaf.bounds));
                    results.push(leaf.clone());
                }
            }
        }
    }

    fn grow(tree: Rc<RefCell<QuadTree>>) {
        let mut this = tree.borrow_mut();

        let size = this.bounds.w;
        let half = size / 2;
        if this.branches[0].is_some() {
            let mut new_tree = QuadTree::new(
                false,
                this.bounds.x - half,
                this.bounds.y - half,
                size,
                Some(Rc::downgrade(&tree)),
            );
            new_tree.branches[2] = mem::replace(&mut this.branches[0], None);
            this.branches[0] = Some(Rc::new(RefCell::new(new_tree)));
        }
        if this.branches[1].is_some() {
            let mut new_tree = QuadTree::new(
                false,
                this.bounds.x + half,
                this.bounds.y - half,
                size,
                Some(Rc::downgrade(&tree)),
            );
            new_tree.branches[3] = mem::replace(&mut this.branches[1], None);
            this.branches[1] = Some(Rc::new(RefCell::new(new_tree)));
        }
        if this.branches[2].is_some() {
            let mut new_tree = QuadTree::new(
                false,
                this.bounds.x + half,
                this.bounds.y + half,
                size,
                Some(Rc::downgrade(&tree)),
            );
            new_tree.branches[0] = mem::replace(&mut this.branches[2], None);
            this.branches[2] = Some(Rc::new(RefCell::new(new_tree)));
        }
        if this.branches[3].is_some() {
            let mut new_tree = QuadTree::new(
                false,
                this.bounds.x - half,
                this.bounds.y + half,
                size,
                Some(Rc::downgrade(&tree)),
            );
            new_tree.branches[1] = mem::replace(&mut this.branches[3], None);
            this.branches[3] = Some(Rc::new(RefCell::new(new_tree)));
        }

        this.bounds.x -= half;
        this.bounds.y -= half;
        this.bounds.w += size;
        this.bounds.h += size;
    }

    pub fn insert(
        tree_rc: Rc<RefCell<QuadTree>>,
        item: i64,
        bounds: QuadTreeBounds,
        level: usize,
    ) {
        //QuadTree::log(format!("inserting item {item} {bounds} level: {level}"));

        let tree_ref = tree_rc.as_ref();

        let this = tree_ref.borrow();

        if this.root {
            //QuadTree::log(format!("root: {level}"));
            if !this.bounds.contains(&bounds) {
                drop(this);
                loop {
                    //QuadTree::log(format!("growing level: {level}"));
                    QuadTree::grow(tree_rc.clone());
                    let this = tree_ref.borrow();
                    if this.bounds.contains(&bounds) {
                        drop(this);
                        break;
                    }
                    drop(this);
                }
            } else {
                drop(this);
            }
        } else {
            drop(this);
        }

        let mut this = tree_ref.borrow_mut();

        let new_leaf = QuadTreeLeaf::new(item, bounds, Rc::downgrade(&tree_rc));
        

        this.items.push(new_leaf);
        if this.items.len() < 2 {
            //QuadTree::log(format!("small list level: {level}"));
            return;
        } else {
            //QuadTree::log(format!("big list level: {level}"));
        }

        //QuadTree::log(format!("clearing list level: {level}"));

        while this.items.len() > 0 {
            //QuadTree::log(format!("working item level: {level}"));
            let mut leaf = this.items.pop().unwrap();

            let index = this.index(&leaf.bounds);

            if index < 0 || this.bounds.w < 16 {
                leaf.parent = Rc::downgrade(&tree_rc);
                this.stuck.push(leaf);
                continue;
            }

            if this.branches[index as usize].is_none() {
                //QuadTree::log(format!("new tree level: {level}"));
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

                this.branches[index as usize] = Some(Rc::new(RefCell::new(QuadTree::new(
                    false,
                    x,
                    y,
                    size,
                    Some(Rc::downgrade(&tree_rc)),
                ))));
            }

            let foo = &this.branches[index as usize];
            let bar = foo.clone().unwrap();

            QuadTree::insert(bar, leaf.identity, leaf.bounds, level + 1);
        }
        //QuadTree::log(format!("done inserting level: {level}"));
    }
}
