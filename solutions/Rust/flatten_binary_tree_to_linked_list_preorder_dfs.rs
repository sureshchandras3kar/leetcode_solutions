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
 * Flatten binary tree to linked list using pre-order DFS.
 * Recursively flattens left and right subtrees, then rewires pointers.
 * Time: O(n), Space: O(h) for recursion stack
 */
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let mut node_ref = node.borrow_mut();

        let mut left = node_ref.left.take();
        let mut right = node_ref.right.take();

        drop(node_ref);

        flatten(&mut left);
        flatten(&mut right);

        let mut node_ref = node.borrow_mut();

        if let Some(mut l) = left {
            // Find rightmost node in flattened left subtree
            let mut rightmost = l.clone();
            loop {
                let rightmost_ref = rightmost.borrow();
                if rightmost_ref.right.is_none() {
                    break;
                }
                let next = rightmost_ref.right.clone().unwrap();
                drop(rightmost_ref);
                rightmost = next;
            }

            // Attach right subtree to rightmost node
            rightmost.borrow_mut().right = right;
            // Move flattened left subtree to right
            node_ref.right = Some(l);
        } else {
            node_ref.right = right;
        }
    }
}

fn main() {
    println!("Example 1: Tree flattened to linked list using pre-order DFS");
}
