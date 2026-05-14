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
 * Populates next pointers using pre-computed links without queue.
 * Uses next pointers of parent level to traverse current level.
 * Time: O(n), Space: O(1)
 */
pub fn connect(mut root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if root.is_none() {
        return None;
    }

    let mut leftmost = root.clone();

    while let Some(lm) = leftmost.clone() {
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        let mut current = Some(lm.clone());

        while let Some(curr) = current.clone() {
            let curr_ref = curr.borrow();

            if let Some(left) = curr_ref.left.clone() {
                if let Some(p) = prev.clone() {
                    p.borrow_mut().next = Some(left.clone());
                }
                prev = Some(left);
            }

            if let Some(right) = curr_ref.right.clone() {
                if let Some(p) = prev.clone() {
                    p.borrow_mut().next = Some(right.clone());
                }
                prev = Some(right);
            }

            current = curr_ref.next.clone();
        }

        let lm_ref = lm.borrow();
        leftmost = lm_ref.left.clone();
    }

    root
}

fn main() {
    println!("Example 1: Tree with next pointers connected via pre-computed links");
}
