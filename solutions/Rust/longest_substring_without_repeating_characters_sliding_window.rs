use std::collections::HashMap;

fn longest_substring_without_repeating_characters_sliding_window(s: &str) -> usize {
    let mut char_index: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut left = 0;

    for (right, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_index.get(&c) {
            if prev_index >= left {
                left = prev_index + 1;
            }
        }
        char_index.insert(c, right);
        max_length = max_length.max(right - left + 1);
    }

    max_length
}

fn main() {
    let s = "abcabcbb";
    let result = longest_substring_without_repeating_characters_sliding_window(s);
    println!("{}", result);
}
