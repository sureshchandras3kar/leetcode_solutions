use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut sums: Vec<i64> = Vec::new();
    let mut counts: Vec<i32> = Vec::new();

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        level: usize,
        sums: &mut Vec<i64>,
        counts: &mut Vec<i32>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();

            if level == sums.len() {
                sums.push(0);
                counts.push(0);
            }

            sums[level] += n_ref.val as i64;
            counts[level] += 1;

            dfs(&n_ref.left, level + 1, sums, counts);
            dfs(&n_ref.right, level + 1, sums, counts);
        }
    }

    dfs(&root, 0, &mut sums, &mut counts);

    sums.iter()
        .enumerate()
        .map(|(i, &sum)| sum as f64 / counts[i] as f64)
        .collect()
}
