use std::cell::RefCell;
use std::collections::HashMap;
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
        head.as_ref()?;

        let mut node_map: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> =
            HashMap::new();

        // First pass: create all copy nodes
        let mut current = head.clone();
        while let Some(node) = current {
            let ptr = node.as_ptr();
            let copy = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            node_map.insert(ptr, copy);
            current = node.borrow_mut().next.take();
        }

        // Second pass: set next and random pointers
        let mut current = head.clone();
        while let Some(node) = current {
            let ptr = node.as_ptr();
            let copy = node_map.get(&ptr).unwrap().clone();

            // Set next pointer
            if let Some(ref next_node) = node.borrow().next {
                let next_ptr = next_node.as_ptr();
                copy.borrow_mut().next =
                    node_map.get(&next_ptr).cloned();
            }

            // Set random pointer
            if let Some(ref random_node) = node.borrow().random {
                let random_ptr = random_node.as_ptr();
                copy.borrow_mut().random = node_map.get(&random_ptr).cloned();
            }

            current = node.borrow_mut().next.take();
        }

        let head_ptr = head.as_ref().unwrap().as_ptr();
        node_map.remove(&head_ptr)
    }
}

pub struct Solution;
