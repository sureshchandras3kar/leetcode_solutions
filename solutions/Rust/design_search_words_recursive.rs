use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    is_end_of_word: bool,
}

struct WordDictionary {
    root: Rc<RefCell<TrieNode>>,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Rc::new(RefCell::new(TrieNode::default())),
        }
    }

    fn add_word(&self, word: String) {
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
        self.search_dfs(&word, 0, Rc::clone(&self.root))
    }

    fn search_dfs(&self, word: &str, index: usize, node: Rc<RefCell<TrieNode>>) -> bool {
        if index == word.len() {
            return node.borrow().is_end_of_word;
        }

        let chars: Vec<char> = word.chars().collect();
        let ch = chars[index];

        if ch == '.' {
            let node_ref = node.borrow();
            for child in node_ref.children.values() {
                if self.search_dfs(word, index + 1, Rc::clone(child)) {
                    return true;
                }
            }
            false
        } else {
            let node_ref = node.borrow();
            if let Some(child) = node_ref.children.get(&ch) {
                self.search_dfs(word, index + 1, Rc::clone(child))
            } else {
                false
            }
        }
    }
}

fn main() {
    let wd = WordDictionary::new();
    wd.add_word("bad".to_string());
    wd.add_word("dad".to_string());
    wd.add_word("mad".to_string());
    println!("{}", wd.search("pad".to_string()));  // false
    println!("{}", wd.search("bad".to_string()));  // true
    println!("{}", wd.search(".ad".to_string()));  // true
    println!("{}", wd.search("b..".to_string()));  // true
}
