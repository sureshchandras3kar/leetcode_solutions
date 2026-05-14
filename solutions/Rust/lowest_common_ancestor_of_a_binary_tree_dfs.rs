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

impl Solution {
    /**
     * Find the lowest common ancestor using DFS recursion.
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(h) where h is the height (call stack depth)
     */
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let mut n = node.borrow_mut();

                // If either p or q is the current node, it's a potential LCA
                if let Some(ref p_node) = p {
                    if p_node.borrow().val == n.val {
                        return Some(Rc::clone(&node));
                    }
                }
                if let Some(ref q_node) = q {
                    if q_node.borrow().val == n.val {
                        return Some(Rc::clone(&node));
                    }
                }

                // Search in left and right subtrees
                let left = Solution::lowest_common_ancestor(
                    n.left.take(),
                    p.clone(),
                    q.clone(),
                );
                let right = Solution::lowest_common_ancestor(
                    n.right.take(),
                    p.clone(),
                    q.clone(),
                );

                // If both p and q are found in different subtrees, root is LCA
                match (&left, &right) {
                    (Some(_), Some(_)) => Some(Rc::clone(&node)),
                    (Some(l), None) => Some(Rc::clone(l)),
                    (None, Some(r)) => Some(Rc::clone(r)),
                    (None, None) => None,
                }
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Example 1: [3,5,1,6,2,0,8,null,null,7,4]
        //       3
        //      / \
        //     5   1
        //    / \ / \
        //   6  2 0  8
        //     / \
        //    7   4
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));

        root.borrow_mut().left = Some(Rc::clone(&node5));
        root.borrow_mut().right = Some(Rc::clone(&node1));

        let result = Solution::lowest_common_ancestor(
            Some(Rc::clone(&root)),
            Some(Rc::clone(&node5)),
            Some(Rc::clone(&node1)),
        );

        assert_eq!(result.map(|r| r.borrow().val), Some(3));
    }
}
