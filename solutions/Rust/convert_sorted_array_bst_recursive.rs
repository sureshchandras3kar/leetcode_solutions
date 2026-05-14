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

fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build(nums: &Vec<i32>, left: usize, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left as i32 > right {
            return None;
        }

        let mid = left + ((right as usize - left) / 2);
        let mut node = TreeNode::new(nums[mid]);
        node.left = build(nums, left, mid as i32 - 1);
        node.right = build(nums, mid + 1, right);

        Some(Rc::new(RefCell::new(node)))
    }

    if nums.is_empty() {
        None
    } else {
        build(&nums, 0, nums.len() as i32 - 1)
    }
}

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];
    let root = sorted_array_to_bst(nums);
    if let Some(r) = root {
        println!("{}", r.borrow().val);  // 0
    }
}
