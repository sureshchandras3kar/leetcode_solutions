use std::collections::{HashMap, VecDeque};

pub fn letter_combinations_bfs(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut phone_map = HashMap::new();
    phone_map.insert('2', "abc");
    phone_map.insert('3', "def");
    phone_map.insert('4', "ghi");
    phone_map.insert('5', "jkl");
    phone_map.insert('6', "mno");
    phone_map.insert('7', "pqrs");
    phone_map.insert('8', "tuv");
    phone_map.insert('9', "wxyz");

    let mut queue = VecDeque::new();
    queue.push_back(String::new());

    for digit in digits.chars() {
        let size = queue.len();
        let letters = phone_map[&digit];

        // Process all current combinations
        for _ in 0..size {
            let current = queue.pop_front().unwrap();

            // Create a new combination for each letter
            for letter in letters.chars() {
                queue.push_back(format!("{}{}", current, letter));
            }
        }
    }

    queue.into_iter().collect()
}

fn main() {
    let result = letter_combinations_bfs("23".to_string());
    println!("{:?}", result);
    // Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
}
