use std::cell::RefCell; use std::rc::Rc;
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Rc<RefCell<TreeNode>>>, pub right: Option<Rc<RefCell<TreeNode>>>, }
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let prev = std::cell::RefCell::new(i64::MIN);
    fn dfs(n: &Option<Rc<RefCell<TreeNode>>>, prev: &std::cell::RefCell<i64>) -> bool {
        if let Some(node) = n {
            let node_ref = node.borrow();
            if !dfs(&node_ref.left, prev) { return false; }
            let mut p = prev.borrow_mut();
            if (node_ref.val as i64) <= *p { return false; }
            *p = node_ref.val as i64;
            dfs(&node_ref.right, prev)
        } else { true }
    }
    dfs(&root, &prev)
}
