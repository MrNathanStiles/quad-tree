use crate::{quad_tree_branch_ts::QuadTreeBranchTs, quad_tree_leaf_ts::QuadTreeLeafTs};



pub struct QuadTreeTs {
    branch: QuadTreeBranchTs,
}

enum QueryResult {
    Leaf(QuadTreeLeafTs),
    Complete,
}

impl QuadTreeTs {
    pub fn new(x: i64, y: i64, half_size: i64) -> Self {
        Self {
            branch: QuadTreeBranchTs::new(true, x, y, half_size.abs() * 2, None)
        }
    }
    pub fn query() {

        

    }
}
