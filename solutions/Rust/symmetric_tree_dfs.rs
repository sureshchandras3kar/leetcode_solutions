use std::rc::Rc;
use std::cell::RefCell;

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

pub struct Solution;

impl Solution {
    /// Check if a binary tree is symmetric using DFS recursion.
    ///
    /// Time Complexity: O(n) - visit each node once
    /// Space Complexity: O(h) - recursion stack, where h is height
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                // Both nodes are None - symmetric
                (None, None) => true,
                // One node is None - not symmetric
                (None, Some(_)) | (Some(_), None) => false,
                // Both nodes exist - check values and recurse
                (Some(l), Some(r)) => {
                    let l_node = l.borrow();
                    let r_node = r.borrow();

                    if l_node.val != r_node.val {
                        return false;
                    }

                    // Recursively check: left's left with right's right
                    // and left's right with right's left (mirror pattern)
                    is_mirror(&l_node.left, &r_node.right)
                        && is_mirror(&l_node.right, &r_node.left)
                }
            }
        }

        match root {
            Some(node) => {
                let node = node.borrow();
                is_mirror(&node.left, &node.right)
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_tree() {
        // Example 1: Symmetric tree
        //       1
        //      / \
        //     2   2
        //    / \ / \
        //   3  4 4  3
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));

        assert_eq!(Solution::is_symmetric(root), true);
    }

    #[test]
    fn test_not_symmetric_tree() {
        // Example 2: Not symmetric
        //       1
        //      / \
        //     2   2
        //      \   \
        //       3   3
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));

        assert_eq!(Solution::is_symmetric(root), false);
    }
}
