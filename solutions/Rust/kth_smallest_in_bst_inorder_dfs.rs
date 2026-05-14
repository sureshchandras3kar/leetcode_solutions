use std::rc::Rc;
use std::cell::RefCell;

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

pub fn kth_smallest_inorder_dfs(
    root: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> i32 {
    let mut count = 0;
    let mut result: Option<i32> = None;

    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        count: &mut i32,
        result: &mut Option<i32>,
        k: i32,
    ) {
        if result.is_some() {
            return;
        }

        if let Some(n) = node {
            let n = n.borrow();

            // Traverse left subtree
            inorder(&n.left, count, result, k);

            // Process current node
            *count += 1;
            if *count == k {
                *result = Some(n.val);
                return;
            }

            // Traverse right subtree
            inorder(&n.right, count, result, k);
        }
    }

    inorder(&root, &mut count, &mut result, k);
    result.unwrap_or(-1)
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let left = Rc::new(RefCell::new(TreeNode::new(1)));
    let right = Rc::new(RefCell::new(TreeNode::new(4)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(2)));

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right);
    left.borrow_mut().right = Some(left_right);

    println!("{}", kth_smallest_inorder_dfs(Some(root.clone()), 1));  // 1
    println!("{}", kth_smallest_inorder_dfs(Some(root), 3));  // 2
}
