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

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n_ref = n.borrow();

            if level == result.len() {
                result.push(n_ref.val);
            }

            dfs(&n_ref.right, level + 1, result);
            dfs(&n_ref.left, level + 1, result);
        }
    }

    dfs(&root, 0, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));
        let left_right = Rc::new(RefCell::new(TreeNode::new(5)));
        let right_right = Rc::new(RefCell::new(TreeNode::new(4)));

        left.borrow_mut().right = Some(left_right);
        right.borrow_mut().right = Some(right_right);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        let result = right_side_view(Some(root));
        assert_eq!(result, vec![1, 3, 4]);
    }
}
