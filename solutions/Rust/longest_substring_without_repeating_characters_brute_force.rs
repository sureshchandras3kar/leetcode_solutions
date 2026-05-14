use std::collections::HashSet;

fn longest_substring_without_repeating_characters_brute_force(s: &str) -> usize {
    let mut max_length = 0;

    for i in 0..s.len() {
        let mut char_set: HashSet<char> = HashSet::new();
        for (j, c) in s.chars().enumerate().skip(i) {
            if char_set.contains(&c) {
                break;
            }
            char_set.insert(c);
            max_length = max_length.max(j - i + 1);
        }
    }

    max_length
}

fn main() {
    let s = "abcabcbb";
    let result = longest_substring_without_repeating_characters_brute_force(s);
    println!("{}", result);
}
