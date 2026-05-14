use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(0)));
    let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize, usize)> = VecDeque::new();
    queue.push_back((Rc::clone(&root), 0, nums.len() - 1));

    while let Some((node, left, right)) = queue.pop_front() {
        let mid = left + (right - left) / 2;
        node.borrow_mut().val = nums[mid];

        if left + 1 <= mid {
            let new_node = Rc::new(RefCell::new(TreeNode::new(0)));
            node.borrow_mut().left = Some(Rc::clone(&new_node));
            queue.push_back((new_node, left, mid - 1));
        }

        if mid + 1 <= right {
            let new_node = Rc::new(RefCell::new(TreeNode::new(0)));
            node.borrow_mut().right = Some(Rc::clone(&new_node));
            queue.push_back((new_node, mid + 1, right));
        }
    }

    Some(root)
}

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];
    let root = sorted_array_to_bst(nums);
    if let Some(r) = root {
        println!("{}", r.borrow().val);  // 0
    }
}
