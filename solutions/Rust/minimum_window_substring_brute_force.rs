use std::collections::HashMap;

fn min_window_brute_force(s: String, t: String) -> String {
    if s.is_empty() || t.is_empty() || s.len() < t.len() {
        return String::new();
    }

    let mut dict_t: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *dict_t.entry(c).or_insert(0) += 1;
    }

    let mut min_len = s.len() + 1;
    let mut min_start = 0;

    let s_chars: Vec<char> = s.chars().collect();

    // Check all possible substrings
    for i in 0..s_chars.len() {
        for j in (i + 1)..=s_chars.len() {
            let substring: String = s_chars[i..j].iter().collect();

            let mut substring_count: HashMap<char, i32> = HashMap::new();
            for c in substring.chars() {
                *substring_count.entry(c).or_insert(0) += 1;
            }

            // Verify if this substring is valid
            let mut valid = true;
            for (&c, &count) in &dict_t {
                if substring_count.get(&c).copied().unwrap_or(0) < count {
                    valid = false;
                    break;
                }
            }

            // Update minimum if this is a valid and shorter substring
            if valid && substring.len() < min_len {
                min_len = substring.len();
                min_start = i;
            }
        }
    }

    if min_len == s.len() + 1 {
        String::new()
    } else {
        s_chars[min_start..(min_start + min_len)].iter().collect()
    }
}

fn main() {
    println!("{}", min_window_brute_force("ADOBECODEBANC".to_string(), "ABC".to_string()));  // "BANC"
    println!("{}", min_window_brute_force("a".to_string(), "a".to_string()));  // "a"
    println!("{}", min_window_brute_force("a".to_string(), "aa".to_string()));  // ""
}
