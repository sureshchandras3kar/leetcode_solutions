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
 * Binary Search Tree Iterator using stack for in-order traversal.
 * Implements lazy evaluation: next() O(1) amortized, hasNext() O(1).
 * Space: O(h) where h is height
 */
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iterator = BSTIterator { stack: Vec::new() };
        iterator.push_left(root);
        iterator
    }

    fn push_left(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
        /**Push all left nodes onto stack.*/
        while let Some(n) = node {
            let n_ref = n.borrow();
            let left = n_ref.left.clone();
            drop(n_ref);
            self.stack.push(n);
            node = left;
        }
    }

    /**
     * Return next smallest element.
     * Time: O(1) amortized
     */
    pub fn next(&mut self) -> i32 {
        if let Some(node) = self.stack.pop() {
            let node_ref = node.borrow();
            let val = node_ref.val;
            let right = node_ref.right.clone();
            drop(node_ref);

            self.push_left(right);
            val
        } else {
            -1
        }
    }

    /**
     * Check if there are more elements.
     * Time: O(1)
     */
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

fn main() {
    println!("Example 1: BST Iterator using stack");
}
