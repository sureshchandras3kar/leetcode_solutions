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
 * Check if tree has root-to-leaf path summing to targetSum using recursive DFS.
 * Time: O(n), Space: O(h) for recursion stack
 */
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            let node_ref = node.borrow();
            let val = node_ref.val;
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();
            drop(node_ref);

            // Leaf node check
            if left.is_none() && right.is_none() {
                return val == target_sum;
            }

            // Subtract current value and check left and right subtrees
            let remaining = target_sum - val;
            has_path_sum(left, remaining) || has_path_sum(right, remaining)
        }
    }
}

fn main() {
    println!("Example 1: Path sum check with DFS");
}
