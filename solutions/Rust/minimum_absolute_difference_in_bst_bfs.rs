use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn get_minimum_difference_bfs(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut stack = Vec::new();
    let mut current = root;
    let mut prev: Option<i32> = None;
    let mut min_diff = i32::MAX;

    loop {
        // Go to leftmost node
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow_mut().left.take();
        }

        if stack.is_empty() {
            break;
        }

        // Pop from stack
        let node = stack.pop().unwrap();
        let node_ref = node.borrow();

        // Process current node
        if let Some(p) = prev {
            min_diff = min(min_diff, node_ref.val - p);
        }
        prev = Some(node_ref.val);

        // Visit right subtree
        current = node_ref.right.as_ref().cloned();
    }

    min_diff
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(4)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(6)));

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());

    let left_left = Rc::new(RefCell::new(TreeNode::new(1)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(3)));

    left.borrow_mut().left = Some(left_left);
    left.borrow_mut().right = Some(left_right);

    println!("{}", get_minimum_difference_bfs(Some(root)));  // 1
}
