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

fn kth_smallest_inorder(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut result = (0, -1);

    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut (i32, i32), k: i32) {
        if let Some(n) = node {
            if result.1 != -1 {
                return;
            }

            let n = n.borrow();

            inorder(n.left.clone(), result, k);

            result.0 += 1;
            if result.0 == k {
                result.1 = n.val;
                return;
            }

            inorder(n.right.clone(), result, k);
        }
    }

    inorder(root, &mut result, k);
    result.1
}

// Test
fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(2))));

    println!("{}", kth_smallest_inorder(Some(root), 1));  // 1
}
