use std::collections::HashMap;

pub fn letter_combinations_backtracking(digits: String) -> Vec<String> {
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

    let mut result = vec![];
    backtrack(&digits, 0, String::new(), &mut result, &phone_map);
    result
}

fn backtrack(
    digits: &str,
    index: usize,
    current: String,
    result: &mut Vec<String>,
    phone_map: &HashMap<char, &'static str>,
) {
    // Base case: we've processed all digits
    if index == digits.len() {
        result.push(current);
        return;
    }

    // Get the letters that the current digit maps to
    let current_digit = digits.chars().nth(index).unwrap();
    let letters = phone_map[&current_digit];

    // Try each letter
    for letter in letters.chars() {
        backtrack(
            digits,
            index + 1,
            format!("{}{}", current, letter),
            result,
            phone_map,
        );
    }
}

fn main() {
    let result = letter_combinations_backtracking("23".to_string());
    println!("{:?}", result);
    // Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
}
