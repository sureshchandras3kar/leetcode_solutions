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
 * Sum all root-to-leaf numbers using iterative BFS.
 * Queue stores (node, current_number) pairs.
 * Time: O(n), Space: O(w) where w is max width
 */
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut queue = VecDeque::new();
    let root_node = root.unwrap();
    let val = root_node.borrow().val;
    queue.push_back((root_node, val));
    let mut total = 0;

    while !queue.is_empty() {
        let (node, current_sum) = queue.pop_front().unwrap();
        let node_ref = node.borrow();

        // Leaf node: add to total
        if node_ref.left.is_none() && node_ref.right.is_none() {
            total += current_sum;
            continue;
        }

        if let Some(left) = node_ref.left.clone() {
            let left_val = left.borrow().val;
            queue.push_back((left, current_sum * 10 + left_val));
        }
        if let Some(right) = node_ref.right.clone() {
            let right_val = right.borrow().val;
            queue.push_back((right, current_sum * 10 + right_val));
        }
    }

    total
}

fn main() {
    println!("Example 1: Sum root to leaf numbers with BFS");
}
