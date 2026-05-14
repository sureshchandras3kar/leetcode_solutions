use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = HashMap::new();
        for ch in s.chars() {
            *count.entry(ch).or_insert(0) += 1;
        }
        for ch in t.chars() {
            if let Some(value) = count.get_mut(&ch) {
                *value -= 1;
                let should_remove = *value == 0;
                if should_remove {
                    count.remove(&ch);
                }
            } else {
                return false;
            }
        }
        count.is_empty()
    }
}
