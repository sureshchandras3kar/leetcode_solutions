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

/*
 * Find the maximum depth of a binary tree using DFS (recursive).
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(h) where h is the height (call stack depth)
 */
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + left_depth.max(right_depth)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // [3,9,20,null,null,15,7]
        //       3
        //      / \
        //     9  20
        //       /  \
        //      15   7
        let mut root = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));

        if let Some(ref right) = root.right {
            let mut right_ref = right.borrow_mut();
            right_ref.left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
            right_ref.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        }

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 3);
    }

    #[test]
    fn test_example2() {
        // [1,null,2]
        //     1
        //      \
        //       2
        let mut root = TreeNode::new(1);
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 2);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(max_depth(None), 0);
    }
}
