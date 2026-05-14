use std::cell::RefCell;
use std::collections::VecDeque;
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
 * Find the maximum depth of a binary tree using BFS (level-order traversal).
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(w) where w is the maximum width of the tree
 */
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            let mut depth = 0;

            while !queue.is_empty() {
                depth += 1;
                let level_size = queue.len();

                for _ in 0..level_size {
                    if let Some(current) = queue.pop_front() {
                        let current_ref = current.borrow();
                        if let Some(left) = current_ref.left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = current_ref.right.clone() {
                            queue.push_back(right);
                        }
                    }
                }
            }

            depth
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
