use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    word: Option<String>,
}

struct Solution;

impl Solution {
    pub fn find_words(board: &mut Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let root = Rc::new(RefCell::new(TrieNode::default()));

        for word in words {
            let mut node = Rc::clone(&root);
            for ch in word.chars() {
                let mut node_ref = node.borrow_mut();
                node = Rc::clone(
                    node_ref
                        .children
                        .entry(ch)
                        .or_insert_with(|| Rc::new(RefCell::new(TrieNode::default()))),
                );
            }
            node.borrow_mut().word = Some(word);
        }

        let mut result = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Solution::dfs(board, i, j, Rc::clone(&root), &mut result);
            }
        }
        result
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        node: Rc<RefCell<TrieNode>>,
        result: &mut Vec<String>,
    ) {
        let ch = board[i][j];
        let mut node_ref = node.borrow_mut();

        if !node_ref.children.contains_key(&ch) {
            return;
        }

        let next_node = Rc::clone(node_ref.children.get(&ch).unwrap());
        drop(node_ref);

        let mut next_ref = next_node.borrow_mut();
        if let Some(word) = next_ref.word.take() {
            result.push(word);
        }
        drop(next_ref);

        board[i][j] = '#';
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (di, dj) in &dirs {
            let ni = (i as i32 + di) as usize;
            let nj = (j as i32 + dj) as usize;
            if ni < board.len() && nj < board[0].len() {
                Solution::dfs(board, ni, nj, Rc::clone(&next_node), result);
            }
        }
        board[i][j] = ch;
    }
}

fn main() {
    let mut board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()];
    let result = Solution::find_words(&mut board, words);
    println!("{:?}", result);
}
