use std::collections::HashSet;

/**
 * Determine if string can be segmented using words from dictionary.
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n + m)
 */
pub fn word_break_dp_set(s: String, word_dict: Vec<String>) -> bool {
    let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..=s.len() {
        for j in 0..i {
            if dp[j] && word_set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[s.len()]
}

fn main() {
    println!("{}", word_break_dp_set("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));  // true
    println!("{}", word_break_dp_set("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]));  // true
}
