use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    key: i32,
    value: i32,
    prev: Option<NodeRef>,
    next: Option<NodeRef>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, NodeRef>,
    head: NodeRef,
    tail: NodeRef,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(Node::new(0, 0)));
        let tail = Rc::new(RefCell::new(Node::new(0, 0)));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            head,
            tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            let node = node.clone();
            let value = node.borrow().value;
            self.move_to_head(node);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            let node = node.clone();
            node.borrow_mut().value = value;
            self.move_to_head(node);
        } else {
            if self.cache.len() >= self.capacity {
                self.remove_tail();
            }
            let node = Rc::new(RefCell::new(Node::new(key, value)));
            self.cache.insert(key, node.clone());
            self.add_to_head(node);
        }
    }

    fn move_to_head(&mut self, node: NodeRef) {
        self.remove_node(node.clone());
        self.add_to_head(node);
    }

    fn remove_node(&mut self, node: NodeRef) {
        let mut node_mut = node.borrow_mut();
        if let Some(prev) = &node_mut.prev {
            if let Some(next) = &node_mut.next {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
            }
        }
    }

    fn add_to_head(&mut self, node: NodeRef) {
        let mut node_mut = node.borrow_mut();
        let head_next = self.head.borrow_mut().next.take();

        node_mut.prev = Some(self.head.clone());
        node_mut.next = head_next.clone();

        if let Some(next) = head_next {
            next.borrow_mut().prev = Some(node.clone());
        }
        self.head.borrow_mut().next = Some(node);
    }

    fn remove_tail(&mut self) {
        let tail_prev = self.tail.borrow_mut().prev.clone();
        if let Some(node) = tail_prev {
            let key = node.borrow().key;
            self.remove_node(node);
            self.cache.remove(&key);
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1));  // 1
    cache.put(3, 3);
    println!("{}", cache.get(2));  // -1
    cache.put(4, 4);
    println!("{}", cache.get(1));  // -1
    println!("{}", cache.get(3));  // 3
    println!("{}", cache.get(4));  // 4
}
