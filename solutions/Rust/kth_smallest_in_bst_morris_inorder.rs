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

pub fn kth_smallest_morris_inorder(
    root: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> i32 {
    if root.is_none() {
        return -1;
    }

    let mut count = 0;
    let mut current = root;

    while let Some(node) = current {
        let mut node_ref = node.borrow_mut();

        if node_ref.left.is_none() {
            // Left is null, process current node
            count += 1;
            if count == k {
                return node_ref.val;
            }
            current = node_ref.right.take();
        } else {
            // Find in-order predecessor
            let left = node_ref.left.take();
            let mut pred = left.clone();

            loop {
                let pred_ref = pred.borrow();
                if pred_ref.right.is_none() || pred_ref.right.as_ref() == Some(&node) {
                    break;
                }
                let next = pred_ref.right.clone();
                drop(pred_ref);
                pred = next.unwrap();
            }

            let mut pred_ref = pred.borrow_mut();
            if pred_ref.right.is_none() {
                // Create link to current node
                pred_ref.right = Some(node.clone());
                node_ref.left = left;
                current = Some(node.clone());
            } else {
                // Link exists, remove it and process current
                pred_ref.right = None;
                drop(pred_ref);
                drop(node_ref);

                count += 1;
                if count == k {
                    return node.borrow().val;
                }

                let right = node.borrow_mut().right.take();
                current = right;
            }
        }
    }

    -1
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let left = Rc::new(RefCell::new(TreeNode::new(1)));
    let right = Rc::new(RefCell::new(TreeNode::new(4)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(2)));

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right);
    left.borrow_mut().right = Some(left_right);

    println!("{}", kth_smallest_morris_inorder(Some(root.clone()), 1));  // 1
    println!("{}", kth_smallest_morris_inorder(Some(root), 3));  // 2
}
