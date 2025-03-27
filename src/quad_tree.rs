use crate::{quad_tree_branch::QuadTreeBranch, quad_tree_leaf::QuadTreeLeaf};

pub struct QuadTree {
    branch: QuadTreeBranch,
}
enum QueryResult {
    Leaf(QuadTreeLeaf),
    Complete,
}
impl QuadTree {
    pub fn new(x: i64, y: i64, half_size: i64) -> Self {
        Self {
            branch: QuadTreeBranch::new(true, x, y, half_size.abs() * 2, None)
        }
    }
    pub fn query() {

        

    }
}
