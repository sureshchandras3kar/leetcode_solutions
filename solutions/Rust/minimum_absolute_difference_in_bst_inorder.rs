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

fn get_minimum_difference_inorder(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = (i32::MAX, None);

    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut (i32, Option<i32>)) {
        if let Some(n) = node {
            let n = n.borrow();

            inorder(n.left.clone(), result);

            if let Some(prev) = result.1 {
                result.0 = result.0.min(n.val - prev);
            }
            result.1 = Some(n.val);

            inorder(n.right.clone(), result);
        }
    }

    inorder(root, &mut result);
    result.0
}

// Test
fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(4)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().left =
        Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(3))));

    println!("{}", get_minimum_difference_inorder(Some(root)));  // 1
}
