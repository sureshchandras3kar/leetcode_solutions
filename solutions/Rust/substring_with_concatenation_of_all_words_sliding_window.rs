use std::collections::HashMap;

fn find_substring_window_approach(s: &str, words: Vec<&str>) -> Vec<i32> {
    if words.is_empty() || s.is_empty() {
        return vec![];
    }

    let word_len = words[0].len();
    let word_count = words.len();
    let total_len = word_len * word_count;

    // Count frequency of each word
    let mut word_freq: HashMap<&str, i32> = HashMap::new();
    for word in &words {
        *word_freq.entry(word).or_insert(0) += 1;
    }

    let mut result = vec![];

    // For each possible starting position
    for i in 0..=(s.len().saturating_sub(total_len)) {
        let mut window_freq: HashMap<&str, i32> = HashMap::new();

        // Extract and count words in this window
        for j in 0..word_count {
            let start = i + j * word_len;
            let end = start + word_len;
            if end <= s.len() {
                let word = &s[start..end];
                *window_freq.entry(word).or_insert(0) += 1;
            }
        }

        // Check if frequencies match
        if window_freq == word_freq {
            result.push(i as i32);
        }
    }

    result
}

fn main() {
    // Example 1
    let s1 = "barfoothefoobarman";
    let words1 = vec!["foo", "bar"];
    println!("{:?}", find_substring_window_approach(s1, words1)); // [0, 9]

    // Example 2
    let s2 = "wordgoodgoodgoodbestword";
    let words2 = vec!["word", "good", "best", "word"];
    println!("{:?}", find_substring_window_approach(s2, words2)); // []

    // Example 3
    let s3 = "barfoofoobarthefoobarman";
    let words3 = vec!["bar", "foo", "the"];
    println!("{:?}", find_substring_window_approach(s3, words3)); // [6, 9, 12]
}
