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
 * Find maximum path sum using DFS with mutable max tracking.
 * Maintains maxSum as mutable reference updated during traversal.
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

            // Max single-path sum extending from this node
            let left_sum = dfs(left, max_sum).max(0);
            let right_sum = dfs(right, max_sum).max(0);

            // Path bending at this node
            let path_sum = val + left_sum + right_sum;
            *max_sum = (*max_sum).max(path_sum);

            // Return best single path extending downward
            val + left_sum.max(right_sum)
        }
    }
}

fn main() {
    println!("Example 1: Binary tree maximum path sum with max tracking");
}
