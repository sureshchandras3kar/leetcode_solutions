pub fn add_binary_string_simulation(a: String, b: String) -> String {
    /*
    String simulation approach - simulate binary addition from right to left.
    Time: O(max(a.len(), b.len()))
    Space: O(max(a.len(), b.len())) for result
    */
    let mut result = Vec::new();
    let mut carry = 0;
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = a_bytes.len() as i32 - 1;
    let mut j = b_bytes.len() as i32 - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let digit_a = if i >= 0 { (a_bytes[i as usize] - b'0') as i32 } else { 0 };
        let digit_b = if j >= 0 { (b_bytes[j as usize] - b'0') as i32 } else { 0 };

        let total = digit_a + digit_b + carry;
        result.push((b'0' + (total % 2) as u8) as char);
        carry = total / 2;

        i -= 1;
        j -= 1;
    }

    result.reverse();
    result.iter().collect()
}

fn main() {
    println!("{}", add_binary_string_simulation("11".to_string(), "1".to_string()));      // "100"
    println!("{}", add_binary_string_simulation("1010".to_string(), "1011".to_string())); // "10101"
    println!("{}", add_binary_string_simulation("0".to_string(), "0".to_string()));       // "0"
}
