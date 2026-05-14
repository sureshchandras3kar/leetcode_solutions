use std::rc::Rc;
use std::cell::RefCell;
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

pub struct Solution;

impl Solution {
    /// Check if a binary tree is symmetric using BFS with a queue.
    ///
    /// Time Complexity: O(n) - visit each node once
    /// Space Complexity: O(w) - queue width, where w is max nodes at any level
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let mut queue: VecDeque<(
                    Option<Rc<RefCell<TreeNode>>>,
                    Option<Rc<RefCell<TreeNode>>>,
                )> = VecDeque::new();

                queue.push_back((node.left.clone(), node.right.clone()));

                while let Some((left_opt, right_opt)) = queue.pop_front() {
                    match (left_opt, right_opt) {
                        // Both nodes are None - continue (symmetric so far)
                        (None, None) => continue,
                        // One node is None - not symmetric
                        (None, Some(_)) | (Some(_), None) => return false,
                        // Both nodes exist - check values and enqueue next level
                        (Some(l), Some(r)) => {
                            let l_node = l.borrow();
                            let r_node = r.borrow();

                            if l_node.val != r_node.val {
                                return false;
                            }

                            // Add pairs for next level: left's left with right's right
                            // and left's right with right's left (mirror pattern)
                            queue.push_back((l_node.left.clone(), r_node.right.clone()));
                            queue.push_back((l_node.right.clone(), r_node.left.clone()));
                        }
                    }
                }

                true
            }
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

    #[test]
    fn test_single_node() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::is_symmetric(root), true);
    }
}
