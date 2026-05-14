use std::collections::HashMap;
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

        // Create a map for quick lookup of indices in inorder
        let inorder_map: HashMap<i32, usize> = inorder
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();

        fn build(
            preorder: &[i32],
            inorder: &[i32],
            preorder_start: usize,
            preorder_end: usize,
            inorder_start: usize,
            inorder_end: usize,
            inorder_map: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder_start > preorder_end || inorder_start > inorder_end {
                return None;
            }

            // Root is the first element in preorder
            let root_val = preorder[preorder_start];
            let mut root = TreeNode::new(root_val);

            // Find root position in inorder
            let root_inorder_idx = *inorder_map.get(&root_val)?;

            // Number of elements in left subtree
            let left_size = root_inorder_idx - inorder_start;

            // Build left subtree
            root.left = build(
                preorder,
                inorder,
                preorder_start + 1,
                preorder_start + left_size,
                inorder_start,
                root_inorder_idx.saturating_sub(1),
                inorder_map,
            );

            // Build right subtree
            root.right = build(
                preorder,
                inorder,
                preorder_start + left_size + 1,
                preorder_end,
                root_inorder_idx + 1,
                inorder_end,
                inorder_map,
            );

            Some(Rc::new(RefCell::new(root)))
        }

        build(
            &preorder,
            &inorder,
            0,
            preorder.len() - 1,
            0,
            inorder.len() - 1,
            &inorder_map,
        )
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
