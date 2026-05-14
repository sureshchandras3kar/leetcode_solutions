impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let reversed: Vec<char> = cleaned.iter().cloned().rev().collect();
        cleaned == reversed
    }
}
