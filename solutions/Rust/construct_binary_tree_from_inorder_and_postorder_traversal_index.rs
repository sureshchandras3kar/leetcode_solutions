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
 * Construct binary tree from inorder and postorder traversal using index tracking.
 *
 * Key insight:
 * - Postorder: left subtree, right subtree, root (last element is always root)
 * - Inorder: left subtree, root, right subtree
 * - Use a pointer to track the current root in postorder (traverse from end to start)
 * - Find root in inorder to split into left and right subtrees
 *
 * Time Complexity: O(n²) in worst case due to linear search for root in inorder
 * Space Complexity: O(h) for recursion call stack where h is height
 */
pub fn build_tree(
    inorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() || postorder.is_empty() {
        return None;
    }

    let mut post_idx = postorder.len() as i32 - 1;

    fn build(
        postorder: &[i32],
        inorder: &[i32],
        post_idx: &mut i32,
        in_start: usize,
        in_end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        /**
         * Recursively build tree by processing postorder from right to left.
         *
         * Args:
         *     in_start, in_end: Range in inorder array
         */

        if in_start > in_end || *post_idx < 0 {
            return None;
        }

        // Current postorder element (processing from end to start)
        let root_val = postorder[*post_idx as usize];
        *post_idx -= 1;

        let mut root = TreeNode::new(root_val);

        // Find root position in inorder
        let mut root_idx = 0;
        for (i, &val) in inorder.iter().enumerate() {
            if val == root_val {
                root_idx = i;
                break;
            }
        }

        // Build right subtree first (postorder: left, right, root)
        // Since we traverse postorder backwards, right comes before left
        root.right = build(postorder, inorder, post_idx, root_idx + 1, in_end);

        // Build left subtree
        root.left = build(
            postorder,
            inorder,
            post_idx,
            in_start,
            if root_idx > 0 { root_idx - 1 } else { root_idx },
        );

        Some(Rc::new(RefCell::new(root)))
    }

    build(
        &postorder,
        &inorder,
        &mut post_idx,
        0,
        inorder.len() - 1,
    )
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
