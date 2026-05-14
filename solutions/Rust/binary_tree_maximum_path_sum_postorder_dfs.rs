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
 * Find maximum path sum in binary tree using post-order DFS.
 * A path can pass through any node (not just root to leaf).
 * Time: O(n), Space: O(h) for recursion stack
 */
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = i64::MIN;
    dfs(root, &mut max_sum);
    max_sum as i32
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i64) -> i64 {
    match node {
        None => 0,
        Some(n) => {
            let node_ref = n.borrow();
            let val = node_ref.val as i64;
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();
            drop(node_ref);

            // Max gain from left and right subtrees (at least 0 if negative)
            let left_gain = dfs(left, max_sum).max(0);
            let right_gain = dfs(right, max_sum).max(0);

            // Max path through this node (may bend at this node)
            let max_path_through_node = val + left_gain + right_gain;
            *max_sum = (*max_sum).max(max_path_through_node);

            // Return max path extending downward from this node
            val + left_gain.max(right_gain)
        }
    }
}

fn main() {
    println!("Example 1: Binary tree maximum path sum with DFS");
}
