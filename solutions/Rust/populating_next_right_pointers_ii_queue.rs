use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

/**
 * Populates next pointers using level-order BFS queue.
 * Time: O(n), Space: O(w) where w is max width
 */
pub fn connect(mut root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if root.is_none() {
        return None;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut prev: Option<Rc<RefCell<Node>>> = None;

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                if let Some(p) = prev.clone() {
                    p.borrow_mut().next = Some(node.clone());
                }
                prev = Some(node.clone());

                let node_ref = node.borrow();
                if let Some(left) = node_ref.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node_ref.right.clone() {
                    queue.push_back(right);
                }
            }
        }
    }

    root
}

fn main() {
    println!("Example 1: Tree with next pointers connected via queue");
}
