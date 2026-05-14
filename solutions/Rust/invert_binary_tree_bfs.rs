use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

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
 * BFS Iterative approach to invert a binary tree.
 * Uses a queue to visit nodes level by level, swapping children.
 *
 * Time Complexity: O(n) - visit each node once
 * Space Complexity: O(w) - w = max width of tree (nodes at widest level)
 */
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    while let Some(Some(node)) = queue.pop_front() {
        let mut n = node.borrow_mut();

        // Swap left and right children
        let temp = n.left.take();
        n.left = n.right.take();
        n.right = temp;

        // Add children to queue for processing
        if let Some(left) = n.left.take() {
            queue.push_back(Some(left));
        }
        if let Some(right) = n.right.take() {
            queue.push_back(Some(right));
        }
    }

    root
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
