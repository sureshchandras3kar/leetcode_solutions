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

/// Check if two binary trees are the same using BFS (level-order traversal).
///
/// Time Complexity: O(min(m, n)) where m and n are the number of nodes
/// Space Complexity: O(min(w1, w2)) where w1 and w2 are the widths
pub fn is_same_tree(
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((p, q));

    while let Some((node1, node2)) = queue.pop_front() {
        match (node1, node2) {
            // Both are None (equal)
            (None, None) => continue,
            // One is None or values differ (not equal)
            (None, Some(_)) | (Some(_), None) => return false,
            // Both are Some, check value and add children to queue
            (Some(p_node), Some(q_node)) => {
                let p_ref = p_node.borrow();
                let q_ref = q_node.borrow();

                // Values must match
                if p_ref.val != q_ref.val {
                    return false;
                }

                // Add children to queue for comparison
                queue.push_back((p_ref.left.clone(), q_ref.left.clone()));
                queue.push_back((p_ref.right.clone(), q_ref.right.clone()));
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_trees() {
        // Example 1: Same trees
        //     1           1
        //    / \         / \
        //   2   3       2   3
        let p1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        let q1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(is_same_tree(p1, q1), true);
    }

    #[test]
    fn test_different_structure() {
        // Example 2: Different structure
        //     1           1
        //    /             \
        //   2               2
        let p2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));

        let q2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));

        assert_eq!(is_same_tree(p2, q2), false);
    }

    #[test]
    fn test_different_values() {
        // Example 3: Different values
        //     1           1
        //    / \         / \
        //   2   1       1   2
        let p3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        })));

        let q3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));

        assert_eq!(is_same_tree(p3, q3), false);
    }

    #[test]
    fn test_both_empty() {
        // Example 4: Both empty
        assert_eq!(is_same_tree(None, None), true);
    }
}
