use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }
}

impl Solution {
    pub fn copy_random_list(
        head: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        let head = head?;

        // First pass: create copies and interleave them
        // original -> copy -> original -> copy -> ...
        let mut current = Some(head.clone());
        while let Some(node) = current {
            let copy = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            let next = node.borrow_mut().next.take();
            node.borrow_mut().next = Some(copy.clone());
            copy.borrow_mut().next = next;
            current = copy.borrow_mut().next.take();
        }

        // Second pass: set random pointers for copy nodes
        let mut current = Some(head.clone());
        while let Some(node) = current {
            let copy = node.borrow().next.clone();
            if let Some(ref copy_node) = copy {
                if let Some(ref random_node) = node.borrow().random {
                    copy_node.borrow_mut().random =
                        random_node.borrow().next.clone();
                }
            }
            current = copy.and_then(|c| c.borrow_mut().next.take());
        }

        // Third pass: restore original list and extract copy list
        let mut current = Some(head.clone());
        let copy_head = head.borrow().next.clone();

        while let Some(node) = current {
            let copy = node.borrow_mut().next.take();
            if let Some(ref copy_node) = copy {
                node.borrow_mut().next = copy_node.borrow_mut().next.take();
            }
            current = node.borrow_mut().next.take();
        }

        copy_head
    }
}

pub struct Solution;
