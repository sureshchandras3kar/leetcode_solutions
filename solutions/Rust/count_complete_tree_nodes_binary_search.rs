use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
 * Count nodes in complete binary tree using binary search on node positions.
 * Uses existence check for node at each possible position.
 * Time: O(log² n), Space: O(log n) for recursion
 */
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let root = root.unwrap();

    // Find height of tree
    let mut height = 0;
    let mut node = Some(root.clone());
    while let Some(n) = node.clone() {
        height += 1;
        let n_ref = n.borrow();
        node = n_ref.left.clone();
    }

    // Binary search on number of nodes
    let mut low = 1i64 << (height - 1);
    let mut high = (1i64 << height) - 1;

    while low <= high {
        let mid = (low + high + 1) / 2;
        if exists(mid, height, Some(root.clone())) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low as i32
}

fn exists(pos: i64, height: i32, mut node: Option<Rc<RefCell<TreeNode>>>) -> bool {
    /**Check if node at position pos exists.*/
    let mut left = 0i64;
    let mut right = (1i64 << (height - 1)) - 1;

    for _ in 0..height - 1 {
        let mid = (left + right + 1) / 2;

        if let Some(n) = node {
            let n_ref = n.borrow();
            if pos >= mid {
                node = n_ref.right.clone();
                left = mid;
            } else {
                node = n_ref.left.clone();
                right = mid - 1;
            }
        }
    }

    node.is_some()
}

fn main() {
    println!("Example 1: Count complete tree nodes with binary search");
}
