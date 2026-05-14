use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
     * Find the lowest common ancestor using parent pointers and hash set.
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(h) where h is the height
     */
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parent: HashMap<i32, Option<i32>> = HashMap::new();

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            parent: &mut HashMap<i32, Option<i32>>,
        ) {
            if let Some(n) = node {
                let n_ref = n.borrow();
                parent.insert(n_ref.val, None); // Initialize without parent

                if let Some(left) = &n_ref.left {
                    parent.insert(left.borrow().val, Some(n_ref.val));
                    dfs(&Some(Rc::clone(left)), parent);
                }
                if let Some(right) = &n_ref.right {
                    parent.insert(right.borrow().val, Some(n_ref.val));
                    dfs(&Some(Rc::clone(right)), parent);
                }
            }
        }

        dfs(&root, &mut parent);

        // Collect ancestors of p
        let mut ancestors = HashSet::new();
        if let Some(p_node) = &p {
            let mut current = Some(p_node.borrow().val);
            while let Some(val) = current {
                ancestors.insert(val);
                current = parent.get(&val).and_then(|&p| p);
            }
        }

        // Walk from q upward to find first common ancestor
        if let Some(q_node) = q {
            let mut current = Some(q_node.borrow().val);
            while let Some(val) = current {
                if ancestors.contains(&val) {
                    // Find the node with this value in the tree
                    fn find_node(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
                        if let Some(n) = node {
                            let n_ref = n.borrow();
                            if n_ref.val == val {
                                return Some(Rc::clone(n));
                            }
                            if let Some(found) = find_node(&n_ref.left, val) {
                                return Some(found);
                            }
                            find_node(&n_ref.right, val)
                        } else {
                            None
                        }
                    }

                    return find_node(&root, val);
                }
                current = parent.get(&val).and_then(|&p| p);
            }
        }

        None
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
