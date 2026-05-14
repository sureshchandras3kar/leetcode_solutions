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

pub fn lowest_common_ancestor_iterative(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: &TreeNode,
    q: &TreeNode,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = root;

    while let Some(node) = current {
        let node_ref = node.borrow();

        if node_ref.val > p.val && node_ref.val > q.val {
            // Both p and q are in left subtree
            let left = node_ref.left.clone();
            drop(node_ref);
            current = left;
        } else if node_ref.val < p.val && node_ref.val < q.val {
            // Both p and q are in right subtree
            let right = node_ref.right.clone();
            drop(node_ref);
            current = right;
        } else {
            // p and q are on different sides or one of them is current
            return Some(node);
        }
    }

    None
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(6)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(8)));

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right);

    let left_left = Rc::new(RefCell::new(TreeNode::new(0)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(4)));

    left.borrow_mut().left = Some(left_left);
    left.borrow_mut().right = Some(left_right.clone());

    let left_right_left = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_right_right = Rc::new(RefCell::new(TreeNode::new(5)));

    left_right.borrow_mut().left = Some(left_right_left);
    left_right.borrow_mut().right = Some(left_right_right);

    let p = left.borrow();
    let q = left_right.borrow();
    if let Some(result) = lowest_common_ancestor_iterative(Some(root), &*p, &*q) {
        println!("{}", result.borrow().val);  // 2
    }
}
