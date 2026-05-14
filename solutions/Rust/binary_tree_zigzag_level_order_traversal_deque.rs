use std::cell::RefCell; use std::collections::VecDeque; use std::rc::Rc;
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Rc<RefCell<TreeNode>>>, pub right: Option<Rc<RefCell<TreeNode>>>, }
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new(); if root.is_none() { return result; }
    let mut q = VecDeque::new(); q.push_back(root.unwrap());
    let mut level = 0;
    while !q.is_empty() { let sz = q.len(); let mut level_vals = VecDeque::new();
        for _ in 0..sz { let n = q.pop_front().unwrap(); let n_ref = n.borrow_mut();
            if level % 2 == 0 { level_vals.push_back(n_ref.val); }
            else { level_vals.push_front(n_ref.val); }
            if let Some(l) = n_ref.left.clone() { q.push_back(l); }
            if let Some(r) = n_ref.right.clone() { q.push_back(r); }
        }
        result.push(level_vals.into_iter().collect());
        level += 1;
    }
    result
}
