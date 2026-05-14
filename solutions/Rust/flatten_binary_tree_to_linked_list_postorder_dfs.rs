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
 * Flatten binary tree to linked list using post-order DFS.
 * Uses previous pointer to track last visited node in reverse in-order.
 * Time: O(n), Space: O(h) for recursion stack
 */
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut prev = None;
    dfs(root, &mut prev);
}

fn dfs(
    node: &mut Option<Rc<RefCell<TreeNode>>>,
    prev: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(n) = node.take() {
        let mut n_ref = n.borrow_mut();

        let mut left = n_ref.left.take();
        let mut right = n_ref.right.take();

        drop(n_ref);

        // Post-order: right, left, then process node
        dfs(&mut right, prev);
        dfs(&mut left, prev);

        let mut n_ref = n.borrow_mut();
        n_ref.right = prev.take();
        n_ref.left = None;

        *prev = Some(n);
    }
}

fn main() {
    println!("Example 1: Tree flattened to linked list using post-order DFS");
}
