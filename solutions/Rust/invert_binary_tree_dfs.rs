use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
 * DFS Recursive approach to invert a binary tree.
 * Recursively swap left and right children for each node.
 *
 * Time Complexity: O(n) - visit each node once
 * Space Complexity: O(h) - recursion stack depth (h = height)
 */
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.and_then(|node| {
        let mut n = node.borrow_mut();

        // Swap left and right children
        let temp = n.left.take();
        n.left = n.right.take();
        n.right = temp;

        // Recursively invert left and right subtrees
        drop(n);
        invert_tree(node.borrow_mut().left.take());
        invert_tree(node.borrow_mut().right.take());

        Some(node)
    })
}

fn main() {
    // Create tree:     1
    //                /   \
    //               2     3
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let result = invert_tree(root);

    // Expected:        1
    //                /   \
    //               3     2
    if let Some(node) = result {
        let n = node.borrow();
        println!("Root: {}", n.val);  // 1
        if let Some(left) = &n.left {
            if let Some(right) = &n.right {
                println!(
                    "Left: {}, Right: {}",
                    left.borrow().val,
                    right.borrow().val
                );  // 3, 2
            }
        }
    }
}
