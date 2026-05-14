use std::collections::HashMap;

#[derive(Clone)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

/**
 * Determine if string can be segmented using words from dictionary (Trie approach).
 *
 * Time Complexity: O(n*m)
 * Space Complexity: O(n + T)
 */
pub fn word_break_dp_trie(s: String, word_dict: Vec<String>) -> bool {
    let mut root = TrieNode::new();

    for word in word_dict {
        let mut node = &mut root;
        for ch in word.chars() {
            node = &mut node.children.entry(ch).or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..=s.len() {
        if !dp[i] {
            let mut node = &root;
            for j in (0..i).rev() {
                if let Some(next) = node.children.get(&s.chars().nth(j).unwrap()) {
                    node = next;
                    if dp[j] && node.is_end {
                        dp[i] = true;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    dp[s.len()]
}

fn main() {
    println!("{}", word_break_dp_trie("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));  // true
}
