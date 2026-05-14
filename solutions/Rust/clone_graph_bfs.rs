use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

fn cloneGraphBFS(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    match node {
        None => None,
        Some(n) => {
            let mut visited = HashMap::new();
            let mut queue = VecDeque::new();
            
            let val = n.borrow().val;
            let cloned = Rc::new(RefCell::new(Node::new(val)));
            visited.insert(val, Rc::clone(&cloned));
            queue.push_back(n);
            
            while let Some(curr) = queue.pop_front() {
                let neighbors = curr.borrow().neighbors.iter().map(|n| Rc::clone(n)).collect::<Vec<_>>();
                
                for neighbor in neighbors {
                    let nval = neighbor.borrow().val;
                    if !visited.contains_key(&nval) {
                        let cloned_neighbor = Rc::new(RefCell::new(Node::new(nval)));
                        visited.insert(nval, Rc::clone(&cloned_neighbor));
                        queue.push_back(neighbor);
                    }
                    visited.get(&curr.borrow().val).unwrap().borrow_mut().neighbors.push(Rc::clone(visited.get(&nval).unwrap()));
                }
            }
            
            Some(visited.get(&n.borrow().val).unwrap().clone())
        }
    }
}
