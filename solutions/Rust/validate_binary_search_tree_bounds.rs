use std::cell::RefCell; use std::rc::Rc;
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Rc<RefCell<TreeNode>>>, pub right: Option<Rc<RefCell<TreeNode>>>, }
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(n: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node) = n {
            let node_ref = node.borrow();
            if (node_ref.val as i64) <= min || (node_ref.val as i64) >= max { return false; }
            dfs(&node_ref.left, min, node_ref.val as i64) && dfs(&node_ref.right, node_ref.val as i64, max)
        } else { true }
    }
    dfs(&root, i64::MIN, i64::MAX)
}
