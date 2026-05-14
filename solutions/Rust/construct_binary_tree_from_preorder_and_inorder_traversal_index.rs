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

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let mut preorder_idx = 0;

        fn build(
            preorder: &[i32],
            inorder: &[i32],
            inorder_start: usize,
            inorder_end: usize,
            preorder_idx: &mut usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder_start > inorder_end || *preorder_idx >= preorder.len() {
                return None;
            }

            // Root is the current element in preorder
            let root_val = preorder[*preorder_idx];
            *preorder_idx += 1;
            let mut root = TreeNode::new(root_val);

            // Find root position in inorder
            let mut root_inorder_idx = inorder_start;
            for (i, &val) in inorder[inorder_start..=inorder_end].iter().enumerate() {
                if val == root_val {
                    root_inorder_idx = inorder_start + i;
                    break;
                }
            }

            // Build left subtree
            root.left = build(
                preorder,
                inorder,
                inorder_start,
                root_inorder_idx.saturating_sub(1),
                preorder_idx,
            );

            // Build right subtree
            root.right = build(
                preorder,
                inorder,
                root_inorder_idx + 1,
                inorder_end,
                preorder_idx,
            );

            Some(Rc::new(RefCell::new(root)))
        }

        build(&preorder, &inorder, 0, inorder.len() - 1, &mut preorder_idx)
    }
}

fn main() {
    // Test 1: [3,9,20,15,7], [9,3,15,20,7]
    let preorder1 = vec![3, 9, 20, 15, 7];
    let inorder1 = vec![9, 3, 15, 20, 7];
    let root1 = Solution::build_tree(preorder1, inorder1);
    if let Some(node) = root1 {
        println!("Test 1 - Root: {}", node.borrow().val);  // 3
    }

    // Test 2: [1], [1]
    let preorder2 = vec![1];
    let inorder2 = vec![1];
    let root2 = Solution::build_tree(preorder2, inorder2);
    if let Some(node) = root2 {
        println!("Test 2 - Root: {}", node.borrow().val);  // 1
    }
}
