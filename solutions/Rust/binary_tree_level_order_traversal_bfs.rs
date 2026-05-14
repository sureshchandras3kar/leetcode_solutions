use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() { return result; }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    while !queue.is_empty() {
        let sz = queue.len();
        let mut level = Vec::new();
        for _ in 0..sz {
            let n = queue.pop_front().unwrap();
            let n_ref = n.borrow_mut();
            level.push(n_ref.val);
            if let Some(l) = n_ref.left.clone() { queue.push_back(l); }
            if let Some(r) = n_ref.right.clone() { queue.push_back(r); }
        }
        result.push(level);
    }
    result
}
