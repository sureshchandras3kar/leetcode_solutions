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
 * Binary Search Tree Iterator using pre-computed Vec.
 * Stores all in-order elements upfront.
 * Space: O(n), next() O(1), hasNext() O(1)
 */
pub struct BSTIterator {
    arr: Vec<i32>,
    index: usize,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut arr = Vec::new();
        BSTIterator::inorder(root, &mut arr);
        BSTIterator { arr, index: 0 }
    }

    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        /**Pre-compute in-order traversal into array.*/
        if let Some(n) = node {
            let node_ref = n.borrow();
            let left = node_ref.left.clone();
            let val = node_ref.val;
            let right = node_ref.right.clone();
            drop(node_ref);

            Self::inorder(left, arr);
            arr.push(val);
            Self::inorder(right, arr);
        }
    }

    /**
     * Return next smallest element.
     * Time: O(1)
     */
    pub fn next(&mut self) -> i32 {
        let val = self.arr[self.index];
        self.index += 1;
        val
    }

    /**
     * Check if there are more elements.
     * Time: O(1)
     */
    pub fn has_next(&self) -> bool {
        self.index < self.arr.len()
    }
}

fn main() {
    println!("Example 1: BST Iterator using ArrayList");
}
