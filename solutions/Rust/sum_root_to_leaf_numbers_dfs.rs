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
 * Sum all root-to-leaf numbers using DFS.
 * Build number by appending digits as we traverse down.
 * Time: O(n), Space: O(h) for recursion stack
 */
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current_sum: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let node_ref = n.borrow();
                let val = node_ref.val;
                let left = node_ref.left.clone();
                let right = node_ref.right.clone();
                drop(node_ref);

                // Build number: multiply by 10 and add current digit
                let current_sum = current_sum * 10 + val;

                // Leaf node: return the complete number
                if left.is_none() && right.is_none() {
                    return current_sum;
                }

                // Recursively process children and sum
                dfs(left, current_sum) + dfs(right, current_sum)
            }
        }
    }

    dfs(root, 0)
}

fn main() {
    println!("Example 1: Sum root to leaf numbers with DFS");
}
