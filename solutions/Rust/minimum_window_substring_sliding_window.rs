use std::collections::HashMap;

fn min_window_sliding_window(s: String, t: String) -> String {
    if s.is_empty() || t.is_empty() || s.len() < t.len() {
        return String::new();
    }

    // Dictionary to store frequency of characters in t
    let mut dict_t: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *dict_t.entry(c).or_insert(0) += 1;
    }

    let required = dict_t.len();
    let mut formed = 0;

    let mut window_counts: HashMap<char, i32> = HashMap::new();

    // ans: (window length, left, right)
    let mut ans_len = s.len() + 1;
    let mut ans_left = 0;
    let mut ans_right = 0;

    let mut l = 0;
    let s_chars: Vec<char> = s.chars().collect();

    for r in 0..s_chars.len() {
        let char_r = s_chars[r];
        *window_counts.entry(char_r).or_insert(0) += 1;

        if dict_t.contains_key(&char_r) && window_counts[&char_r] == dict_t[&char_r] {
            formed += 1;
        }

        while l <= r && formed == required {
            let char_l = s_chars[l];

            if r - l + 1 < ans_len {
                ans_len = r - l + 1;
                ans_left = l;
                ans_right = r;
            }

            if let Some(count) = window_counts.get_mut(&char_l) {
                *count -= 1;
            }

            if dict_t.contains_key(&char_l) && window_counts[&char_l] < dict_t[&char_l] {
                formed -= 1;
            }

            l += 1;
        }
    }

    if ans_len == s.len() + 1 {
        String::new()
    } else {
        s_chars[ans_left..=ans_right].iter().collect()
    }
}

fn main() {
    println!("{}", min_window_sliding_window("ADOBECODEBANC".to_string(), "ABC".to_string()));  // "BANC"
    println!("{}", min_window_sliding_window("a".to_string(), "a".to_string()));  // "a"
    println!("{}", min_window_sliding_window("a".to_string(), "aa".to_string()));  // ""
}
