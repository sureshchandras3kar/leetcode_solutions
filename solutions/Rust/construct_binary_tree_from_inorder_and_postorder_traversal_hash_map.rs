use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
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
 * Construct binary tree from inorder and postorder traversal using HashMap.
 *
 * Key insight:
 * - Postorder: left subtree, right subtree, root (last element is always root)
 * - Inorder: left subtree, root, right subtree
 *
 * Use a HashMap to quickly find the root's position in inorder,
 * then recursively build left and right subtrees.
 *
 * Time Complexity: O(n) where n is the number of nodes
 * Space Complexity: O(n) for HashMap and recursion call stack
 */
pub fn build_tree(
    inorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() || postorder.is_empty() {
        return None;
    }

    // Build HashMap for O(1) inorder lookup
    let inorder_map: HashMap<i32, usize> = inorder
        .iter()
        .enumerate()
        .map(|(i, &val)| (val, i))
        .collect();

    fn build(
        postorder: &[i32],
        post_start: usize,
        post_end: usize,
        in_start: usize,
        in_end: usize,
        inorder_map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        /**
         * Recursively build tree from postorder and inorder ranges.
         *
         * Args:
         *     post_start, post_end: Range in postorder array
         *     in_start, in_end: Range in inorder array
         */

        if post_start > post_end || in_start > in_end {
            return None;
        }

        // Last element in postorder range is the root
        let root_val = postorder[post_end];
        let mut root = TreeNode::new(root_val);

        // Find root position in inorder
        let root_idx = inorder_map[&root_val];

        // Number of nodes in left subtree
        let left_size = root_idx - in_start;

        // Recursively build left subtree
        root.left = build(
            postorder,
            post_start,
            post_start + left_size - 1,
            in_start,
            root_idx - 1,
            inorder_map,
        );

        // Recursively build right subtree
        root.right = build(
            postorder,
            post_start + left_size,
            post_end - 1,
            root_idx + 1,
            in_end,
            inorder_map,
        );

        Some(Rc::new(RefCell::new(root)))
    }

    build(&postorder, 0, postorder.len() - 1, 0, inorder.len() - 1, &inorder_map)
}

// Helper function for inorder traversal
fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match node {
        None => vec![],
        Some(n) => {
            let n_ref = n.borrow();
            let mut result = inorder_traversal(n_ref.left.clone());
            result.push(n_ref.val);
            result.extend(inorder_traversal(n_ref.right.clone()));
            result
        }
    }
}

// Helper function for postorder traversal
fn postorder_traversal(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match node {
        None => vec![],
        Some(n) => {
            let n_ref = n.borrow();
            let mut result = postorder_traversal(n_ref.left.clone());
            result.extend(postorder_traversal(n_ref.right.clone()));
            result.push(n_ref.val);
            result
        }
    }
}

fn main() {
    // Example 1: [3,9,20,null,null,15,7]
    //     3
    //    / \
    //   9  20
    //      / \
    //     15  7
    let inorder1 = vec![9, 3, 15, 20, 7];
    let postorder1 = vec![9, 15, 7, 20, 3];
    let root1 = build_tree(inorder1, postorder1);

    println!("Inorder: {:?}", inorder_traversal(root1.clone()));   // Expected: [9, 3, 15, 20, 7]
    println!("Postorder: {:?}", postorder_traversal(root1));      // Expected: [9, 15, 7, 20, 3]

    // Example 2: Single node
    let inorder2 = vec![1];
    let postorder2 = vec![1];
    let root2 = build_tree(inorder2, postorder2);
    println!("Single node: {:?}", inorder_traversal(root2));

    // Example 3: Left skewed tree
    let inorder3 = vec![3, 2, 1];
    let postorder3 = vec![3, 2, 1];
    let root3 = build_tree(inorder3, postorder3);
    println!("Left skewed: {:?}", inorder_traversal(root3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let root = build_tree(inorder, postorder);
        assert_eq!(inorder_traversal(root), vec![9, 3, 15, 20, 7]);
    }

    #[test]
    fn test_single_node() {
        let inorder = vec![1];
        let postorder = vec![1];
        let root = build_tree(inorder, postorder);
        assert_eq!(inorder_traversal(root), vec![1]);
    }
}
