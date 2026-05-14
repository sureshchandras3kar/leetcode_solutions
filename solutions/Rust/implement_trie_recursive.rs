use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    is_end_of_word: bool,
}

struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode::default())),
        }
    }

    fn insert(&self, word: String) {
        let mut node = Rc::clone(&self.root);
        for ch in word.chars() {
            let mut node_ref = node.borrow_mut();
            node = Rc::clone(
                node_ref
                    .children
                    .entry(ch)
                    .or_insert_with(|| Rc::new(RefCell::new(TrieNode::default()))),
            );
        }
        node.borrow_mut().is_end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        if let Some(node) = self.find_node(word) {
            node.borrow().is_end_of_word
        } else {
            false
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, prefix: String) -> Option<Rc<RefCell<TrieNode>>> {
        let mut node = Rc::clone(&self.root);
        for ch in prefix.chars() {
            let node_ref = node.borrow();
            if let Some(child) = node_ref.children.get(&ch) {
                node = Rc::clone(child);
            } else {
                return None;
            }
        }
        Some(node)
    }
}

fn main() {
    let trie = Trie::new();
    trie.insert("apple".to_string());
    println!("{}", trie.search("apple".to_string()));    // true
    println!("{}", trie.search("app".to_string()));      // false
    println!("{}", trie.starts_with("app".to_string())); // true
}
