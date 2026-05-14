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

pub fn get_minimum_difference_inorder_dfs(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    let mut min_diff = i32::MAX;
    let mut prev: Option<i32> = None;

    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        min_diff: &mut i32,
        prev: &mut Option<i32>,
    ) {
        if let Some(n) = node {
            let n = n.borrow();

            // Traverse left subtree
            inorder(&n.left, min_diff, prev);

            // Process current node
            if let Some(p) = prev {
                *min_diff = min(*min_diff, n.val - p);
            }
            *prev = Some(n.val);

            // Traverse right subtree
            inorder(&n.right, min_diff, prev);
        }
    }

    inorder(&root, &mut min_diff, &mut prev);
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

    println!("{}", get_minimum_difference_inorder_dfs(Some(root)));  // 1
}
