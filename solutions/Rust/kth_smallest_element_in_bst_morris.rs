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

fn kth_smallest_morris(mut root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut count = 0;

    while let Some(current) = root {
        let mut curr_borrow = current.borrow_mut();

        if curr_borrow.left.is_none() {
            count += 1;
            if count == k {
                return curr_borrow.val;
            }
            root = curr_borrow.right.take();
        } else {
            let left = curr_borrow.left.take().unwrap();
            let mut predecessor = left.clone();

            loop {
                let mut pred_borrow = predecessor.borrow_mut();
                if pred_borrow.right.is_none() || matches!(&pred_borrow.right, Some(r) if Rc::ptr_eq(r, &current)) {
                    break;
                }
                let right = pred_borrow.right.take().unwrap();
                drop(pred_borrow);
                predecessor = right;
            }

            let mut pred_borrow = predecessor.borrow_mut();
            if pred_borrow.right.is_none() {
                pred_borrow.right = Some(current.clone());
                drop(pred_borrow);
                drop(curr_borrow);
                root = Some(left);
            } else {
                pred_borrow.right = None;
                count += 1;
                if count == k {
                    return curr_borrow.val;
                }
                drop(pred_borrow);
                root = curr_borrow.right.take();
            }
        }
    }

    -1
}

// Test
fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(2))));

    println!("{}", kth_smallest_morris(Some(root), 1));  // 1
}
