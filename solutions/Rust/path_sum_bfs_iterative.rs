use std::cell::RefCell;
use std::collections::VecDeque;
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
 * Check if tree has root-to-leaf path summing to targetSum using iterative BFS.
 * Queue stores (node, current_sum) pairs.
 * Time: O(n), Space: O(w) where w is max width
 */
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }

    let mut queue = VecDeque::new();
    queue.push_back((root.clone().unwrap(), root.unwrap().borrow().val));

    while !queue.is_empty() {
        let (node, current_sum) = queue.pop_front().unwrap();
        let node_ref = node.borrow();

        // Check leaf node
        if node_ref.left.is_none() && node_ref.right.is_none() && current_sum == target_sum {
            return true;
        }

        if let Some(left) = node_ref.left.clone() {
            queue.push_back((left.clone(), current_sum + left.borrow().val));
        }
        if let Some(right) = node_ref.right.clone() {
            queue.push_back((right.clone(), current_sum + right.borrow().val));
        }
    }

    false
}

fn main() {
    println!("Example 1: Path sum check with BFS");
}
