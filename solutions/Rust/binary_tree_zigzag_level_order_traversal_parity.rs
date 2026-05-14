use std::cell::RefCell; use std::rc::Rc;
#[derive(Debug)]
pub struct TreeNode { pub val: i32, pub left: Option<Rc<RefCell<TreeNode>>>, pub right: Option<Rc<RefCell<TreeNode>>>, }
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    fn dfs(n: &Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<Vec<i32>>) {
        if let Some(node) = n {
            let node_ref = node.borrow();
            if level == result.len() { result.push(Vec::new()); }
            if level % 2 == 0 { result[level].push(node_ref.val); }
            else { result[level].insert(0, node_ref.val); }
            dfs(&node_ref.left, level + 1, result);
            dfs(&node_ref.right, level + 1, result);
        }
    }
    dfs(&root, 0, &mut result);
    result
}
