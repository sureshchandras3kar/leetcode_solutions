use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

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

fn cloneGraphDFS(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    match node {
        None => None,
        Some(n) => {
            let mut visited = HashMap::new();
            Some(dfs(n, &mut visited))
        }
    }
}

fn dfs(
    node: Rc<RefCell<Node>>,
    visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
) -> Rc<RefCell<Node>> {
    let val = node.borrow().val;
    
    if let Some(cloned) = visited.get(&val) {
        return Rc::clone(cloned);
    }
    
    let cloned = Rc::new(RefCell::new(Node::new(val)));
    visited.insert(val, Rc::clone(&cloned));
    
    let neighbors = node.borrow().neighbors.iter().map(|n| dfs(Rc::clone(n), visited)).collect();
    cloned.borrow_mut().neighbors = neighbors;
    
    cloned
}
