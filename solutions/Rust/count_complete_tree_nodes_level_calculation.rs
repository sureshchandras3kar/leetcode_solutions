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
 * Count nodes in complete binary tree using level calculation.
 * For complete tree, if left height == right height, left is perfect.
 * Time: O(log² n), Space: O(log n) for recursion
 */
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node_ref = node.borrow();

            // Calculate left and right heights
            let mut left_height = 0;
            let mut left_node = node_ref.left.clone();
            while let Some(n) = left_node.clone() {
                left_height += 1;
                let n_ref = n.borrow();
                left_node = n_ref.left.clone();
            }

            let mut right_height = 0;
            let mut right_node = node_ref.right.clone();
            while let Some(n) = right_node.clone() {
                right_height += 1;
                let n_ref = n.borrow();
                right_node = n_ref.right.clone();
            }

            let right = node_ref.right.clone();
            let left = node_ref.left.clone();
            drop(node_ref);

            if left_height == right_height {
                // Left subtree is perfect: 2^(h+1) - 1 nodes + root + recursively count right
                ((1 << (left_height + 1)) - 1) + count_nodes(right)
            } else {
                // Right subtree is perfect: 2^h - 1 nodes + root + recursively count left
                ((1 << (right_height + 1)) - 1) + count_nodes(left)
            }
        }
    }
}

fn main() {
    println!("Example 1: Count complete tree nodes with level calculation");
}
