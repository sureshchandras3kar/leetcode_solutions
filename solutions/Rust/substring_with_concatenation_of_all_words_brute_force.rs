fn find_substring_brute_force(s: &str, words: Vec<&str>) -> Vec<i32> {
    if words.is_empty() || s.is_empty() {
        return vec![];
    }

    let word_len = words[0].len();
    let word_count = words.len();
    let total_len = word_len * word_count;

    let mut result = vec![];

    // For each possible starting position in the string
    for i in 0..=(s.len().saturating_sub(total_len)) {
        // Extract the substring of exact length
        if i + total_len > s.len() {
            break;
        }
        let substring = &s[i..i + total_len];

        // Check if this substring can be formed by concatenating all words
        let mut temp_words = words.clone();
        let mut valid = true;

        for j in 0..word_count {
            let start = j * word_len;
            let end = start + word_len;
            let word = &substring[start..end];

            if let Some(pos) = temp_words.iter().position(|&w| w == word) {
                temp_words.remove(pos);
            } else {
                valid = false;
                break;
            }
        }

        if valid && temp_words.is_empty() {
            result.push(i as i32);
        }
    }

    result
}

fn main() {
    // Example 1
    let s1 = "barfoothefoobarman";
    let words1 = vec!["foo", "bar"];
    println!("{:?}", find_substring_brute_force(s1, words1)); // [0, 9]

    // Example 2
    let s2 = "wordgoodgoodgoodbestword";
    let words2 = vec!["word", "good", "best", "word"];
    println!("{:?}", find_substring_brute_force(s2, words2)); // []

    // Example 3
    let s3 = "barfoofoobarthefoobarman";
    let words3 = vec!["bar", "foo", "the"];
    println!("{:?}", find_substring_brute_force(s3, words3)); // [6, 9, 12]
}
