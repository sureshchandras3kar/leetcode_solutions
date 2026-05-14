impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes: Vec<u8> = s.bytes().collect();
        let mut left = 0usize;
        let mut right = bytes.len().saturating_sub(1);
        while left < right {
            while left < right && !bytes[left].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
            }
            if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            if right == 0 { break; }
            right -= 1;
        }
        true
    }
}
