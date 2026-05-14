use std::cell::RefCell;
use std::collections::VecDeque;
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
    if root.is_none() {
        return result;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let level_size = queue.len();

        for i in 0..level_size {
            let node = queue.pop_front().unwrap();
            let node_ref = node.borrow_mut();

            if i == level_size - 1 {
                result.push(node_ref.val);
            }

            if let Some(left) = node_ref.left.clone() {
                queue.push_back(left);
            }
            if let Some(right) = node_ref.right.clone() {
                queue.push_back(right);
            }
        }
    }

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
